package endpoints

import (
	"equation-site-backend/models"
	"equation-site-backend/utils"
	"net/http"

	"github.com/gin-gonic/gin"
	"golang.org/x/crypto/bcrypt"
)

type postUsersCreateBody struct {
	Username models.Username `json:"username" binding:"required"`
	Password models.Password `json:"password" binding:"required"`
}

func (shared *SharedContext) PostUsersCreateEndpoint(c *gin.Context) {
	body := postUsersCreateBody{}

	if err := c.ShouldBindJSON(&body); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"ok": false, "msg": "bad request"})
		return
	}

	if _, err := shared.Database.UserFromUsername(body.Username); err == nil {
		c.JSON(http.StatusBadRequest, gin.H{"ok": false, "msg": "invalid username"})
		return
	}

	hashedPassword, err := bcrypt.GenerateFromPassword([]byte(body.Password), bcrypt.DefaultCost)
	if err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"ok": false, "msg": "bad request"})
		return
	}

	id, err := utils.GenerateSecureToken(32)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"ok": false, "msg": "internal server error"})
		return
	}

	user := models.User{
		Id:          models.UserId(id),
		Username:    models.Username(body.Username),
		Password:    models.Password(hashedPassword),
		Permission:  models.UserPermission,
		DateCreated: utils.TodayAsDateCreated(),
	}

	if insertErr := shared.Database.InsertUser(user); insertErr != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"ok": false, "msg": "internal server error"})
		return
	}

	c.JSON(http.StatusOK, gin.H{"ok": true, "msg": "success"})
}
