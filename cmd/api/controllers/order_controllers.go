package controllers

import (
	"fmt"

	"github.com/gofiber/fiber/v2"
	"github.com/raghav1030/kazex/cmd/api/protobuf_generated_types"
	redis_manager "github.com/raghav1030/kazex/cmd/api/redis"
	customTypes "github.com/raghav1030/kazex/cmd/api/types"
	"google.golang.org/protobuf/encoding/protojson"
)

type CreateOrderRequest struct {
	BaseAsset  string                `json:"baseAsset"`
	QuoteAsset string                `json:"quoteAsset"`
	Price      string                `json:"price"`
	Quantity   int                   `json:"quantity"`
	Side       customTypes.Side      `json:"side"`
	Type       customTypes.OrderType `json:"type"`
	Kind       *customTypes.Kind     `json:"kind,omitempty"`
}

var redisManager *redis_manager.RedisManager = redis_manager.GetRedisManager()

func CreateOrder(c *fiber.Ctx) error {

	order := &protobuf_generated_types.MessageToEngine{}

	bodyBytes := c.Body()

	err := protojson.Unmarshal(bodyBytes, order)

	if err != nil {
		return c.Status(400).JSON(fiber.Map{
			"error": err.Error(),
		})
	}
	fmt.Println(order)

	feedbackMessage, err := redisManager.SendAndAwait(order)

	if err != nil {
		return c.Status(500).JSON(fiber.Map{
			"error": err.Error(),
		})
	}

	return c.Status(200).JSON(fiber.Map{
		"message":         "order created",
		"feedbackMessage": feedbackMessage,
	})

}
