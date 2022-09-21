package endpoints

import (
	"equation-site-backend/models"
	"equation-site-backend/utils"
	"net/http"

	"github.com/gin-gonic/gin"
)

type postEquationsCreateBody struct {
	Title   string `json:"title" binding:"required"`
	Content string `json:"content" binding:"required"`
}

func (shared *SharedContext) PostEquationsCreateEndpoint(c *gin.Context) {
	body := postEquationsCreateBody{}

	if err := c.ShouldBindJSON(&body); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"ok": false, "msg": "bad request"})
		return
	}

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

	id, err := utils.GenerateSecureToken(8)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"ok": false, "msg": "internal server error"})
		return
	}

	equation := models.Equation{
		Id:          models.Id(id),
		Title:       body.Title,
		Content:     body.Content,
		Creator:     user,
		DateCreated: utils.TodayAsDateCreated(),
	}

	if insertErr := shared.Database.InsertEquation(equation); insertErr != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"ok": false, "msg": "internal server error"})
		return
	}

	c.JSON(http.StatusOK, gin.H{"ok": true, "msg": "success"})
}
