package redis_manager

import (
	"context"
	"crypto/rand"
	"encoding/hex"
	"encoding/json"
	"sync"
	"time"

	"github.com/raghav1030/kazex/cmd/api/protobuf_generated_types"
	"github.com/redis/go-redis/v9"
	"google.golang.org/protobuf/proto"
)

const ()

var once sync.Once
var instance *RedisManager
var ctx context.Context = context.Background()

type RedisManager struct {
	client    *redis.Client
	publisher *redis.Client
}

func GetRedisManager() *RedisManager {

	once.Do(func() {
		instance = &RedisManager{
			client:    redis.NewClient(&redis.Options{Addr: "localhost:6379"}),
			publisher: redis.NewClient(&redis.Options{Addr: "localhost:6379"}),
		}
	})
	return instance
}

type queueMessageToOrderbook struct {
	ClientId string `json:"clientId"`
	Message  []byte `json:"message"`
}

func (m *RedisManager) SendAndAwait(message *protobuf_generated_types.MessageToEngine) (*protobuf_generated_types.MessageFromOrderBook, error) {

	clientId, _ := GenerateClientId()

	ctxWithTimeout, cancel := context.WithTimeout(ctx, 2*time.Second)
	defer cancel()

	subscriber := m.client.Subscribe(ctxWithTimeout, clientId)
	defer subscriber.Close()

	// pubsubChannel := subscriber.Channel()

	messageData, err := proto.Marshal(message)

	if err != nil {
		return &protobuf_generated_types.MessageFromOrderBook{}, err
	}

	// err := &struct {
	// 	ClientId string `json:"clientId"`
	// 	Message  []byte `json:"message"`
	// }{
	// 	ClientId: clientId,
	// 	Message:  messageData,
	// }

	messageToOrderbook := &queueMessageToOrderbook{
		ClientId: clientId,
		Message:  messageData,
	}
	
	messageToOrderbookJSON, err := json.Marshal(messageToOrderbook)
	
	if err != nil {
		return &protobuf_generated_types.MessageFromOrderBook{}, err
	}

	err = m.publisher.LPush(ctxWithTimeout, "message", messageToOrderbookJSON).Err()

	if err != nil {
		return &protobuf_generated_types.MessageFromOrderBook{}, err
	}

	return &protobuf_generated_types.MessageFromOrderBook{}, nil

	// for {
	// 	select {
	// 	case msg := <-pubsubChannel:
	// 		var messageFromOrderbook *protobuf_generated_types.MessageFromOrderBook
	// 		decodedMessage := proto.Unmarshal([]byte(msg.Payload), messageFromOrderbook)
	// 		if msg.Payload == clientId {
	// 			return &protobuf_generated_types.MessageFromOrderBook{
	// 				Type: "yoyo",
	// 			}, nil
	// 		}
	// 	case <-ctxWithTimeout.Done():
	// 		return &protobuf_generated_types.MessageFromOrderBook{}, ctxWithTimeout.Err()
	// 	}
	// }

}

func GenerateClientId() (string, error) {
	arr := make([]byte, 16)

	_, err := rand.Read(arr)

	if err != nil {
		return "", err
	}

	return hex.EncodeToString(arr), nil
}
