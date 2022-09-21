package endpoints

import (
	"equation-site-backend/models"
	"net/http"
	"sort"

	"github.com/agnivade/levenshtein"
	"github.com/gin-gonic/gin"
)

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

type scoredEquation struct {
	Id          models.Id
	Title       string
	DateCreated models.DateCreated
	Score       int
}

type byScore []scoredEquation

func (a byScore) Len() int           { return len(a) }
func (a byScore) Less(i, j int) bool { return a[i].Score < a[j].Score }
func (a byScore) Swap(i, j int)      { a[i], a[j] = a[j], a[i] }

func closestMatches(eqs []models.PreviewableEquation, search string) byScore {
	eqsWithScore := make(byScore, len(eqs))
	for idx, eq := range eqs {
		eqsWithScore[idx] = scoredEquation{
			Id:          eq.Id,
			Title:       eq.Title,
			DateCreated: eq.DateCreated,
			Score:       levenshtein.ComputeDistance(eq.Title, search),
		}
	}

	sort.Sort(eqsWithScore)
	top100Eqs := eqsWithScore[:int(min(100, len(eqsWithScore)))]

	return top100Eqs
}

type resEquation struct {
	Id          models.Id          `json:"id"`
	Title       string             `json:"title"`
	DateCreated models.DateCreated `json:"date_created"`
}

func (shared *SharedContext) GetEquationsSearchEndpoint(c *gin.Context) {
	search := c.Param("title")

	eq, err := shared.Database.AllEquations()
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"ok": false, "msg": "internal server error"})
		return
	}

	ranked := closestMatches(eq, search)
	equations := make([]resEquation, len(ranked))
	for idx, eq := range ranked {
		equations[idx] = resEquation{
			Title:       eq.Title,
			Id:          eq.Id,
			DateCreated: eq.DateCreated,
		}
	}

	c.JSON(http.StatusOK, gin.H{"ok": true, "msg": "success", "equations": equations})
}
