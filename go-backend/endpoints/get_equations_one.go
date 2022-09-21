package endpoints

import (
	"equation-site-backend/models"
	"net/http"

	"github.com/gin-gonic/gin"
)

func (shared *SharedContext) GetEquationsOneEndpoint(c *gin.Context) {
	eqId := models.Id(c.Param("id"))

	eq, err := shared.Database.EquationFromId(eqId)
	if err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"ok": false, "msg": "invalid id"})
		return
	}

	c.JSON(http.StatusOK, gin.H{"ok": true, "msg": "success", "equation": map[string]any{
		"id":           string(eq.Id),
		"title":        eq.Title,
		"content":      eq.Content,
		"date_created": string(eq.DateCreated),
		"creator": map[string]any{
			"id":           string(eq.Creator.Id),
			"username":     string(eq.Creator.Username),
			"date_created": string(eq.DateCreated),
		},
	}})
}
