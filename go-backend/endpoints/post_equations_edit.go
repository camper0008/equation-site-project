package endpoints

import (
	"equation-site-backend/models"
	"net/http"

	"github.com/gin-gonic/gin"
)

type postEquationsEditBody struct {
	Title   string `json:"title" binding:"required"`
	Content string `json:"content" binding:"required"`
}

func (shared *SharedContext) PostEquationsEditEndpoint(c *gin.Context) {
	body := postEquationsEditBody{}

	if err := c.ShouldBindJSON(&body); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"ok": false, "msg": "bad request"})
		return
	}

	eqId := models.Id(c.Param("id"))

	oldEq, err := shared.Database.EquationFromId(eqId)
	if err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"ok": false, "msg": "invalid id"})
		return
	}

	equation := models.Equation{
		Id:          oldEq.Id,
		Title:       body.Title,
		Content:     body.Content,
		Creator:     oldEq.Creator,
		DateCreated: oldEq.DateCreated,
	}

	if updateErr := shared.Database.UpdateEquation(equation); updateErr != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"ok": false, "msg": "internal server error"})
		return
	}

	c.JSON(http.StatusOK, gin.H{"ok": true, "msg": "success"})
}
