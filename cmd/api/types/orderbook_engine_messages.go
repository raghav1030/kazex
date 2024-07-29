package customTypes

const (
	CREATE_ORDER    = "CREATE_ORDER"
	CANCEL_ORDER    = "CANCEL_ORDER"
	ON_RAMP         = "ON_RAMP"
	GET_OPEN_ORDERS = "GET_OPEN_ORDERS"
	GET_DEPTH       = "GET_DEPTH"
)

type DepthPayload struct {
	Market string      `json:"market"`
	Bids   [][2]string `json:"bids"`
	Asks   [][2]string `json:"asks"`
}

type OpenOrdersPayload struct {
	OrderId     string `json:"orderId"`
	Price       string `json:"price"`
	Quantity    int    `json:"quantity"`
	ExecutedQty int    `json:"executedQty"`
	Side        Side   `json:"side"`
	UserId      string `json:"userId"`
}

type OrderCancelledPayload struct {
	OrderId      string `json:"orderId"`
	ExecutedQty  string `json:"executedQty"`
	RemainingQty int    `json:"remainingQty"`
}

type OrderPlacedPayload struct {
	OrderId     string  `json:"orderId"`
	ExecutedQty int     `json:"executedQty"`
	Fills       []Fills `json:"fills"`
}

type Fills struct {
	Price    string `json:"price"`
	Quantity int    `json:"quantity"`
	TradeId  int    `json:"tradeId"`
}

const (
	DEPTH           string = "DEPTH"
	OPEN_ORDERS     string = "OPEN_ORDERS"
	ORDER_COMPLETED string = "ORDER_COMPLETED"
	ORDER_CANCELLED string = "ORDER_CANCELLED"
)

// type OrderPayload interface {
// 	OrderCancelledPayload | DepthPayload | OpenOrdersPayload |  OrderPlacedPayload
// }

type OrderPayload interface {
	GetType() string
}

type MessageFromOrderbook struct {
	Type    string       `json:"type"`
	Payload OrderPayload `json:"payload"`
}
