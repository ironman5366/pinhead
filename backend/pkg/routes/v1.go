package routes

import (
	"github.com/gin-gonic/gin"
)

func V1Routes(r *gin.RouterGroup) {
	r.GET("/documents/:id", func(c *gin.Context) {

	})
	r.POST("/documents/:id")
}
