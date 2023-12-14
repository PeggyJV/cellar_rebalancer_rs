//
// Protos for function calls to AaveV2EnableAssetAsCollateralAdaptor.sol

// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.26.0
// 	protoc        v3.21.12
// source: aave_v2_enable_asset_as_collateral_adaptor.proto

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

// Represents call data for the Aave AToken adaptor, used to manage lending positions on Aave
type AaveV2EnableAssetAsCollateralAdaptorV1 struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// Types that are assignable to Function:
	//	*AaveV2EnableAssetAsCollateralAdaptorV1_RevokeApproval
	//	*AaveV2EnableAssetAsCollateralAdaptorV1_SetUserUseReserveAsCollateral_
	Function isAaveV2EnableAssetAsCollateralAdaptorV1_Function `protobuf_oneof:"function"`
}

func (x *AaveV2EnableAssetAsCollateralAdaptorV1) Reset() {
	*x = AaveV2EnableAssetAsCollateralAdaptorV1{}
	if protoimpl.UnsafeEnabled {
		mi := &file_aave_v2_enable_asset_as_collateral_adaptor_proto_msgTypes[0]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *AaveV2EnableAssetAsCollateralAdaptorV1) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*AaveV2EnableAssetAsCollateralAdaptorV1) ProtoMessage() {}

func (x *AaveV2EnableAssetAsCollateralAdaptorV1) ProtoReflect() protoreflect.Message {
	mi := &file_aave_v2_enable_asset_as_collateral_adaptor_proto_msgTypes[0]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use AaveV2EnableAssetAsCollateralAdaptorV1.ProtoReflect.Descriptor instead.
func (*AaveV2EnableAssetAsCollateralAdaptorV1) Descriptor() ([]byte, []int) {
	return file_aave_v2_enable_asset_as_collateral_adaptor_proto_rawDescGZIP(), []int{0}
}

func (m *AaveV2EnableAssetAsCollateralAdaptorV1) GetFunction() isAaveV2EnableAssetAsCollateralAdaptorV1_Function {
	if m != nil {
		return m.Function
	}
	return nil
}

func (x *AaveV2EnableAssetAsCollateralAdaptorV1) GetRevokeApproval() *RevokeApproval {
	if x, ok := x.GetFunction().(*AaveV2EnableAssetAsCollateralAdaptorV1_RevokeApproval); ok {
		return x.RevokeApproval
	}
	return nil
}

func (x *AaveV2EnableAssetAsCollateralAdaptorV1) GetSetUserUseReserveAsCollateral() *AaveV2EnableAssetAsCollateralAdaptorV1_SetUserUseReserveAsCollateral {
	if x, ok := x.GetFunction().(*AaveV2EnableAssetAsCollateralAdaptorV1_SetUserUseReserveAsCollateral_); ok {
		return x.SetUserUseReserveAsCollateral
	}
	return nil
}

type isAaveV2EnableAssetAsCollateralAdaptorV1_Function interface {
	isAaveV2EnableAssetAsCollateralAdaptorV1_Function()
}

type AaveV2EnableAssetAsCollateralAdaptorV1_RevokeApproval struct {
	// Represents function `revokeApproval(ERC20 asset, address spender)`
	RevokeApproval *RevokeApproval `protobuf:"bytes,1,opt,name=revoke_approval,json=revokeApproval,proto3,oneof"`
}

type AaveV2EnableAssetAsCollateralAdaptorV1_SetUserUseReserveAsCollateral_ struct {
	// Represents function `setUserUseReserveAsCollateral(address asset, bool useAsCollateral)`
	SetUserUseReserveAsCollateral *AaveV2EnableAssetAsCollateralAdaptorV1_SetUserUseReserveAsCollateral `protobuf:"bytes,2,opt,name=set_user_use_reserve_as_collateral,json=setUserUseReserveAsCollateral,proto3,oneof"`
}

func (*AaveV2EnableAssetAsCollateralAdaptorV1_RevokeApproval) isAaveV2EnableAssetAsCollateralAdaptorV1_Function() {
}

func (*AaveV2EnableAssetAsCollateralAdaptorV1_SetUserUseReserveAsCollateral_) isAaveV2EnableAssetAsCollateralAdaptorV1_Function() {
}

type AaveV2EnableAssetAsCollateralAdaptorV1Calls struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Calls []*AaveV2EnableAssetAsCollateralAdaptorV1 `protobuf:"bytes,1,rep,name=calls,proto3" json:"calls,omitempty"`
}

func (x *AaveV2EnableAssetAsCollateralAdaptorV1Calls) Reset() {
	*x = AaveV2EnableAssetAsCollateralAdaptorV1Calls{}
	if protoimpl.UnsafeEnabled {
		mi := &file_aave_v2_enable_asset_as_collateral_adaptor_proto_msgTypes[1]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *AaveV2EnableAssetAsCollateralAdaptorV1Calls) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*AaveV2EnableAssetAsCollateralAdaptorV1Calls) ProtoMessage() {}

func (x *AaveV2EnableAssetAsCollateralAdaptorV1Calls) ProtoReflect() protoreflect.Message {
	mi := &file_aave_v2_enable_asset_as_collateral_adaptor_proto_msgTypes[1]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use AaveV2EnableAssetAsCollateralAdaptorV1Calls.ProtoReflect.Descriptor instead.
func (*AaveV2EnableAssetAsCollateralAdaptorV1Calls) Descriptor() ([]byte, []int) {
	return file_aave_v2_enable_asset_as_collateral_adaptor_proto_rawDescGZIP(), []int{1}
}

func (x *AaveV2EnableAssetAsCollateralAdaptorV1Calls) GetCalls() []*AaveV2EnableAssetAsCollateralAdaptorV1 {
	if x != nil {
		return x.Calls
	}
	return nil
}

//
// Allows a strategist to choose to use an asset as collateral or not.
//
// Represents function `setUserUseReserveAsCollateral(address asset, bool useAsCollateral)`
type AaveV2EnableAssetAsCollateralAdaptorV1_SetUserUseReserveAsCollateral struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// The address of the asset to set as collateral
	Asset string `protobuf:"bytes,1,opt,name=asset,proto3" json:"asset,omitempty"`
	// Whether to use the asset as collateral
	UseAsCollateral bool `protobuf:"varint,2,opt,name=use_as_collateral,json=useAsCollateral,proto3" json:"use_as_collateral,omitempty"`
}

func (x *AaveV2EnableAssetAsCollateralAdaptorV1_SetUserUseReserveAsCollateral) Reset() {
	*x = AaveV2EnableAssetAsCollateralAdaptorV1_SetUserUseReserveAsCollateral{}
	if protoimpl.UnsafeEnabled {
		mi := &file_aave_v2_enable_asset_as_collateral_adaptor_proto_msgTypes[2]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *AaveV2EnableAssetAsCollateralAdaptorV1_SetUserUseReserveAsCollateral) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*AaveV2EnableAssetAsCollateralAdaptorV1_SetUserUseReserveAsCollateral) ProtoMessage() {}

func (x *AaveV2EnableAssetAsCollateralAdaptorV1_SetUserUseReserveAsCollateral) ProtoReflect() protoreflect.Message {
	mi := &file_aave_v2_enable_asset_as_collateral_adaptor_proto_msgTypes[2]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use AaveV2EnableAssetAsCollateralAdaptorV1_SetUserUseReserveAsCollateral.ProtoReflect.Descriptor instead.
func (*AaveV2EnableAssetAsCollateralAdaptorV1_SetUserUseReserveAsCollateral) Descriptor() ([]byte, []int) {
	return file_aave_v2_enable_asset_as_collateral_adaptor_proto_rawDescGZIP(), []int{0, 0}
}

func (x *AaveV2EnableAssetAsCollateralAdaptorV1_SetUserUseReserveAsCollateral) GetAsset() string {
	if x != nil {
		return x.Asset
	}
	return ""
}

func (x *AaveV2EnableAssetAsCollateralAdaptorV1_SetUserUseReserveAsCollateral) GetUseAsCollateral() bool {
	if x != nil {
		return x.UseAsCollateral
	}
	return false
}

var File_aave_v2_enable_asset_as_collateral_adaptor_proto protoreflect.FileDescriptor

var file_aave_v2_enable_asset_as_collateral_adaptor_proto_rawDesc = []byte{
	0x0a, 0x30, 0x61, 0x61, 0x76, 0x65, 0x5f, 0x76, 0x32, 0x5f, 0x65, 0x6e, 0x61, 0x62, 0x6c, 0x65,
	0x5f, 0x61, 0x73, 0x73, 0x65, 0x74, 0x5f, 0x61, 0x73, 0x5f, 0x63, 0x6f, 0x6c, 0x6c, 0x61, 0x74,
	0x65, 0x72, 0x61, 0x6c, 0x5f, 0x61, 0x64, 0x61, 0x70, 0x74, 0x6f, 0x72, 0x2e, 0x70, 0x72, 0x6f,
	0x74, 0x6f, 0x12, 0x0a, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e, 0x76, 0x34, 0x1a, 0x0a,
	0x62, 0x61, 0x73, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xfe, 0x02, 0x0a, 0x26, 0x41,
	0x61, 0x76, 0x65, 0x56, 0x32, 0x45, 0x6e, 0x61, 0x62, 0x6c, 0x65, 0x41, 0x73, 0x73, 0x65, 0x74,
	0x41, 0x73, 0x43, 0x6f, 0x6c, 0x6c, 0x61, 0x74, 0x65, 0x72, 0x61, 0x6c, 0x41, 0x64, 0x61, 0x70,
	0x74, 0x6f, 0x72, 0x56, 0x31, 0x12, 0x45, 0x0a, 0x0f, 0x72, 0x65, 0x76, 0x6f, 0x6b, 0x65, 0x5f,
	0x61, 0x70, 0x70, 0x72, 0x6f, 0x76, 0x61, 0x6c, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a,
	0x2e, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e, 0x76, 0x34, 0x2e, 0x52, 0x65, 0x76, 0x6f,
	0x6b, 0x65, 0x41, 0x70, 0x70, 0x72, 0x6f, 0x76, 0x61, 0x6c, 0x48, 0x00, 0x52, 0x0e, 0x72, 0x65,
	0x76, 0x6f, 0x6b, 0x65, 0x41, 0x70, 0x70, 0x72, 0x6f, 0x76, 0x61, 0x6c, 0x12, 0x9d, 0x01, 0x0a,
	0x22, 0x73, 0x65, 0x74, 0x5f, 0x75, 0x73, 0x65, 0x72, 0x5f, 0x75, 0x73, 0x65, 0x5f, 0x72, 0x65,
	0x73, 0x65, 0x72, 0x76, 0x65, 0x5f, 0x61, 0x73, 0x5f, 0x63, 0x6f, 0x6c, 0x6c, 0x61, 0x74, 0x65,
	0x72, 0x61, 0x6c, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x50, 0x2e, 0x73, 0x74, 0x65, 0x77,
	0x61, 0x72, 0x64, 0x2e, 0x76, 0x34, 0x2e, 0x41, 0x61, 0x76, 0x65, 0x56, 0x32, 0x45, 0x6e, 0x61,
	0x62, 0x6c, 0x65, 0x41, 0x73, 0x73, 0x65, 0x74, 0x41, 0x73, 0x43, 0x6f, 0x6c, 0x6c, 0x61, 0x74,
	0x65, 0x72, 0x61, 0x6c, 0x41, 0x64, 0x61, 0x70, 0x74, 0x6f, 0x72, 0x56, 0x31, 0x2e, 0x53, 0x65,
	0x74, 0x55, 0x73, 0x65, 0x72, 0x55, 0x73, 0x65, 0x52, 0x65, 0x73, 0x65, 0x72, 0x76, 0x65, 0x41,
	0x73, 0x43, 0x6f, 0x6c, 0x6c, 0x61, 0x74, 0x65, 0x72, 0x61, 0x6c, 0x48, 0x00, 0x52, 0x1d, 0x73,
	0x65, 0x74, 0x55, 0x73, 0x65, 0x72, 0x55, 0x73, 0x65, 0x52, 0x65, 0x73, 0x65, 0x72, 0x76, 0x65,
	0x41, 0x73, 0x43, 0x6f, 0x6c, 0x6c, 0x61, 0x74, 0x65, 0x72, 0x61, 0x6c, 0x1a, 0x61, 0x0a, 0x1d,
	0x53, 0x65, 0x74, 0x55, 0x73, 0x65, 0x72, 0x55, 0x73, 0x65, 0x52, 0x65, 0x73, 0x65, 0x72, 0x76,
	0x65, 0x41, 0x73, 0x43, 0x6f, 0x6c, 0x6c, 0x61, 0x74, 0x65, 0x72, 0x61, 0x6c, 0x12, 0x14, 0x0a,
	0x05, 0x61, 0x73, 0x73, 0x65, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x61, 0x73,
	0x73, 0x65, 0x74, 0x12, 0x2a, 0x0a, 0x11, 0x75, 0x73, 0x65, 0x5f, 0x61, 0x73, 0x5f, 0x63, 0x6f,
	0x6c, 0x6c, 0x61, 0x74, 0x65, 0x72, 0x61, 0x6c, 0x18, 0x02, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0f,
	0x75, 0x73, 0x65, 0x41, 0x73, 0x43, 0x6f, 0x6c, 0x6c, 0x61, 0x74, 0x65, 0x72, 0x61, 0x6c, 0x42,
	0x0a, 0x0a, 0x08, 0x66, 0x75, 0x6e, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x22, 0x77, 0x0a, 0x2b, 0x41,
	0x61, 0x76, 0x65, 0x56, 0x32, 0x45, 0x6e, 0x61, 0x62, 0x6c, 0x65, 0x41, 0x73, 0x73, 0x65, 0x74,
	0x41, 0x73, 0x43, 0x6f, 0x6c, 0x6c, 0x61, 0x74, 0x65, 0x72, 0x61, 0x6c, 0x41, 0x64, 0x61, 0x70,
	0x74, 0x6f, 0x72, 0x56, 0x31, 0x43, 0x61, 0x6c, 0x6c, 0x73, 0x12, 0x48, 0x0a, 0x05, 0x63, 0x61,
	0x6c, 0x6c, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x32, 0x2e, 0x73, 0x74, 0x65, 0x77,
	0x61, 0x72, 0x64, 0x2e, 0x76, 0x34, 0x2e, 0x41, 0x61, 0x76, 0x65, 0x56, 0x32, 0x45, 0x6e, 0x61,
	0x62, 0x6c, 0x65, 0x41, 0x73, 0x73, 0x65, 0x74, 0x41, 0x73, 0x43, 0x6f, 0x6c, 0x6c, 0x61, 0x74,
	0x65, 0x72, 0x61, 0x6c, 0x41, 0x64, 0x61, 0x70, 0x74, 0x6f, 0x72, 0x56, 0x31, 0x52, 0x05, 0x63,
	0x61, 0x6c, 0x6c, 0x73, 0x42, 0x10, 0x5a, 0x0e, 0x2f, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64,
	0x5f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
}

var (
	file_aave_v2_enable_asset_as_collateral_adaptor_proto_rawDescOnce sync.Once
	file_aave_v2_enable_asset_as_collateral_adaptor_proto_rawDescData = file_aave_v2_enable_asset_as_collateral_adaptor_proto_rawDesc
)

func file_aave_v2_enable_asset_as_collateral_adaptor_proto_rawDescGZIP() []byte {
	file_aave_v2_enable_asset_as_collateral_adaptor_proto_rawDescOnce.Do(func() {
		file_aave_v2_enable_asset_as_collateral_adaptor_proto_rawDescData = protoimpl.X.CompressGZIP(file_aave_v2_enable_asset_as_collateral_adaptor_proto_rawDescData)
	})
	return file_aave_v2_enable_asset_as_collateral_adaptor_proto_rawDescData
}

var file_aave_v2_enable_asset_as_collateral_adaptor_proto_msgTypes = make([]protoimpl.MessageInfo, 3)
var file_aave_v2_enable_asset_as_collateral_adaptor_proto_goTypes = []interface{}{
	(*AaveV2EnableAssetAsCollateralAdaptorV1)(nil),                               // 0: steward.v4.AaveV2EnableAssetAsCollateralAdaptorV1
	(*AaveV2EnableAssetAsCollateralAdaptorV1Calls)(nil),                          // 1: steward.v4.AaveV2EnableAssetAsCollateralAdaptorV1Calls
	(*AaveV2EnableAssetAsCollateralAdaptorV1_SetUserUseReserveAsCollateral)(nil), // 2: steward.v4.AaveV2EnableAssetAsCollateralAdaptorV1.SetUserUseReserveAsCollateral
	(*RevokeApproval)(nil), // 3: steward.v4.RevokeApproval
}
var file_aave_v2_enable_asset_as_collateral_adaptor_proto_depIdxs = []int32{
	3, // 0: steward.v4.AaveV2EnableAssetAsCollateralAdaptorV1.revoke_approval:type_name -> steward.v4.RevokeApproval
	2, // 1: steward.v4.AaveV2EnableAssetAsCollateralAdaptorV1.set_user_use_reserve_as_collateral:type_name -> steward.v4.AaveV2EnableAssetAsCollateralAdaptorV1.SetUserUseReserveAsCollateral
	0, // 2: steward.v4.AaveV2EnableAssetAsCollateralAdaptorV1Calls.calls:type_name -> steward.v4.AaveV2EnableAssetAsCollateralAdaptorV1
	3, // [3:3] is the sub-list for method output_type
	3, // [3:3] is the sub-list for method input_type
	3, // [3:3] is the sub-list for extension type_name
	3, // [3:3] is the sub-list for extension extendee
	0, // [0:3] is the sub-list for field type_name
}

func init() { file_aave_v2_enable_asset_as_collateral_adaptor_proto_init() }
func file_aave_v2_enable_asset_as_collateral_adaptor_proto_init() {
	if File_aave_v2_enable_asset_as_collateral_adaptor_proto != nil {
		return
	}
	file_base_proto_init()
	if !protoimpl.UnsafeEnabled {
		file_aave_v2_enable_asset_as_collateral_adaptor_proto_msgTypes[0].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*AaveV2EnableAssetAsCollateralAdaptorV1); i {
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
		file_aave_v2_enable_asset_as_collateral_adaptor_proto_msgTypes[1].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*AaveV2EnableAssetAsCollateralAdaptorV1Calls); i {
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
		file_aave_v2_enable_asset_as_collateral_adaptor_proto_msgTypes[2].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*AaveV2EnableAssetAsCollateralAdaptorV1_SetUserUseReserveAsCollateral); i {
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
	file_aave_v2_enable_asset_as_collateral_adaptor_proto_msgTypes[0].OneofWrappers = []interface{}{
		(*AaveV2EnableAssetAsCollateralAdaptorV1_RevokeApproval)(nil),
		(*AaveV2EnableAssetAsCollateralAdaptorV1_SetUserUseReserveAsCollateral_)(nil),
	}
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: file_aave_v2_enable_asset_as_collateral_adaptor_proto_rawDesc,
			NumEnums:      0,
			NumMessages:   3,
			NumExtensions: 0,
			NumServices:   0,
		},
		GoTypes:           file_aave_v2_enable_asset_as_collateral_adaptor_proto_goTypes,
		DependencyIndexes: file_aave_v2_enable_asset_as_collateral_adaptor_proto_depIdxs,
		MessageInfos:      file_aave_v2_enable_asset_as_collateral_adaptor_proto_msgTypes,
	}.Build()
	File_aave_v2_enable_asset_as_collateral_adaptor_proto = out.File
	file_aave_v2_enable_asset_as_collateral_adaptor_proto_rawDesc = nil
	file_aave_v2_enable_asset_as_collateral_adaptor_proto_goTypes = nil
	file_aave_v2_enable_asset_as_collateral_adaptor_proto_depIdxs = nil
}
