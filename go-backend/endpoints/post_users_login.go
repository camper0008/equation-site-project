package endpoints

import (
	"equation-site-backend/models"
	"equation-site-backend/utils"
	"net/http"

	"github.com/gin-gonic/gin"
	"golang.org/x/crypto/bcrypt"
)

type postUsersLoginBody struct {
	Username models.Username `json:"username" binding:"required"`
	Password models.Password `json:"password" binding:"required"`
}

func (shared *SharedContext) PostUsersLoginEndpoint(c *gin.Context) {
	body := postUsersLoginBody{}

	if err := c.ShouldBindJSON(&body); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"ok": false, "msg": "invalid login"})
		return
	}

	dbUser, dbErr := shared.Database.UserFromUsername(body.Username)
	if dbErr != nil {
		c.JSON(http.StatusBadRequest, gin.H{"ok": false, "msg": "invalid login"})
		return
	}

	hashedPassword := dbUser.Password
	if err := bcrypt.CompareHashAndPassword([]byte(hashedPassword), []byte(body.Password)); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"ok": false, "msg": "invalid login"})
		return
	}

	token, err := utils.GenerateSecureToken(32)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"ok": false, "msg": "internal server error"})
		return
	}

	session := models.Session(token)
	shared.Database.BindSessionToUser(session, dbUser)

	c.SetSameSite(http.SameSiteLaxMode)
	c.SetCookie("SESSION_TOKEN", string(session), 0, "/", utils.Domain(), false, true)

	c.JSON(http.StatusOK, gin.H{"ok": true, "msg": "success"})
}
