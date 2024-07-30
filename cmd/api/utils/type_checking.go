package utils

import (
	"fmt"

	config "github.com/raghav1030/kazex/cmd/api/config/protojson_config"
	"github.com/raghav1030/kazex/cmd/api/protobuf_generated_types"
)

func CheckMessageToEnginePayloadType(body *protobuf_generated_types.MessageToEngine) error {
	messageType := body.Type

	switch messageType {
	case protobuf_generated_types.MessageToEngineType_CREATE_ORDER:

		payload := body.GetCreateOrderPayload()

		if payload == nil {
			return fmt.Errorf("invalid payload key")
		}

		// marshalledPayload, err := protojson.Marshal(payload)
		marshalledPayload, err := config.ProtojsonMarshallingOptions.Marshal(payload)
		if err != nil {
			return err
		}

		var createOrderPayload protobuf_generated_types.CreateOrderPayload

		err = config.ProtojsonUnmarshallingOptions.Unmarshal(marshalledPayload, &createOrderPayload)

		if err != nil {
			return err
		}

		return nil

	case protobuf_generated_types.MessageToEngineType_GET_DEPTH:
		payload := body.GetGetDepthPayload()

		if payload == nil {
			return fmt.Errorf("invalid payload key")
		}

		// marshalledPayload, err := protojson.Marshal(payload)
		marshalledPayload, err := config.ProtojsonMarshallingOptions.Marshal(payload)

		if err != nil {
			return err
		}

		var getDepthPayload protobuf_generated_types.GetDepthPayload

		err = config.ProtojsonUnmarshallingOptions.Unmarshal(marshalledPayload, &getDepthPayload)

		if err != nil {
			return err
		}

		return nil

	case protobuf_generated_types.MessageToEngineType_CANCEL_ORDER:
		payload := body.GetCancelOrderPayload()

		if payload == nil {
			return fmt.Errorf("invalid payload key")
		}

		// marshalledPayload, err := protojson.Marshal(payload)
		marshalledPayload, err := config.ProtojsonMarshallingOptions.Marshal(payload)

		if err != nil {
			return err
		}

		var cancelOrderPayload protobuf_generated_types.CancelOrderPayload

		err = config.ProtojsonUnmarshallingOptions.Unmarshal(marshalledPayload, &cancelOrderPayload)

		if err != nil {
			return err
		}

		return nil

	case protobuf_generated_types.MessageToEngineType_GET_OPEN_ORDERS:
		payload := body.GetGetOpenOrdersPayload()
		if payload == nil {
			return fmt.Errorf("invalid payload key")
		}
		// marshalledPayload, err := protojson.Marshal(payload)
		marshalledPayload, err := config.ProtojsonMarshallingOptions.Marshal(payload)

		if err != nil {
			return err
		}

		var getOpenOrdersPayload protobuf_generated_types.GetOpenOrdersPayload

		err = config.ProtojsonUnmarshallingOptions.Unmarshal(marshalledPayload, &getOpenOrdersPayload)

		if err != nil {
			return err
		}

		return nil

	case protobuf_generated_types.MessageToEngineType_ON_RAMP:

		payload := body.GetOnRampPayload()

		if payload == nil {
			return fmt.Errorf("invalid payload key")
		}

		// marshalledPayload, err := protojson.Marshal(payload)
		marshalledPayload, err := config.ProtojsonMarshallingOptions.Marshal(payload)

		if err != nil {
			return err
		}

		var onRampPayload protobuf_generated_types.OnRampPayload

		err = config.ProtojsonUnmarshallingOptions.Unmarshal(marshalledPayload, &onRampPayload)

		if err != nil {
			return err
		}

		return nil

	default:
		return fmt.Errorf("invalid type")

	}

}
