package main

import (
	"embed"
	"net/http"

	"github.com/labstack/echo/v4"
	"github.com/labstack/echo/v4/middleware"
)

// TODO: core/cmd/outなら動作するもののディレクトリ構造を見直す
//
//go:embed all:out
var webAssets embed.FS

func main() {
	e := echo.New()

	e.Use(middleware.Recover())
	e.Use(middleware.Logger())

	e.Use(middleware.StaticWithConfig(middleware.StaticConfig{
		HTML5:      true,
		Root:       "out",
		Browse:     false,
		Filesystem: http.FS(webAssets),
	}))

	e.Logger.Fatal(e.Start(":1323"))
}
