//
// Protos for function calls to the Frax Collateral F Token adaptor.

// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.26.0
// 	protoc        v3.21.12
// source: collateral_f_token.proto

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

// Represents call data for the Frax Collateral F Token adaptor.
type CollateralFTokenAdaptorV1 struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// Types that are assignable to Function:
	//	*CollateralFTokenAdaptorV1_RevokeApproval
	//	*CollateralFTokenAdaptorV1_AddCollateral_
	//	*CollateralFTokenAdaptorV1_RemoveCollateral_
	Function isCollateralFTokenAdaptorV1_Function `protobuf_oneof:"function"`
}

func (x *CollateralFTokenAdaptorV1) Reset() {
	*x = CollateralFTokenAdaptorV1{}
	if protoimpl.UnsafeEnabled {
		mi := &file_collateral_f_token_proto_msgTypes[0]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *CollateralFTokenAdaptorV1) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*CollateralFTokenAdaptorV1) ProtoMessage() {}

func (x *CollateralFTokenAdaptorV1) ProtoReflect() protoreflect.Message {
	mi := &file_collateral_f_token_proto_msgTypes[0]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use CollateralFTokenAdaptorV1.ProtoReflect.Descriptor instead.
func (*CollateralFTokenAdaptorV1) Descriptor() ([]byte, []int) {
	return file_collateral_f_token_proto_rawDescGZIP(), []int{0}
}

func (m *CollateralFTokenAdaptorV1) GetFunction() isCollateralFTokenAdaptorV1_Function {
	if m != nil {
		return m.Function
	}
	return nil
}

func (x *CollateralFTokenAdaptorV1) GetRevokeApproval() *RevokeApproval {
	if x, ok := x.GetFunction().(*CollateralFTokenAdaptorV1_RevokeApproval); ok {
		return x.RevokeApproval
	}
	return nil
}

func (x *CollateralFTokenAdaptorV1) GetAddCollateral() *CollateralFTokenAdaptorV1_AddCollateral {
	if x, ok := x.GetFunction().(*CollateralFTokenAdaptorV1_AddCollateral_); ok {
		return x.AddCollateral
	}
	return nil
}

func (x *CollateralFTokenAdaptorV1) GetRemoveCollateral() *CollateralFTokenAdaptorV1_RemoveCollateral {
	if x, ok := x.GetFunction().(*CollateralFTokenAdaptorV1_RemoveCollateral_); ok {
		return x.RemoveCollateral
	}
	return nil
}

type isCollateralFTokenAdaptorV1_Function interface {
	isCollateralFTokenAdaptorV1_Function()
}

type CollateralFTokenAdaptorV1_RevokeApproval struct {
	// Represents function `revokeApproval(ERC20 asset, address spender)`
	RevokeApproval *RevokeApproval `protobuf:"bytes,1,opt,name=revoke_approval,json=revokeApproval,proto3,oneof"`
}

type CollateralFTokenAdaptorV1_AddCollateral_ struct {
	// Represents function `addCollateral(IFToken _fraxlendPair, uint256 _collateralToDeposit)`
	AddCollateral *CollateralFTokenAdaptorV1_AddCollateral `protobuf:"bytes,2,opt,name=add_collateral,json=addCollateral,proto3,oneof"`
}

type CollateralFTokenAdaptorV1_RemoveCollateral_ struct {
	// Represents function `removeCollateral(uint256 _collateralAmount, IFToken _fraxlendPair)`
	RemoveCollateral *CollateralFTokenAdaptorV1_RemoveCollateral `protobuf:"bytes,3,opt,name=remove_collateral,json=removeCollateral,proto3,oneof"`
}

func (*CollateralFTokenAdaptorV1_RevokeApproval) isCollateralFTokenAdaptorV1_Function() {}

func (*CollateralFTokenAdaptorV1_AddCollateral_) isCollateralFTokenAdaptorV1_Function() {}

func (*CollateralFTokenAdaptorV1_RemoveCollateral_) isCollateralFTokenAdaptorV1_Function() {}

type CollateralFTokenAdaptorV1Calls struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Calls []*CollateralFTokenAdaptorV1 `protobuf:"bytes,1,rep,name=calls,proto3" json:"calls,omitempty"`
}

func (x *CollateralFTokenAdaptorV1Calls) Reset() {
	*x = CollateralFTokenAdaptorV1Calls{}
	if protoimpl.UnsafeEnabled {
		mi := &file_collateral_f_token_proto_msgTypes[1]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *CollateralFTokenAdaptorV1Calls) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*CollateralFTokenAdaptorV1Calls) ProtoMessage() {}

func (x *CollateralFTokenAdaptorV1Calls) ProtoReflect() protoreflect.Message {
	mi := &file_collateral_f_token_proto_msgTypes[1]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use CollateralFTokenAdaptorV1Calls.ProtoReflect.Descriptor instead.
func (*CollateralFTokenAdaptorV1Calls) Descriptor() ([]byte, []int) {
	return file_collateral_f_token_proto_rawDescGZIP(), []int{1}
}

func (x *CollateralFTokenAdaptorV1Calls) GetCalls() []*CollateralFTokenAdaptorV1 {
	if x != nil {
		return x.Calls
	}
	return nil
}

// Allows strategists to add collateral to the respective cellar position on FraxLend, enabling borrowing.
//
// Represents function `addCollateral(IFToken _fraxlendPair, uint256 _collateralToDeposit)`
type CollateralFTokenAdaptorV1_AddCollateral struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// The FraxLend pair to add collateral to.
	FraxlendPair string `protobuf:"bytes,1,opt,name=fraxlend_pair,json=fraxlendPair,proto3" json:"fraxlend_pair,omitempty"`
	// The amount of collateral to add to the cellar position.
	CollateralToDeposit string `protobuf:"bytes,2,opt,name=collateral_to_deposit,json=collateralToDeposit,proto3" json:"collateral_to_deposit,omitempty"`
}

func (x *CollateralFTokenAdaptorV1_AddCollateral) Reset() {
	*x = CollateralFTokenAdaptorV1_AddCollateral{}
	if protoimpl.UnsafeEnabled {
		mi := &file_collateral_f_token_proto_msgTypes[2]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *CollateralFTokenAdaptorV1_AddCollateral) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*CollateralFTokenAdaptorV1_AddCollateral) ProtoMessage() {}

func (x *CollateralFTokenAdaptorV1_AddCollateral) ProtoReflect() protoreflect.Message {
	mi := &file_collateral_f_token_proto_msgTypes[2]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use CollateralFTokenAdaptorV1_AddCollateral.ProtoReflect.Descriptor instead.
func (*CollateralFTokenAdaptorV1_AddCollateral) Descriptor() ([]byte, []int) {
	return file_collateral_f_token_proto_rawDescGZIP(), []int{0, 0}
}

func (x *CollateralFTokenAdaptorV1_AddCollateral) GetFraxlendPair() string {
	if x != nil {
		return x.FraxlendPair
	}
	return ""
}

func (x *CollateralFTokenAdaptorV1_AddCollateral) GetCollateralToDeposit() string {
	if x != nil {
		return x.CollateralToDeposit
	}
	return ""
}

// Allows strategists to remove collateral from the respective cellar position on FraxLend.
//
// Represents function `removeCollateral(uint256 _collateralAmount, IFToken _fraxlendPair)`
type CollateralFTokenAdaptorV1_RemoveCollateral struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// The amount of collateral to remove from the cellar position.
	CollateralAmount string `protobuf:"bytes,1,opt,name=collateral_amount,json=collateralAmount,proto3" json:"collateral_amount,omitempty"`
	// The FraxLend pair to remove collateral from.
	FraxlendPair string `protobuf:"bytes,2,opt,name=fraxlend_pair,json=fraxlendPair,proto3" json:"fraxlend_pair,omitempty"`
}

func (x *CollateralFTokenAdaptorV1_RemoveCollateral) Reset() {
	*x = CollateralFTokenAdaptorV1_RemoveCollateral{}
	if protoimpl.UnsafeEnabled {
		mi := &file_collateral_f_token_proto_msgTypes[3]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *CollateralFTokenAdaptorV1_RemoveCollateral) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*CollateralFTokenAdaptorV1_RemoveCollateral) ProtoMessage() {}

func (x *CollateralFTokenAdaptorV1_RemoveCollateral) ProtoReflect() protoreflect.Message {
	mi := &file_collateral_f_token_proto_msgTypes[3]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use CollateralFTokenAdaptorV1_RemoveCollateral.ProtoReflect.Descriptor instead.
func (*CollateralFTokenAdaptorV1_RemoveCollateral) Descriptor() ([]byte, []int) {
	return file_collateral_f_token_proto_rawDescGZIP(), []int{0, 1}
}

func (x *CollateralFTokenAdaptorV1_RemoveCollateral) GetCollateralAmount() string {
	if x != nil {
		return x.CollateralAmount
	}
	return ""
}

func (x *CollateralFTokenAdaptorV1_RemoveCollateral) GetFraxlendPair() string {
	if x != nil {
		return x.FraxlendPair
	}
	return ""
}

var File_collateral_f_token_proto protoreflect.FileDescriptor

var file_collateral_f_token_proto_rawDesc = []byte{
	0x0a, 0x18, 0x63, 0x6f, 0x6c, 0x6c, 0x61, 0x74, 0x65, 0x72, 0x61, 0x6c, 0x5f, 0x66, 0x5f, 0x74,
	0x6f, 0x6b, 0x65, 0x6e, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0a, 0x73, 0x74, 0x65, 0x77,
	0x61, 0x72, 0x64, 0x2e, 0x76, 0x34, 0x1a, 0x0a, 0x62, 0x61, 0x73, 0x65, 0x2e, 0x70, 0x72, 0x6f,
	0x74, 0x6f, 0x22, 0x83, 0x04, 0x0a, 0x19, 0x43, 0x6f, 0x6c, 0x6c, 0x61, 0x74, 0x65, 0x72, 0x61,
	0x6c, 0x46, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x41, 0x64, 0x61, 0x70, 0x74, 0x6f, 0x72, 0x56, 0x31,
	0x12, 0x45, 0x0a, 0x0f, 0x72, 0x65, 0x76, 0x6f, 0x6b, 0x65, 0x5f, 0x61, 0x70, 0x70, 0x72, 0x6f,
	0x76, 0x61, 0x6c, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x73, 0x74, 0x65, 0x77,
	0x61, 0x72, 0x64, 0x2e, 0x76, 0x34, 0x2e, 0x52, 0x65, 0x76, 0x6f, 0x6b, 0x65, 0x41, 0x70, 0x70,
	0x72, 0x6f, 0x76, 0x61, 0x6c, 0x48, 0x00, 0x52, 0x0e, 0x72, 0x65, 0x76, 0x6f, 0x6b, 0x65, 0x41,
	0x70, 0x70, 0x72, 0x6f, 0x76, 0x61, 0x6c, 0x12, 0x5c, 0x0a, 0x0e, 0x61, 0x64, 0x64, 0x5f, 0x63,
	0x6f, 0x6c, 0x6c, 0x61, 0x74, 0x65, 0x72, 0x61, 0x6c, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32,
	0x33, 0x2e, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e, 0x76, 0x34, 0x2e, 0x43, 0x6f, 0x6c,
	0x6c, 0x61, 0x74, 0x65, 0x72, 0x61, 0x6c, 0x46, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x41, 0x64, 0x61,
	0x70, 0x74, 0x6f, 0x72, 0x56, 0x31, 0x2e, 0x41, 0x64, 0x64, 0x43, 0x6f, 0x6c, 0x6c, 0x61, 0x74,
	0x65, 0x72, 0x61, 0x6c, 0x48, 0x00, 0x52, 0x0d, 0x61, 0x64, 0x64, 0x43, 0x6f, 0x6c, 0x6c, 0x61,
	0x74, 0x65, 0x72, 0x61, 0x6c, 0x12, 0x65, 0x0a, 0x11, 0x72, 0x65, 0x6d, 0x6f, 0x76, 0x65, 0x5f,
	0x63, 0x6f, 0x6c, 0x6c, 0x61, 0x74, 0x65, 0x72, 0x61, 0x6c, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b,
	0x32, 0x36, 0x2e, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e, 0x76, 0x34, 0x2e, 0x43, 0x6f,
	0x6c, 0x6c, 0x61, 0x74, 0x65, 0x72, 0x61, 0x6c, 0x46, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x41, 0x64,
	0x61, 0x70, 0x74, 0x6f, 0x72, 0x56, 0x31, 0x2e, 0x52, 0x65, 0x6d, 0x6f, 0x76, 0x65, 0x43, 0x6f,
	0x6c, 0x6c, 0x61, 0x74, 0x65, 0x72, 0x61, 0x6c, 0x48, 0x00, 0x52, 0x10, 0x72, 0x65, 0x6d, 0x6f,
	0x76, 0x65, 0x43, 0x6f, 0x6c, 0x6c, 0x61, 0x74, 0x65, 0x72, 0x61, 0x6c, 0x1a, 0x68, 0x0a, 0x0d,
	0x41, 0x64, 0x64, 0x43, 0x6f, 0x6c, 0x6c, 0x61, 0x74, 0x65, 0x72, 0x61, 0x6c, 0x12, 0x23, 0x0a,
	0x0d, 0x66, 0x72, 0x61, 0x78, 0x6c, 0x65, 0x6e, 0x64, 0x5f, 0x70, 0x61, 0x69, 0x72, 0x18, 0x01,
	0x20, 0x01, 0x28, 0x09, 0x52, 0x0c, 0x66, 0x72, 0x61, 0x78, 0x6c, 0x65, 0x6e, 0x64, 0x50, 0x61,
	0x69, 0x72, 0x12, 0x32, 0x0a, 0x15, 0x63, 0x6f, 0x6c, 0x6c, 0x61, 0x74, 0x65, 0x72, 0x61, 0x6c,
	0x5f, 0x74, 0x6f, 0x5f, 0x64, 0x65, 0x70, 0x6f, 0x73, 0x69, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28,
	0x09, 0x52, 0x13, 0x63, 0x6f, 0x6c, 0x6c, 0x61, 0x74, 0x65, 0x72, 0x61, 0x6c, 0x54, 0x6f, 0x44,
	0x65, 0x70, 0x6f, 0x73, 0x69, 0x74, 0x1a, 0x64, 0x0a, 0x10, 0x52, 0x65, 0x6d, 0x6f, 0x76, 0x65,
	0x43, 0x6f, 0x6c, 0x6c, 0x61, 0x74, 0x65, 0x72, 0x61, 0x6c, 0x12, 0x2b, 0x0a, 0x11, 0x63, 0x6f,
	0x6c, 0x6c, 0x61, 0x74, 0x65, 0x72, 0x61, 0x6c, 0x5f, 0x61, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x18,
	0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x10, 0x63, 0x6f, 0x6c, 0x6c, 0x61, 0x74, 0x65, 0x72, 0x61,
	0x6c, 0x41, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x12, 0x23, 0x0a, 0x0d, 0x66, 0x72, 0x61, 0x78, 0x6c,
	0x65, 0x6e, 0x64, 0x5f, 0x70, 0x61, 0x69, 0x72, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0c,
	0x66, 0x72, 0x61, 0x78, 0x6c, 0x65, 0x6e, 0x64, 0x50, 0x61, 0x69, 0x72, 0x42, 0x0a, 0x0a, 0x08,
	0x66, 0x75, 0x6e, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x22, 0x5d, 0x0a, 0x1e, 0x43, 0x6f, 0x6c, 0x6c,
	0x61, 0x74, 0x65, 0x72, 0x61, 0x6c, 0x46, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x41, 0x64, 0x61, 0x70,
	0x74, 0x6f, 0x72, 0x56, 0x31, 0x43, 0x61, 0x6c, 0x6c, 0x73, 0x12, 0x3b, 0x0a, 0x05, 0x63, 0x61,
	0x6c, 0x6c, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x25, 0x2e, 0x73, 0x74, 0x65, 0x77,
	0x61, 0x72, 0x64, 0x2e, 0x76, 0x34, 0x2e, 0x43, 0x6f, 0x6c, 0x6c, 0x61, 0x74, 0x65, 0x72, 0x61,
	0x6c, 0x46, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x41, 0x64, 0x61, 0x70, 0x74, 0x6f, 0x72, 0x56, 0x31,
	0x52, 0x05, 0x63, 0x61, 0x6c, 0x6c, 0x73, 0x42, 0x10, 0x5a, 0x0e, 0x2f, 0x73, 0x74, 0x65, 0x77,
	0x61, 0x72, 0x64, 0x5f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f,
	0x33,
}

var (
	file_collateral_f_token_proto_rawDescOnce sync.Once
	file_collateral_f_token_proto_rawDescData = file_collateral_f_token_proto_rawDesc
)

func file_collateral_f_token_proto_rawDescGZIP() []byte {
	file_collateral_f_token_proto_rawDescOnce.Do(func() {
		file_collateral_f_token_proto_rawDescData = protoimpl.X.CompressGZIP(file_collateral_f_token_proto_rawDescData)
	})
	return file_collateral_f_token_proto_rawDescData
}

var file_collateral_f_token_proto_msgTypes = make([]protoimpl.MessageInfo, 4)
var file_collateral_f_token_proto_goTypes = []interface{}{
	(*CollateralFTokenAdaptorV1)(nil),                  // 0: steward.v4.CollateralFTokenAdaptorV1
	(*CollateralFTokenAdaptorV1Calls)(nil),             // 1: steward.v4.CollateralFTokenAdaptorV1Calls
	(*CollateralFTokenAdaptorV1_AddCollateral)(nil),    // 2: steward.v4.CollateralFTokenAdaptorV1.AddCollateral
	(*CollateralFTokenAdaptorV1_RemoveCollateral)(nil), // 3: steward.v4.CollateralFTokenAdaptorV1.RemoveCollateral
	(*RevokeApproval)(nil),                             // 4: steward.v4.RevokeApproval
}
var file_collateral_f_token_proto_depIdxs = []int32{
	4, // 0: steward.v4.CollateralFTokenAdaptorV1.revoke_approval:type_name -> steward.v4.RevokeApproval
	2, // 1: steward.v4.CollateralFTokenAdaptorV1.add_collateral:type_name -> steward.v4.CollateralFTokenAdaptorV1.AddCollateral
	3, // 2: steward.v4.CollateralFTokenAdaptorV1.remove_collateral:type_name -> steward.v4.CollateralFTokenAdaptorV1.RemoveCollateral
	0, // 3: steward.v4.CollateralFTokenAdaptorV1Calls.calls:type_name -> steward.v4.CollateralFTokenAdaptorV1
	4, // [4:4] is the sub-list for method output_type
	4, // [4:4] is the sub-list for method input_type
	4, // [4:4] is the sub-list for extension type_name
	4, // [4:4] is the sub-list for extension extendee
	0, // [0:4] is the sub-list for field type_name
}

func init() { file_collateral_f_token_proto_init() }
func file_collateral_f_token_proto_init() {
	if File_collateral_f_token_proto != nil {
		return
	}
	file_base_proto_init()
	if !protoimpl.UnsafeEnabled {
		file_collateral_f_token_proto_msgTypes[0].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*CollateralFTokenAdaptorV1); i {
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
		file_collateral_f_token_proto_msgTypes[1].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*CollateralFTokenAdaptorV1Calls); i {
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
		file_collateral_f_token_proto_msgTypes[2].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*CollateralFTokenAdaptorV1_AddCollateral); i {
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
		file_collateral_f_token_proto_msgTypes[3].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*CollateralFTokenAdaptorV1_RemoveCollateral); i {
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
	file_collateral_f_token_proto_msgTypes[0].OneofWrappers = []interface{}{
		(*CollateralFTokenAdaptorV1_RevokeApproval)(nil),
		(*CollateralFTokenAdaptorV1_AddCollateral_)(nil),
		(*CollateralFTokenAdaptorV1_RemoveCollateral_)(nil),
	}
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: file_collateral_f_token_proto_rawDesc,
			NumEnums:      0,
			NumMessages:   4,
			NumExtensions: 0,
			NumServices:   0,
		},
		GoTypes:           file_collateral_f_token_proto_goTypes,
		DependencyIndexes: file_collateral_f_token_proto_depIdxs,
		MessageInfos:      file_collateral_f_token_proto_msgTypes,
	}.Build()
	File_collateral_f_token_proto = out.File
	file_collateral_f_token_proto_rawDesc = nil
	file_collateral_f_token_proto_goTypes = nil
	file_collateral_f_token_proto_depIdxs = nil
}
