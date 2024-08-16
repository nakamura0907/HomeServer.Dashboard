package main

import (
	"embed"
	"net/http"

	"github.com/labstack/echo/v4"
	"github.com/labstack/echo/v4/middleware"
)

//go:embed all:web/out
var webAssets embed.FS

func main() {
	e := echo.New()

	e.Use(middleware.Recover())
	e.Use(middleware.Logger())

	e.Use(middleware.StaticWithConfig(middleware.StaticConfig{
		HTML5:      true,
		Root:       "web/out",
		Browse:     false,
		Filesystem: http.FS(webAssets),
	}))

	apiGroup := e.Group("/api")
	apiGroup.GET("", func(c echo.Context) error {
		response := map[string]interface{}{
			"items": map[string]string{
				"title": "Example",
				"url":   "https://example.com",
			},
		}

		return c.JSON(http.StatusOK, response)
	})

	e.Logger.Fatal(e.Start(":1323"))
}
