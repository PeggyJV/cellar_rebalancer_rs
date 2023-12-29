//
// Steward Strategy Provider API
//
// This proto defines the service/methods used by Strategy Providers to interact with Cellars through the Sommelier chain.

// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.26.0
// 	protoc        v4.25.1
// source: steward.proto

package steward_proto

import (
	protoreflect "google.golang.org/protobuf/reflect/protoreflect"
	protoimpl "google.golang.org/protobuf/runtime/protoimpl"
	reflect "reflect"
	sync "sync"
)

const (
	// Verify that this generated code is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(20 - protoimpl.MinVersion)
	// Verify that runtime/protoimpl is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(protoimpl.MaxVersion - 20)
)

//
// Represents a single function call on a particular Cellar
type SubmitRequest struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// The ID (currently simply an Ethereum address) of the target Cellar
	CellarId string `protobuf:"bytes,1,opt,name=cellar_id,json=cellarId,proto3" json:"cellar_id,omitempty"`
	// The data from which the desired contract function will be encoded
	//
	// Types that are assignable to CallData:
	//	*SubmitRequest_AaveV2Stablecoin
	//	*SubmitRequest_CellarV1
	//	*SubmitRequest_CellarV2
	//	*SubmitRequest_CellarV2Dot2
	//	*SubmitRequest_CellarV2Dot5
	CallData isSubmitRequest_CallData `protobuf_oneof:"call_data"`
}

func (x *SubmitRequest) Reset() {
	*x = SubmitRequest{}
	if protoimpl.UnsafeEnabled {
		mi := &file_steward_proto_msgTypes[0]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *SubmitRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*SubmitRequest) ProtoMessage() {}

func (x *SubmitRequest) ProtoReflect() protoreflect.Message {
	mi := &file_steward_proto_msgTypes[0]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use SubmitRequest.ProtoReflect.Descriptor instead.
func (*SubmitRequest) Descriptor() ([]byte, []int) {
	return file_steward_proto_rawDescGZIP(), []int{0}
}

func (x *SubmitRequest) GetCellarId() string {
	if x != nil {
		return x.CellarId
	}
	return ""
}

func (m *SubmitRequest) GetCallData() isSubmitRequest_CallData {
	if m != nil {
		return m.CallData
	}
	return nil
}

func (x *SubmitRequest) GetAaveV2Stablecoin() *AaveV2Stablecoin {
	if x, ok := x.GetCallData().(*SubmitRequest_AaveV2Stablecoin); ok {
		return x.AaveV2Stablecoin
	}
	return nil
}

func (x *SubmitRequest) GetCellarV1() *CellarV1 {
	if x, ok := x.GetCallData().(*SubmitRequest_CellarV1); ok {
		return x.CellarV1
	}
	return nil
}

func (x *SubmitRequest) GetCellarV2() *CellarV2 {
	if x, ok := x.GetCallData().(*SubmitRequest_CellarV2); ok {
		return x.CellarV2
	}
	return nil
}

func (x *SubmitRequest) GetCellarV2Dot2() *CellarV2_2 {
	if x, ok := x.GetCallData().(*SubmitRequest_CellarV2Dot2); ok {
		return x.CellarV2Dot2
	}
	return nil
}

func (x *SubmitRequest) GetCellarV2Dot5() *CellarV2_5 {
	if x, ok := x.GetCallData().(*SubmitRequest_CellarV2Dot5); ok {
		return x.CellarV2Dot5
	}
	return nil
}

type isSubmitRequest_CallData interface {
	isSubmitRequest_CallData()
}

type SubmitRequest_AaveV2Stablecoin struct {
	AaveV2Stablecoin *AaveV2Stablecoin `protobuf:"bytes,2,opt,name=aave_v2_stablecoin,json=aaveV2Stablecoin,proto3,oneof"`
}

type SubmitRequest_CellarV1 struct {
	CellarV1 *CellarV1 `protobuf:"bytes,3,opt,name=cellar_v1,json=cellarV1,proto3,oneof"`
}

type SubmitRequest_CellarV2 struct {
	CellarV2 *CellarV2 `protobuf:"bytes,4,opt,name=cellar_v2,json=cellarV2,proto3,oneof"`
}

type SubmitRequest_CellarV2Dot2 struct {
	CellarV2Dot2 *CellarV2_2 `protobuf:"bytes,5,opt,name=cellar_v2dot2,json=cellarV2dot2,proto3,oneof"`
}

type SubmitRequest_CellarV2Dot5 struct {
	CellarV2Dot5 *CellarV2_5 `protobuf:"bytes,6,opt,name=cellar_v2dot5,json=cellarV2dot5,proto3,oneof"`
}

func (*SubmitRequest_AaveV2Stablecoin) isSubmitRequest_CallData() {}

func (*SubmitRequest_CellarV1) isSubmitRequest_CallData() {}

func (*SubmitRequest_CellarV2) isSubmitRequest_CallData() {}

func (*SubmitRequest_CellarV2Dot2) isSubmitRequest_CallData() {}

func (*SubmitRequest_CellarV2Dot5) isSubmitRequest_CallData() {}

type SubmitResponse struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields
}

func (x *SubmitResponse) Reset() {
	*x = SubmitResponse{}
	if protoimpl.UnsafeEnabled {
		mi := &file_steward_proto_msgTypes[1]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *SubmitResponse) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*SubmitResponse) ProtoMessage() {}

func (x *SubmitResponse) ProtoReflect() protoreflect.Message {
	mi := &file_steward_proto_msgTypes[1]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use SubmitResponse.ProtoReflect.Descriptor instead.
func (*SubmitResponse) Descriptor() ([]byte, []int) {
	return file_steward_proto_rawDescGZIP(), []int{1}
}

type VersionRequest struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields
}

func (x *VersionRequest) Reset() {
	*x = VersionRequest{}
	if protoimpl.UnsafeEnabled {
		mi := &file_steward_proto_msgTypes[2]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *VersionRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*VersionRequest) ProtoMessage() {}

func (x *VersionRequest) ProtoReflect() protoreflect.Message {
	mi := &file_steward_proto_msgTypes[2]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use VersionRequest.ProtoReflect.Descriptor instead.
func (*VersionRequest) Descriptor() ([]byte, []int) {
	return file_steward_proto_rawDescGZIP(), []int{2}
}

type VersionResponse struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Version string `protobuf:"bytes,1,opt,name=version,proto3" json:"version,omitempty"`
}

func (x *VersionResponse) Reset() {
	*x = VersionResponse{}
	if protoimpl.UnsafeEnabled {
		mi := &file_steward_proto_msgTypes[3]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *VersionResponse) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*VersionResponse) ProtoMessage() {}

func (x *VersionResponse) ProtoReflect() protoreflect.Message {
	mi := &file_steward_proto_msgTypes[3]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use VersionResponse.ProtoReflect.Descriptor instead.
func (*VersionResponse) Descriptor() ([]byte, []int) {
	return file_steward_proto_rawDescGZIP(), []int{3}
}

func (x *VersionResponse) GetVersion() string {
	if x != nil {
		return x.Version
	}
	return ""
}

var File_steward_proto protoreflect.FileDescriptor

var file_steward_proto_rawDesc = []byte{
	0x0a, 0x0d, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12,
	0x0a, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e, 0x76, 0x33, 0x1a, 0x18, 0x61, 0x61, 0x76,
	0x65, 0x5f, 0x76, 0x32, 0x5f, 0x73, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x63, 0x6f, 0x69, 0x6e, 0x2e,
	0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x0f, 0x63, 0x65, 0x6c, 0x6c, 0x61, 0x72, 0x5f, 0x76, 0x31,
	0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x0f, 0x63, 0x65, 0x6c, 0x6c, 0x61, 0x72, 0x5f, 0x76,
	0x32, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xef, 0x02, 0x0a, 0x0d, 0x53, 0x75, 0x62, 0x6d,
	0x69, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x1b, 0x0a, 0x09, 0x63, 0x65, 0x6c,
	0x6c, 0x61, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x63, 0x65,
	0x6c, 0x6c, 0x61, 0x72, 0x49, 0x64, 0x12, 0x4c, 0x0a, 0x12, 0x61, 0x61, 0x76, 0x65, 0x5f, 0x76,
	0x32, 0x5f, 0x73, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x63, 0x6f, 0x69, 0x6e, 0x18, 0x02, 0x20, 0x01,
	0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e, 0x76, 0x33, 0x2e,
	0x41, 0x61, 0x76, 0x65, 0x56, 0x32, 0x53, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x63, 0x6f, 0x69, 0x6e,
	0x48, 0x00, 0x52, 0x10, 0x61, 0x61, 0x76, 0x65, 0x56, 0x32, 0x53, 0x74, 0x61, 0x62, 0x6c, 0x65,
	0x63, 0x6f, 0x69, 0x6e, 0x12, 0x33, 0x0a, 0x09, 0x63, 0x65, 0x6c, 0x6c, 0x61, 0x72, 0x5f, 0x76,
	0x31, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x14, 0x2e, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72,
	0x64, 0x2e, 0x76, 0x33, 0x2e, 0x43, 0x65, 0x6c, 0x6c, 0x61, 0x72, 0x56, 0x31, 0x48, 0x00, 0x52,
	0x08, 0x63, 0x65, 0x6c, 0x6c, 0x61, 0x72, 0x56, 0x31, 0x12, 0x33, 0x0a, 0x09, 0x63, 0x65, 0x6c,
	0x6c, 0x61, 0x72, 0x5f, 0x76, 0x32, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x14, 0x2e, 0x73,
	0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e, 0x76, 0x33, 0x2e, 0x43, 0x65, 0x6c, 0x6c, 0x61, 0x72,
	0x56, 0x32, 0x48, 0x00, 0x52, 0x08, 0x63, 0x65, 0x6c, 0x6c, 0x61, 0x72, 0x56, 0x32, 0x12, 0x3d,
	0x0a, 0x0d, 0x63, 0x65, 0x6c, 0x6c, 0x61, 0x72, 0x5f, 0x76, 0x32, 0x64, 0x6f, 0x74, 0x32, 0x18,
	0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x16, 0x2e, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e,
	0x76, 0x33, 0x2e, 0x43, 0x65, 0x6c, 0x6c, 0x61, 0x72, 0x56, 0x32, 0x5f, 0x32, 0x48, 0x00, 0x52,
	0x0c, 0x63, 0x65, 0x6c, 0x6c, 0x61, 0x72, 0x56, 0x32, 0x64, 0x6f, 0x74, 0x32, 0x12, 0x3d, 0x0a,
	0x0d, 0x63, 0x65, 0x6c, 0x6c, 0x61, 0x72, 0x5f, 0x76, 0x32, 0x64, 0x6f, 0x74, 0x35, 0x18, 0x06,
	0x20, 0x01, 0x28, 0x0b, 0x32, 0x16, 0x2e, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e, 0x76,
	0x33, 0x2e, 0x43, 0x65, 0x6c, 0x6c, 0x61, 0x72, 0x56, 0x32, 0x5f, 0x35, 0x48, 0x00, 0x52, 0x0c,
	0x63, 0x65, 0x6c, 0x6c, 0x61, 0x72, 0x56, 0x32, 0x64, 0x6f, 0x74, 0x35, 0x42, 0x0b, 0x0a, 0x09,
	0x63, 0x61, 0x6c, 0x6c, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x22, 0x10, 0x0a, 0x0e, 0x53, 0x75, 0x62,
	0x6d, 0x69, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x10, 0x0a, 0x0e, 0x56,
	0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x22, 0x2b, 0x0a,
	0x0f, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
	0x12, 0x18, 0x0a, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28,
	0x09, 0x52, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x32, 0x51, 0x0a, 0x0c, 0x43, 0x6f,
	0x6e, 0x74, 0x72, 0x61, 0x63, 0x74, 0x43, 0x61, 0x6c, 0x6c, 0x12, 0x41, 0x0a, 0x06, 0x53, 0x75,
	0x62, 0x6d, 0x69, 0x74, 0x12, 0x19, 0x2e, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e, 0x76,
	0x33, 0x2e, 0x53, 0x75, 0x62, 0x6d, 0x69, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a,
	0x1a, 0x2e, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e, 0x76, 0x33, 0x2e, 0x53, 0x75, 0x62,
	0x6d, 0x69, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x00, 0x32, 0x4e, 0x0a,
	0x06, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x12, 0x44, 0x0a, 0x07, 0x56, 0x65, 0x72, 0x73, 0x69,
	0x6f, 0x6e, 0x12, 0x1a, 0x2e, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e, 0x76, 0x33, 0x2e,
	0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x1b,
	0x2e, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e, 0x76, 0x33, 0x2e, 0x56, 0x65, 0x72, 0x73,
	0x69, 0x6f, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x00, 0x42, 0x10, 0x5a,
	0x0e, 0x2f, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x5f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62,
	0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
}

var (
	file_steward_proto_rawDescOnce sync.Once
	file_steward_proto_rawDescData = file_steward_proto_rawDesc
)

func file_steward_proto_rawDescGZIP() []byte {
	file_steward_proto_rawDescOnce.Do(func() {
		file_steward_proto_rawDescData = protoimpl.X.CompressGZIP(file_steward_proto_rawDescData)
	})
	return file_steward_proto_rawDescData
}

var file_steward_proto_msgTypes = make([]protoimpl.MessageInfo, 4)
var file_steward_proto_goTypes = []interface{}{
	(*SubmitRequest)(nil),    // 0: steward.v3.SubmitRequest
	(*SubmitResponse)(nil),   // 1: steward.v3.SubmitResponse
	(*VersionRequest)(nil),   // 2: steward.v3.VersionRequest
	(*VersionResponse)(nil),  // 3: steward.v3.VersionResponse
	(*AaveV2Stablecoin)(nil), // 4: steward.v3.AaveV2Stablecoin
	(*CellarV1)(nil),         // 5: steward.v3.CellarV1
	(*CellarV2)(nil),         // 6: steward.v3.CellarV2
	(*CellarV2_2)(nil),       // 7: steward.v3.CellarV2_2
	(*CellarV2_5)(nil),       // 8: steward.v3.CellarV2_5
}
var file_steward_proto_depIdxs = []int32{
	4, // 0: steward.v3.SubmitRequest.aave_v2_stablecoin:type_name -> steward.v3.AaveV2Stablecoin
	5, // 1: steward.v3.SubmitRequest.cellar_v1:type_name -> steward.v3.CellarV1
	6, // 2: steward.v3.SubmitRequest.cellar_v2:type_name -> steward.v3.CellarV2
	7, // 3: steward.v3.SubmitRequest.cellar_v2dot2:type_name -> steward.v3.CellarV2_2
	8, // 4: steward.v3.SubmitRequest.cellar_v2dot5:type_name -> steward.v3.CellarV2_5
	0, // 5: steward.v3.ContractCall.Submit:input_type -> steward.v3.SubmitRequest
	2, // 6: steward.v3.Status.Version:input_type -> steward.v3.VersionRequest
	1, // 7: steward.v3.ContractCall.Submit:output_type -> steward.v3.SubmitResponse
	3, // 8: steward.v3.Status.Version:output_type -> steward.v3.VersionResponse
	7, // [7:9] is the sub-list for method output_type
	5, // [5:7] is the sub-list for method input_type
	5, // [5:5] is the sub-list for extension type_name
	5, // [5:5] is the sub-list for extension extendee
	0, // [0:5] is the sub-list for field type_name
}

func init() { file_steward_proto_init() }
func file_steward_proto_init() {
	if File_steward_proto != nil {
		return
	}
	file_aave_v2_stablecoin_proto_init()
	file_cellar_v1_proto_init()
	file_cellar_v2_proto_init()
	if !protoimpl.UnsafeEnabled {
		file_steward_proto_msgTypes[0].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*SubmitRequest); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
		file_steward_proto_msgTypes[1].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*SubmitResponse); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
		file_steward_proto_msgTypes[2].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*VersionRequest); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
		file_steward_proto_msgTypes[3].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*VersionResponse); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
	}
	file_steward_proto_msgTypes[0].OneofWrappers = []interface{}{
		(*SubmitRequest_AaveV2Stablecoin)(nil),
		(*SubmitRequest_CellarV1)(nil),
		(*SubmitRequest_CellarV2)(nil),
		(*SubmitRequest_CellarV2Dot2)(nil),
		(*SubmitRequest_CellarV2Dot5)(nil),
	}
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: file_steward_proto_rawDesc,
			NumEnums:      0,
			NumMessages:   4,
			NumExtensions: 0,
			NumServices:   2,
		},
		GoTypes:           file_steward_proto_goTypes,
		DependencyIndexes: file_steward_proto_depIdxs,
		MessageInfos:      file_steward_proto_msgTypes,
	}.Build()
	File_steward_proto = out.File
	file_steward_proto_rawDesc = nil
	file_steward_proto_goTypes = nil
	file_steward_proto_depIdxs = nil
}
