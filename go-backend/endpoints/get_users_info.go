package endpoints

import (
	"equation-site-backend/models"
	"net/http"

	"github.com/gin-gonic/gin"
)

func (shared *SharedContext) GetUsersInfoEndpoint(c *gin.Context) {
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

	c.JSON(http.StatusOK, gin.H{"ok": true, "msg": "success", "user": map[string]any{
		"id":           user.Id,
		"username":     user.Username,
		"permission":   user.Permission.String(),
		"date_created": user.DateCreated,
	}})
}
