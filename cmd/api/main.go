package main

import (
	"github.com/gofiber/fiber/v2"
	"github.com/raghav1030/kazex/cmd/api/routes"
)

func main() {
	app := fiber.New()

	api := app.Group("/api/v1")

	routes.OrderRoutes(api)
	routes.DepthRoutes(api)

	

	app.Listen(":3000")

}
