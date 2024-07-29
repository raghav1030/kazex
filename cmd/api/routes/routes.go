package routes

import (
	"github.com/gofiber/fiber/v2"
	"github.com/raghav1030/kazex/cmd/api/controllers"
)

func OrderRoutes(api fiber.Router) {
	order_router := api.Group("/orders")

	order_router.Post("/", controllers.CreateOrder)

}
