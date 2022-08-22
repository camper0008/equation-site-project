package endpoints

import (
	"equation-site-backend/models"
	"equation-site-backend/utils"
	"fmt"
	"net/http"

	"github.com/gin-gonic/gin"
	"golang.org/x/crypto/bcrypt"
)

type requestBody struct {
	Username models.Username `json:"username" binding:"required"`
	Password models.Password `json:"password" binding:"required"`
}

func (shared *SharedContext) PostUsersLoginEndpoint(c *gin.Context) {
	body := requestBody{}

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

	fmt.Printf("remember to set to domain in production")
	c.SetCookie("SESSION_TOKEN", string(session), 0, "/", "127.0.0.1", false, true)

	c.JSON(http.StatusOK, gin.H{"ok": true, "msg": "success"})
}
