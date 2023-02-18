package main

import (
	"github.com/gin-gonic/gin"
	"github.com/ironman5366/pinhead/backend/pkg/routes"
)

func main() {
	r := gin.Default()
	apiGroup := r.Group("api")
	routes.V1Routes(apiGroup.Group("v1"))
	r.Run()
}
