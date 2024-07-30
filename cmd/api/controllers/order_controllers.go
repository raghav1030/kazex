package controllers

import (
	"github.com/gofiber/fiber/v2"
	config "github.com/raghav1030/kazex/cmd/api/config/protojson_config"
	"github.com/raghav1030/kazex/cmd/api/protobuf_generated_types"
	redis_manager "github.com/raghav1030/kazex/cmd/api/redis"
	customTypes "github.com/raghav1030/kazex/cmd/api/types"
	"github.com/raghav1030/kazex/cmd/api/utils"
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

func CRUDOrders(c *fiber.Ctx) error {

	order := &protobuf_generated_types.MessageToEngine{}

	bodyBytes := c.Body()

	// err := protojson.Unmarshal(bodyBytes, order)
	err := config.ProtojsonUnmarshallingOptions.Unmarshal(bodyBytes, order)

	if err != nil {
		return c.Status(400).JSON(fiber.Map{
			"error": err.Error(),
		})
	}
	err = utils.CheckMessageToEnginePayloadType(order)
	if err != nil {
		return c.Status(400).JSON(fiber.Map{
			"error": "invalid payload type",
		})
	}

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

func GetOrdersByQuery(c *fiber.Ctx) error {
	userId := c.Query("userId")
	market := c.Query("market")

	message := &protobuf_generated_types.MessageToEngine{
		Type: protobuf_generated_types.MessageToEngineType_GET_OPEN_ORDERS,
		Payload: &protobuf_generated_types.MessageToEngine_GetOpenOrdersPayload{
			GetOpenOrdersPayload: &protobuf_generated_types.GetOpenOrdersPayload{
				UserId: userId,
				Market: market,
			},
		},
	}
	feedbackMessage, err := redisManager.SendAndAwait(message)

	if err != nil {
		return c.Status(500).JSON(fiber.Map{
			"error": err.Error(),
		})
	}

	return c.Status(200).JSON(fiber.Map{
		"message":         "orders fetched",
		"feedbackMessage": feedbackMessage,
	})
}
