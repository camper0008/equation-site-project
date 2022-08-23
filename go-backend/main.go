package main

import (
	"equation-site-backend/db_impl"
	"equation-site-backend/endpoints"
	"os"

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

	db := &db_impl.MemoryDb{}
	db.Init()

	sharedContext := endpoints.SharedContext{
		Database: db,
	}

	api := router.Group("/api")
	users := api.Group("/users")
	{
		users.POST("/login", sharedContext.PostUsersLoginEndpoint)
	}
	equations := api.Group("/equations")
	{
		equations.GET("/ping", func(c *gin.Context) {
			panic("not implemented")
		})
	}

	contributorOnlyEquations := api.Group("/equations")
	contributorOnlyEquations.Use(sharedContext.ContributorRequired)
	{
		contributorOnlyEquations.GET("/test", func(c *gin.Context) {
			panic("not implemented")
		})
	}

	// By default it serves on :8080 unless a
	// PORT environment variable was defined.
	router.Run()
}
