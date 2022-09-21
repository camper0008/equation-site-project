package endpoints

import (
	"equation-site-backend/models"
	"net/http"

	"github.com/gin-gonic/gin"
)

func (shared *SharedContext) PostEquationsRemoveEndpoint(c *gin.Context) {
	eqId := models.Id(c.Param("id"))

	eq := models.Equation{
		Id: eqId,
	}

	if deleteErr := shared.Database.RemoveEquation(eq); deleteErr != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"ok": false, "msg": "internal server error"})
		return
	}

	c.JSON(http.StatusOK, gin.H{"ok": true, "msg": "success"})
}
