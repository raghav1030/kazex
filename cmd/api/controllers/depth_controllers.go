package controllers

import (
	"fmt"

	"github.com/gofiber/fiber/v2"
	config "github.com/raghav1030/kazex/cmd/api/config/protojson_config"
	"github.com/raghav1030/kazex/cmd/api/protobuf_generated_types"
	"github.com/raghav1030/kazex/cmd/api/utils"
)

func GetDepth(c *fiber.Ctx) error {

	marketQuery := c.Query("market")
	fmt.Println(marketQuery)
	if marketQuery != "" {
		fmt.Println("marketQuery")
		message := &protobuf_generated_types.MessageToEngine{
			Type: protobuf_generated_types.MessageToEngineType_GET_DEPTH,
			Payload: &protobuf_generated_types.MessageToEngine_GetDepthPayload{
				GetDepthPayload: &protobuf_generated_types.GetDepthPayload{
					Market: marketQuery,
				},
			},
		}

		feedbackMessage, err := redisManager.SendAndAwait(message)

		if err != nil {
			return c.Status(500).JSON(fiber.Map{
				"error": err.Error(),
			})
		}

		return c.Status(200).JSON(feedbackMessage)
	}

	depth := protobuf_generated_types.MessageToEngine{}

	bodyBytes := c.Body()

	// err := protojson.Unmarshal(bodyBytes, depth)
	err := config.ProtojsonUnmarshallingOptions.Unmarshal(bodyBytes, &depth)

	if err != nil {
		return c.Status(400).JSON(fiber.Map{
			"error": err.Error(),
		})
	}

	if depth.Type != protobuf_generated_types.MessageToEngineType_GET_DEPTH {
		return c.Status(400).JSON(fiber.Map{
			"error": "invalid payload type",
		})
	}
	err = utils.CheckMessageToEnginePayloadType(&depth)
	if err != nil {
		return c.Status(400).JSON(fiber.Map{
			"error": "invalid payload type",
		})
	}

	feedbackMessage, err := redisManager.SendAndAwait(&depth)

	if err != nil {
		return c.Status(500).JSON(fiber.Map{
			"error": err.Error(),
		})
	}

	return c.Status(200).JSON(feedbackMessage)
}
