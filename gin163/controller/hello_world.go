package controller

import (
	"fmt"

	"net/http"
	"sync/atomic"

	"github.com/gin-gonic/gin"
)

var counter uint64 = 0

func HelloWorld(ginContext *gin.Context) {
	atomic.AddUint64(&counter, 1)

	response := fmt.Sprintf("Request # %d", counter)

	ginContext.String(http.StatusOK, response)
}
