package customTypes

//enums

type Side string

const(
	BUY Side = "buy"
	SELL Side = "sell"
)

type OrderType string

const (
	MARKET OrderType = "market"
	LIMIT OrderType = "limit"
)

type Kind string 

const (
	IOC Kind = "kind"
)


