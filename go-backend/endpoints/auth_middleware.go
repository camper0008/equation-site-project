package endpoints

import (
	"equation-site-backend/models"
	"net/http"

	"github.com/gin-gonic/gin"
)

func (shared *SharedContext) ContributorRequired(c *gin.Context) {
	sessionCookie, err := c.Cookie("SESSION_TOKEN")
	if err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"ok": false, "msg": "invalid cookie"})
		return
	}
	session := models.Session(sessionCookie)
	user, err := shared.Database.UserFromSession(session)
	if err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"ok": false, "msg": "invalid cookie"})
		return
	}

	if user.Permission < models.ContributorPermission {
		c.JSON(http.StatusBadRequest, gin.H{"ok": false, "msg": "unauthorized"})
		return
	}

	c.Next()
}
