package main

import (
	"github.com/gin-gonic/gin"
	"github.com/ironman5366/pinhead/backend/pkg/routes"
	"github.com/ironman5366/pinhead/backend/pkg/util"
	"github.com/joho/godotenv"
)

func main() {
	err := godotenv.Load()
	if err != nil {
		panic(err)
	}
	// After we call this, util.DB is safe to access for the rest of the project
	err = util.InitializeDatabase()
	if err != nil {
		panic(err)
	}

	r := gin.Default()
	apiGroup := r.Group("api")
	routes.V1Routes(apiGroup.Group("v1"))
	r.Run()
}
