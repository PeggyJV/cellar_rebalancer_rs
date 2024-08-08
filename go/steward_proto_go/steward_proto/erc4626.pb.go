//
// Protos for function calls to the ERC4626 adaptor.

// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.33.0
// 	protoc        v4.25.3
// source: erc4626.proto

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

// Represents call data for the ERC4626 adaptor V1
type ERC4626AdaptorV1 struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// Types that are assignable to Function:
	//
	//	*ERC4626AdaptorV1_RevokeApproval
	//	*ERC4626AdaptorV1_DepositToVault_
	//	*ERC4626AdaptorV1_WithdrawFromVault_
	Function isERC4626AdaptorV1_Function `protobuf_oneof:"function"`
}

func (x *ERC4626AdaptorV1) Reset() {
	*x = ERC4626AdaptorV1{}
	if protoimpl.UnsafeEnabled {
		mi := &file_erc4626_proto_msgTypes[0]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *ERC4626AdaptorV1) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*ERC4626AdaptorV1) ProtoMessage() {}

func (x *ERC4626AdaptorV1) ProtoReflect() protoreflect.Message {
	mi := &file_erc4626_proto_msgTypes[0]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use ERC4626AdaptorV1.ProtoReflect.Descriptor instead.
func (*ERC4626AdaptorV1) Descriptor() ([]byte, []int) {
	return file_erc4626_proto_rawDescGZIP(), []int{0}
}

func (m *ERC4626AdaptorV1) GetFunction() isERC4626AdaptorV1_Function {
	if m != nil {
		return m.Function
	}
	return nil
}

func (x *ERC4626AdaptorV1) GetRevokeApproval() *RevokeApproval {
	if x, ok := x.GetFunction().(*ERC4626AdaptorV1_RevokeApproval); ok {
		return x.RevokeApproval
	}
	return nil
}

func (x *ERC4626AdaptorV1) GetDepositToVault() *ERC4626AdaptorV1_DepositToVault {
	if x, ok := x.GetFunction().(*ERC4626AdaptorV1_DepositToVault_); ok {
		return x.DepositToVault
	}
	return nil
}

func (x *ERC4626AdaptorV1) GetWithdrawFromVault() *ERC4626AdaptorV1_WithdrawFromVault {
	if x, ok := x.GetFunction().(*ERC4626AdaptorV1_WithdrawFromVault_); ok {
		return x.WithdrawFromVault
	}
	return nil
}

type isERC4626AdaptorV1_Function interface {
	isERC4626AdaptorV1_Function()
}

type ERC4626AdaptorV1_RevokeApproval struct {
	// Represents function `revokeApproval(ERC20 asset, address spender)`
	RevokeApproval *RevokeApproval `protobuf:"bytes,1,opt,name=revoke_approval,json=revokeApproval,proto3,oneof"`
}

type ERC4626AdaptorV1_DepositToVault_ struct {
	// Represents function `depositToVault(ERC4626 erc4626Vault, uint256 assets)`
	DepositToVault *ERC4626AdaptorV1_DepositToVault `protobuf:"bytes,2,opt,name=deposit_to_vault,json=depositToVault,proto3,oneof"`
}

type ERC4626AdaptorV1_WithdrawFromVault_ struct {
	// Represents function `withdrawFromVault(ERC4626 erc4626Vault, uint256 assets)`
	WithdrawFromVault *ERC4626AdaptorV1_WithdrawFromVault `protobuf:"bytes,3,opt,name=withdraw_from_vault,json=withdrawFromVault,proto3,oneof"`
}

func (*ERC4626AdaptorV1_RevokeApproval) isERC4626AdaptorV1_Function() {}

func (*ERC4626AdaptorV1_DepositToVault_) isERC4626AdaptorV1_Function() {}

func (*ERC4626AdaptorV1_WithdrawFromVault_) isERC4626AdaptorV1_Function() {}

type ERC4626AdaptorV1Calls struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Calls []*ERC4626AdaptorV1 `protobuf:"bytes,1,rep,name=calls,proto3" json:"calls,omitempty"`
}

func (x *ERC4626AdaptorV1Calls) Reset() {
	*x = ERC4626AdaptorV1Calls{}
	if protoimpl.UnsafeEnabled {
		mi := &file_erc4626_proto_msgTypes[1]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *ERC4626AdaptorV1Calls) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*ERC4626AdaptorV1Calls) ProtoMessage() {}

func (x *ERC4626AdaptorV1Calls) ProtoReflect() protoreflect.Message {
	mi := &file_erc4626_proto_msgTypes[1]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use ERC4626AdaptorV1Calls.ProtoReflect.Descriptor instead.
func (*ERC4626AdaptorV1Calls) Descriptor() ([]byte, []int) {
	return file_erc4626_proto_rawDescGZIP(), []int{1}
}

func (x *ERC4626AdaptorV1Calls) GetCalls() []*ERC4626AdaptorV1 {
	if x != nil {
		return x.Calls
	}
	return nil
}

// Allows strategists to deposit into ERC4626 positions.
//
// Represents function `depositToVault(ERC4626 erc4626Vault, uint256 assets)`
type ERC4626AdaptorV1_DepositToVault struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// The address of the ERC4626 vault
	Erc4626Vault string `protobuf:"bytes,1,opt,name=erc4626_vault,json=erc4626Vault,proto3" json:"erc4626_vault,omitempty"`
	// The amount of assets to deposit
	Assets string `protobuf:"bytes,2,opt,name=assets,proto3" json:"assets,omitempty"`
}

func (x *ERC4626AdaptorV1_DepositToVault) Reset() {
	*x = ERC4626AdaptorV1_DepositToVault{}
	if protoimpl.UnsafeEnabled {
		mi := &file_erc4626_proto_msgTypes[2]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *ERC4626AdaptorV1_DepositToVault) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*ERC4626AdaptorV1_DepositToVault) ProtoMessage() {}

func (x *ERC4626AdaptorV1_DepositToVault) ProtoReflect() protoreflect.Message {
	mi := &file_erc4626_proto_msgTypes[2]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use ERC4626AdaptorV1_DepositToVault.ProtoReflect.Descriptor instead.
func (*ERC4626AdaptorV1_DepositToVault) Descriptor() ([]byte, []int) {
	return file_erc4626_proto_rawDescGZIP(), []int{0, 0}
}

func (x *ERC4626AdaptorV1_DepositToVault) GetErc4626Vault() string {
	if x != nil {
		return x.Erc4626Vault
	}
	return ""
}

func (x *ERC4626AdaptorV1_DepositToVault) GetAssets() string {
	if x != nil {
		return x.Assets
	}
	return ""
}

// Allows strategists to withdraw from ERC4626 positions.
//
// Represents function `withdrawFromVault(ERC4626 erc4626Vault, uint256 assets)`
type ERC4626AdaptorV1_WithdrawFromVault struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// The address of the ERC4626 vault
	Erc4626Vault string `protobuf:"bytes,1,opt,name=erc4626_vault,json=erc4626Vault,proto3" json:"erc4626_vault,omitempty"`
	// The amount of assets to withdraw
	Assets string `protobuf:"bytes,2,opt,name=assets,proto3" json:"assets,omitempty"`
}

func (x *ERC4626AdaptorV1_WithdrawFromVault) Reset() {
	*x = ERC4626AdaptorV1_WithdrawFromVault{}
	if protoimpl.UnsafeEnabled {
		mi := &file_erc4626_proto_msgTypes[3]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *ERC4626AdaptorV1_WithdrawFromVault) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*ERC4626AdaptorV1_WithdrawFromVault) ProtoMessage() {}

func (x *ERC4626AdaptorV1_WithdrawFromVault) ProtoReflect() protoreflect.Message {
	mi := &file_erc4626_proto_msgTypes[3]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use ERC4626AdaptorV1_WithdrawFromVault.ProtoReflect.Descriptor instead.
func (*ERC4626AdaptorV1_WithdrawFromVault) Descriptor() ([]byte, []int) {
	return file_erc4626_proto_rawDescGZIP(), []int{0, 1}
}

func (x *ERC4626AdaptorV1_WithdrawFromVault) GetErc4626Vault() string {
	if x != nil {
		return x.Erc4626Vault
	}
	return ""
}

func (x *ERC4626AdaptorV1_WithdrawFromVault) GetAssets() string {
	if x != nil {
		return x.Assets
	}
	return ""
}

var File_erc4626_proto protoreflect.FileDescriptor

var file_erc4626_proto_rawDesc = []byte{
	0x0a, 0x0d, 0x65, 0x72, 0x63, 0x34, 0x36, 0x32, 0x36, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12,
	0x0a, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e, 0x76, 0x34, 0x1a, 0x0a, 0x62, 0x61, 0x73,
	0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xc1, 0x03, 0x0a, 0x10, 0x45, 0x52, 0x43, 0x34,
	0x36, 0x32, 0x36, 0x41, 0x64, 0x61, 0x70, 0x74, 0x6f, 0x72, 0x56, 0x31, 0x12, 0x45, 0x0a, 0x0f,
	0x72, 0x65, 0x76, 0x6f, 0x6b, 0x65, 0x5f, 0x61, 0x70, 0x70, 0x72, 0x6f, 0x76, 0x61, 0x6c, 0x18,
	0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e,
	0x76, 0x34, 0x2e, 0x52, 0x65, 0x76, 0x6f, 0x6b, 0x65, 0x41, 0x70, 0x70, 0x72, 0x6f, 0x76, 0x61,
	0x6c, 0x48, 0x00, 0x52, 0x0e, 0x72, 0x65, 0x76, 0x6f, 0x6b, 0x65, 0x41, 0x70, 0x70, 0x72, 0x6f,
	0x76, 0x61, 0x6c, 0x12, 0x57, 0x0a, 0x10, 0x64, 0x65, 0x70, 0x6f, 0x73, 0x69, 0x74, 0x5f, 0x74,
	0x6f, 0x5f, 0x76, 0x61, 0x75, 0x6c, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x2b, 0x2e,
	0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e, 0x76, 0x34, 0x2e, 0x45, 0x52, 0x43, 0x34, 0x36,
	0x32, 0x36, 0x41, 0x64, 0x61, 0x70, 0x74, 0x6f, 0x72, 0x56, 0x31, 0x2e, 0x44, 0x65, 0x70, 0x6f,
	0x73, 0x69, 0x74, 0x54, 0x6f, 0x56, 0x61, 0x75, 0x6c, 0x74, 0x48, 0x00, 0x52, 0x0e, 0x64, 0x65,
	0x70, 0x6f, 0x73, 0x69, 0x74, 0x54, 0x6f, 0x56, 0x61, 0x75, 0x6c, 0x74, 0x12, 0x60, 0x0a, 0x13,
	0x77, 0x69, 0x74, 0x68, 0x64, 0x72, 0x61, 0x77, 0x5f, 0x66, 0x72, 0x6f, 0x6d, 0x5f, 0x76, 0x61,
	0x75, 0x6c, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x2e, 0x2e, 0x73, 0x74, 0x65, 0x77,
	0x61, 0x72, 0x64, 0x2e, 0x76, 0x34, 0x2e, 0x45, 0x52, 0x43, 0x34, 0x36, 0x32, 0x36, 0x41, 0x64,
	0x61, 0x70, 0x74, 0x6f, 0x72, 0x56, 0x31, 0x2e, 0x57, 0x69, 0x74, 0x68, 0x64, 0x72, 0x61, 0x77,
	0x46, 0x72, 0x6f, 0x6d, 0x56, 0x61, 0x75, 0x6c, 0x74, 0x48, 0x00, 0x52, 0x11, 0x77, 0x69, 0x74,
	0x68, 0x64, 0x72, 0x61, 0x77, 0x46, 0x72, 0x6f, 0x6d, 0x56, 0x61, 0x75, 0x6c, 0x74, 0x1a, 0x4d,
	0x0a, 0x0e, 0x44, 0x65, 0x70, 0x6f, 0x73, 0x69, 0x74, 0x54, 0x6f, 0x56, 0x61, 0x75, 0x6c, 0x74,
	0x12, 0x23, 0x0a, 0x0d, 0x65, 0x72, 0x63, 0x34, 0x36, 0x32, 0x36, 0x5f, 0x76, 0x61, 0x75, 0x6c,
	0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0c, 0x65, 0x72, 0x63, 0x34, 0x36, 0x32, 0x36,
	0x56, 0x61, 0x75, 0x6c, 0x74, 0x12, 0x16, 0x0a, 0x06, 0x61, 0x73, 0x73, 0x65, 0x74, 0x73, 0x18,
	0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x61, 0x73, 0x73, 0x65, 0x74, 0x73, 0x1a, 0x50, 0x0a,
	0x11, 0x57, 0x69, 0x74, 0x68, 0x64, 0x72, 0x61, 0x77, 0x46, 0x72, 0x6f, 0x6d, 0x56, 0x61, 0x75,
	0x6c, 0x74, 0x12, 0x23, 0x0a, 0x0d, 0x65, 0x72, 0x63, 0x34, 0x36, 0x32, 0x36, 0x5f, 0x76, 0x61,
	0x75, 0x6c, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0c, 0x65, 0x72, 0x63, 0x34, 0x36,
	0x32, 0x36, 0x56, 0x61, 0x75, 0x6c, 0x74, 0x12, 0x16, 0x0a, 0x06, 0x61, 0x73, 0x73, 0x65, 0x74,
	0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x61, 0x73, 0x73, 0x65, 0x74, 0x73, 0x42,
	0x0a, 0x0a, 0x08, 0x66, 0x75, 0x6e, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x22, 0x4b, 0x0a, 0x15, 0x45,
	0x52, 0x43, 0x34, 0x36, 0x32, 0x36, 0x41, 0x64, 0x61, 0x70, 0x74, 0x6f, 0x72, 0x56, 0x31, 0x43,
	0x61, 0x6c, 0x6c, 0x73, 0x12, 0x32, 0x0a, 0x05, 0x63, 0x61, 0x6c, 0x6c, 0x73, 0x18, 0x01, 0x20,
	0x03, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e, 0x76, 0x34,
	0x2e, 0x45, 0x52, 0x43, 0x34, 0x36, 0x32, 0x36, 0x41, 0x64, 0x61, 0x70, 0x74, 0x6f, 0x72, 0x56,
	0x31, 0x52, 0x05, 0x63, 0x61, 0x6c, 0x6c, 0x73, 0x42, 0x10, 0x5a, 0x0e, 0x2f, 0x73, 0x74, 0x65,
	0x77, 0x61, 0x72, 0x64, 0x5f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74,
	0x6f, 0x33,
}

var (
	file_erc4626_proto_rawDescOnce sync.Once
	file_erc4626_proto_rawDescData = file_erc4626_proto_rawDesc
)

func file_erc4626_proto_rawDescGZIP() []byte {
	file_erc4626_proto_rawDescOnce.Do(func() {
		file_erc4626_proto_rawDescData = protoimpl.X.CompressGZIP(file_erc4626_proto_rawDescData)
	})
	return file_erc4626_proto_rawDescData
}

var file_erc4626_proto_msgTypes = make([]protoimpl.MessageInfo, 4)
var file_erc4626_proto_goTypes = []interface{}{
	(*ERC4626AdaptorV1)(nil),                   // 0: steward.v4.ERC4626AdaptorV1
	(*ERC4626AdaptorV1Calls)(nil),              // 1: steward.v4.ERC4626AdaptorV1Calls
	(*ERC4626AdaptorV1_DepositToVault)(nil),    // 2: steward.v4.ERC4626AdaptorV1.DepositToVault
	(*ERC4626AdaptorV1_WithdrawFromVault)(nil), // 3: steward.v4.ERC4626AdaptorV1.WithdrawFromVault
	(*RevokeApproval)(nil),                     // 4: steward.v4.RevokeApproval
}
var file_erc4626_proto_depIdxs = []int32{
	4, // 0: steward.v4.ERC4626AdaptorV1.revoke_approval:type_name -> steward.v4.RevokeApproval
	2, // 1: steward.v4.ERC4626AdaptorV1.deposit_to_vault:type_name -> steward.v4.ERC4626AdaptorV1.DepositToVault
	3, // 2: steward.v4.ERC4626AdaptorV1.withdraw_from_vault:type_name -> steward.v4.ERC4626AdaptorV1.WithdrawFromVault
	0, // 3: steward.v4.ERC4626AdaptorV1Calls.calls:type_name -> steward.v4.ERC4626AdaptorV1
	4, // [4:4] is the sub-list for method output_type
	4, // [4:4] is the sub-list for method input_type
	4, // [4:4] is the sub-list for extension type_name
	4, // [4:4] is the sub-list for extension extendee
	0, // [0:4] is the sub-list for field type_name
}

func init() { file_erc4626_proto_init() }
func file_erc4626_proto_init() {
	if File_erc4626_proto != nil {
		return
	}
	file_base_proto_init()
	if !protoimpl.UnsafeEnabled {
		file_erc4626_proto_msgTypes[0].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*ERC4626AdaptorV1); i {
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
		file_erc4626_proto_msgTypes[1].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*ERC4626AdaptorV1Calls); i {
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
		file_erc4626_proto_msgTypes[2].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*ERC4626AdaptorV1_DepositToVault); i {
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
		file_erc4626_proto_msgTypes[3].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*ERC4626AdaptorV1_WithdrawFromVault); i {
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
	file_erc4626_proto_msgTypes[0].OneofWrappers = []interface{}{
		(*ERC4626AdaptorV1_RevokeApproval)(nil),
		(*ERC4626AdaptorV1_DepositToVault_)(nil),
		(*ERC4626AdaptorV1_WithdrawFromVault_)(nil),
	}
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: file_erc4626_proto_rawDesc,
			NumEnums:      0,
			NumMessages:   4,
			NumExtensions: 0,
			NumServices:   0,
		},
		GoTypes:           file_erc4626_proto_goTypes,
		DependencyIndexes: file_erc4626_proto_depIdxs,
		MessageInfos:      file_erc4626_proto_msgTypes,
	}.Build()
	File_erc4626_proto = out.File
	file_erc4626_proto_rawDesc = nil
	file_erc4626_proto_goTypes = nil
	file_erc4626_proto_depIdxs = nil
}