package config

import "google.golang.org/protobuf/encoding/protojson"

var ProtojsonMarshallingOptions = protojson.MarshalOptions{
	EmitUnpopulated: false, // Do not emit unpopulated fields
	// UseProtoNames:   true,  // Use proto field names
	AllowPartial:   false,  // Allow partial marshalling
	// UseEnumNumbers: false, // Use enum numbers instead of names
}

var ProtojsonUnmarshallingOptions = protojson.UnmarshalOptions{
	// DiscardUnknown: true, // Ignore fields in the input that do not map to exported fields in the message type.
	AllowPartial: false, // Allow partial unmarshalling

}
