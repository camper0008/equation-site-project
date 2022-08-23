package main

import (
	"equation-site-backend/db_impl"
	"equation-site-backend/endpoints"
	"equation-site-backend/utils"
	"os"
	"time"

	"github.com/gin-contrib/cors"
	"github.com/gin-gonic/gin"

	_ "github.com/joho/godotenv/autoload"
)

func environmentVariablesSpecified() {
	if os.Getenv("DOMAIN") == "" {
		panic("domain environment variable not set")
	}
}

func main() {
	environmentVariablesSpecified()
	router := gin.Default()
	router.Use(cors.New(cors.Config{
		AllowOrigins:     []string{utils.Domain()},
		AllowMethods:     []string{"GET", "POST"},
		AllowHeaders:     []string{"Origin", "Content-Length", "Content-Type", "Cookie"},
		AllowCredentials: true,
		MaxAge:           12 * time.Hour,
	}))

	db := &db_impl.MemoryDb{}
	db.Init()

	sharedContext := endpoints.SharedContext{
		Database: db,
	}

	api := router.Group("/")
	users := api.Group("/users")
	{
		users.POST("/login", sharedContext.PostUsersLoginEndpoint)
		users.POST("/logout", sharedContext.PostUsersLogoutEndpoint)
		users.POST("/create", sharedContext.PostUsersCreateEndpoint)
		users.GET("/info", sharedContext.GetUsersInfoEndpoint)
	}
	equations := api.Group("/equations")
	{
		equations.GET("/one/:id", sharedContext.GetEquationsOneEndpoint)
		equations.GET("/search/:title", sharedContext.GetEquationsSearchEndpoint)
	}

	contributorOnlyEquations := api.Group("/equations")
	contributorOnlyEquations.Use(sharedContext.ContributorRequired)
	{
		contributorOnlyEquations.POST("/create", sharedContext.PostEquationsCreateEndpoint)
		contributorOnlyEquations.POST("/edit/:id", sharedContext.PostEquationsEditEndpoint)
		contributorOnlyEquations.POST("/remove/:id", sharedContext.PostEquationsRemoveEndpoint)
	}

	// By default it serves on :8080 unless a
	// PORT environment variable was defined.
	router.Run()
}
