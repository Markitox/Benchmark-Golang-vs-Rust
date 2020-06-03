package main

import (
	"fmt"
	"gin163/controller"

	"github.com/gin-gonic/gin"
)

func main() {
	fmt.Println("Starting Gin app!")

	router := configureRouters()

	router.Run()
}

func configureRouters() *gin.Engine {

	ginRouter := gin.Default()

	ginRouter.GET("/helloWorld", controller.HelloWorld)

	return ginRouter
}
