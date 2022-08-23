package endpoints

import (
	"equation-site-backend/models"
	"equation-site-backend/utils"
	"net/http"

	"github.com/gin-gonic/gin"
)

func (shared *SharedContext) PostUsersLogoutEndpoint(c *gin.Context) {
	sessionCookie, err := c.Cookie("SESSION_TOKEN")
	if err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"ok": false, "msg": "invalid cookie"})
		return
	}
	session := models.Session(sessionCookie)
	if dbErr := shared.Database.UnbindSession(session); dbErr != nil {
		c.JSON(http.StatusBadRequest, gin.H{"ok": false, "msg": "invalid cookie"})
		return
	}
	c.SetCookie("SESSION_TOKEN", "", -1, "/", utils.Domain(), false, true)

	c.JSON(http.StatusOK, gin.H{"ok": true, "msg": "success"})
}
