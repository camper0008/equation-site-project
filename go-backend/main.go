package main

import (
	"equation-site-backend/db_impl"
	"equation-site-backend/endpoints"

	"github.com/gin-gonic/gin"
)

func main() {
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
		equations.GET("/", func(c *gin.Context) {
			panic("not implemented")
		})
	}

	contributorOnlyEquations := api.Group("/equations")
	contributorOnlyEquations.Use(sharedContext.ContributorRequired)
	{
		contributorOnlyEquations.GET("/", func(c *gin.Context) {
			panic("not implemented")
		})
	}

	// By default it serves on :8080 unless a
	// PORT environment variable was defined.
	router.Run()
}
