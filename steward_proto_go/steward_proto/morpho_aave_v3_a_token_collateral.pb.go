//
// Protos for function calls to the Morpho Aave V3 AToken Collateral adaptor.

// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.26.0
// 	protoc        v3.21.12
// source: morpho_aave_v3_a_token_collateral.proto

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

// Represents call data for the Morpho Aave V3 AToken Collateral adaptor.
type MorphoAaveV3ATokenCollateralAdaptorV1 struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// Types that are assignable to Function:
	//	*MorphoAaveV3ATokenCollateralAdaptorV1_RevokeApproval
	//	*MorphoAaveV3ATokenCollateralAdaptorV1_DepositToAaveV3Morpho_
	//	*MorphoAaveV3ATokenCollateralAdaptorV1_WithdrawFromAaveV3Morpho_
	//	*MorphoAaveV3ATokenCollateralAdaptorV1_Claim
	Function isMorphoAaveV3ATokenCollateralAdaptorV1_Function `protobuf_oneof:"function"`
}

func (x *MorphoAaveV3ATokenCollateralAdaptorV1) Reset() {
	*x = MorphoAaveV3ATokenCollateralAdaptorV1{}
	if protoimpl.UnsafeEnabled {
		mi := &file_morpho_aave_v3_a_token_collateral_proto_msgTypes[0]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *MorphoAaveV3ATokenCollateralAdaptorV1) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*MorphoAaveV3ATokenCollateralAdaptorV1) ProtoMessage() {}

func (x *MorphoAaveV3ATokenCollateralAdaptorV1) ProtoReflect() protoreflect.Message {
	mi := &file_morpho_aave_v3_a_token_collateral_proto_msgTypes[0]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use MorphoAaveV3ATokenCollateralAdaptorV1.ProtoReflect.Descriptor instead.
func (*MorphoAaveV3ATokenCollateralAdaptorV1) Descriptor() ([]byte, []int) {
	return file_morpho_aave_v3_a_token_collateral_proto_rawDescGZIP(), []int{0}
}

func (m *MorphoAaveV3ATokenCollateralAdaptorV1) GetFunction() isMorphoAaveV3ATokenCollateralAdaptorV1_Function {
	if m != nil {
		return m.Function
	}
	return nil
}

func (x *MorphoAaveV3ATokenCollateralAdaptorV1) GetRevokeApproval() *RevokeApproval {
	if x, ok := x.GetFunction().(*MorphoAaveV3ATokenCollateralAdaptorV1_RevokeApproval); ok {
		return x.RevokeApproval
	}
	return nil
}

func (x *MorphoAaveV3ATokenCollateralAdaptorV1) GetDepositToAaveV3Morpho() *MorphoAaveV3ATokenCollateralAdaptorV1_DepositToAaveV3Morpho {
	if x, ok := x.GetFunction().(*MorphoAaveV3ATokenCollateralAdaptorV1_DepositToAaveV3Morpho_); ok {
		return x.DepositToAaveV3Morpho
	}
	return nil
}

func (x *MorphoAaveV3ATokenCollateralAdaptorV1) GetWithdrawFromAaveV3Morpho() *MorphoAaveV3ATokenCollateralAdaptorV1_WithdrawFromAaveV3Morpho {
	if x, ok := x.GetFunction().(*MorphoAaveV3ATokenCollateralAdaptorV1_WithdrawFromAaveV3Morpho_); ok {
		return x.WithdrawFromAaveV3Morpho
	}
	return nil
}

func (x *MorphoAaveV3ATokenCollateralAdaptorV1) GetClaim() *Claim {
	if x, ok := x.GetFunction().(*MorphoAaveV3ATokenCollateralAdaptorV1_Claim); ok {
		return x.Claim
	}
	return nil
}

type isMorphoAaveV3ATokenCollateralAdaptorV1_Function interface {
	isMorphoAaveV3ATokenCollateralAdaptorV1_Function()
}

type MorphoAaveV3ATokenCollateralAdaptorV1_RevokeApproval struct {
	// Represents function `revokeApproval(ERC20 asset, address spender)`
	RevokeApproval *RevokeApproval `protobuf:"bytes,1,opt,name=revoke_approval,json=revokeApproval,proto3,oneof"`
}

type MorphoAaveV3ATokenCollateralAdaptorV1_DepositToAaveV3Morpho_ struct {
	// Represents function `depositToAaveV3Morpho(ERC20 tokenToDeposit, uint256 amountToDeposit)`
	DepositToAaveV3Morpho *MorphoAaveV3ATokenCollateralAdaptorV1_DepositToAaveV3Morpho `protobuf:"bytes,2,opt,name=deposit_to_aave_v3_morpho,json=depositToAaveV3Morpho,proto3,oneof"`
}

type MorphoAaveV3ATokenCollateralAdaptorV1_WithdrawFromAaveV3Morpho_ struct {
	// Represents function `withdrawFromAaveV3Morpho(ERC20 tokenToWithdraw, uint256 amountToWithdraw)`
	WithdrawFromAaveV3Morpho *MorphoAaveV3ATokenCollateralAdaptorV1_WithdrawFromAaveV3Morpho `protobuf:"bytes,3,opt,name=withdraw_from_aave_v3_morpho,json=withdrawFromAaveV3Morpho,proto3,oneof"`
}

type MorphoAaveV3ATokenCollateralAdaptorV1_Claim struct {
	// Represents function `claim(uint256 claimable, bytes32[] memory proof)`
	Claim *Claim `protobuf:"bytes,4,opt,name=claim,proto3,oneof"`
}

func (*MorphoAaveV3ATokenCollateralAdaptorV1_RevokeApproval) isMorphoAaveV3ATokenCollateralAdaptorV1_Function() {
}

func (*MorphoAaveV3ATokenCollateralAdaptorV1_DepositToAaveV3Morpho_) isMorphoAaveV3ATokenCollateralAdaptorV1_Function() {
}

func (*MorphoAaveV3ATokenCollateralAdaptorV1_WithdrawFromAaveV3Morpho_) isMorphoAaveV3ATokenCollateralAdaptorV1_Function() {
}

func (*MorphoAaveV3ATokenCollateralAdaptorV1_Claim) isMorphoAaveV3ATokenCollateralAdaptorV1_Function() {
}

type MorphoAaveV3ATokenCollateralAdaptorV1Calls struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Calls []*MorphoAaveV3ATokenCollateralAdaptorV1 `protobuf:"bytes,1,rep,name=calls,proto3" json:"calls,omitempty"`
}

func (x *MorphoAaveV3ATokenCollateralAdaptorV1Calls) Reset() {
	*x = MorphoAaveV3ATokenCollateralAdaptorV1Calls{}
	if protoimpl.UnsafeEnabled {
		mi := &file_morpho_aave_v3_a_token_collateral_proto_msgTypes[1]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *MorphoAaveV3ATokenCollateralAdaptorV1Calls) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*MorphoAaveV3ATokenCollateralAdaptorV1Calls) ProtoMessage() {}

func (x *MorphoAaveV3ATokenCollateralAdaptorV1Calls) ProtoReflect() protoreflect.Message {
	mi := &file_morpho_aave_v3_a_token_collateral_proto_msgTypes[1]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use MorphoAaveV3ATokenCollateralAdaptorV1Calls.ProtoReflect.Descriptor instead.
func (*MorphoAaveV3ATokenCollateralAdaptorV1Calls) Descriptor() ([]byte, []int) {
	return file_morpho_aave_v3_a_token_collateral_proto_rawDescGZIP(), []int{1}
}

func (x *MorphoAaveV3ATokenCollateralAdaptorV1Calls) GetCalls() []*MorphoAaveV3ATokenCollateralAdaptorV1 {
	if x != nil {
		return x.Calls
	}
	return nil
}

//
// Allows strategists to lend assets on Morpho
//
// Represents function `depositToAaveV3Morpho(ERC20 tokenToDeposit, uint256 amountToDeposit)`
type MorphoAaveV3ATokenCollateralAdaptorV1_DepositToAaveV3Morpho struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// The address of the token to deposit
	TokenToDeposit string `protobuf:"bytes,1,opt,name=token_to_deposit,json=tokenToDeposit,proto3" json:"token_to_deposit,omitempty"`
	// The amount of tokens to deposit
	AmountToDeposit string `protobuf:"bytes,2,opt,name=amount_to_deposit,json=amountToDeposit,proto3" json:"amount_to_deposit,omitempty"`
}

func (x *MorphoAaveV3ATokenCollateralAdaptorV1_DepositToAaveV3Morpho) Reset() {
	*x = MorphoAaveV3ATokenCollateralAdaptorV1_DepositToAaveV3Morpho{}
	if protoimpl.UnsafeEnabled {
		mi := &file_morpho_aave_v3_a_token_collateral_proto_msgTypes[2]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *MorphoAaveV3ATokenCollateralAdaptorV1_DepositToAaveV3Morpho) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*MorphoAaveV3ATokenCollateralAdaptorV1_DepositToAaveV3Morpho) ProtoMessage() {}

func (x *MorphoAaveV3ATokenCollateralAdaptorV1_DepositToAaveV3Morpho) ProtoReflect() protoreflect.Message {
	mi := &file_morpho_aave_v3_a_token_collateral_proto_msgTypes[2]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use MorphoAaveV3ATokenCollateralAdaptorV1_DepositToAaveV3Morpho.ProtoReflect.Descriptor instead.
func (*MorphoAaveV3ATokenCollateralAdaptorV1_DepositToAaveV3Morpho) Descriptor() ([]byte, []int) {
	return file_morpho_aave_v3_a_token_collateral_proto_rawDescGZIP(), []int{0, 0}
}

func (x *MorphoAaveV3ATokenCollateralAdaptorV1_DepositToAaveV3Morpho) GetTokenToDeposit() string {
	if x != nil {
		return x.TokenToDeposit
	}
	return ""
}

func (x *MorphoAaveV3ATokenCollateralAdaptorV1_DepositToAaveV3Morpho) GetAmountToDeposit() string {
	if x != nil {
		return x.AmountToDeposit
	}
	return ""
}

//
// Allows strategists to withdraw assets from Morpho
//
// Represents function `withdrawFromAaveV3Morpho(ERC20 tokenToWithdraw, uint256 amountToWithdraw)`
type MorphoAaveV3ATokenCollateralAdaptorV1_WithdrawFromAaveV3Morpho struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// The address of the token to withdraw
	TokenToWithdraw string `protobuf:"bytes,1,opt,name=token_to_withdraw,json=tokenToWithdraw,proto3" json:"token_to_withdraw,omitempty"`
	// The amount of tokens to withdraw
	AmountToWithdraw string `protobuf:"bytes,2,opt,name=amount_to_withdraw,json=amountToWithdraw,proto3" json:"amount_to_withdraw,omitempty"`
}

func (x *MorphoAaveV3ATokenCollateralAdaptorV1_WithdrawFromAaveV3Morpho) Reset() {
	*x = MorphoAaveV3ATokenCollateralAdaptorV1_WithdrawFromAaveV3Morpho{}
	if protoimpl.UnsafeEnabled {
		mi := &file_morpho_aave_v3_a_token_collateral_proto_msgTypes[3]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *MorphoAaveV3ATokenCollateralAdaptorV1_WithdrawFromAaveV3Morpho) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*MorphoAaveV3ATokenCollateralAdaptorV1_WithdrawFromAaveV3Morpho) ProtoMessage() {}

func (x *MorphoAaveV3ATokenCollateralAdaptorV1_WithdrawFromAaveV3Morpho) ProtoReflect() protoreflect.Message {
	mi := &file_morpho_aave_v3_a_token_collateral_proto_msgTypes[3]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use MorphoAaveV3ATokenCollateralAdaptorV1_WithdrawFromAaveV3Morpho.ProtoReflect.Descriptor instead.
func (*MorphoAaveV3ATokenCollateralAdaptorV1_WithdrawFromAaveV3Morpho) Descriptor() ([]byte, []int) {
	return file_morpho_aave_v3_a_token_collateral_proto_rawDescGZIP(), []int{0, 1}
}

func (x *MorphoAaveV3ATokenCollateralAdaptorV1_WithdrawFromAaveV3Morpho) GetTokenToWithdraw() string {
	if x != nil {
		return x.TokenToWithdraw
	}
	return ""
}

func (x *MorphoAaveV3ATokenCollateralAdaptorV1_WithdrawFromAaveV3Morpho) GetAmountToWithdraw() string {
	if x != nil {
		return x.AmountToWithdraw
	}
	return ""
}

var File_morpho_aave_v3_a_token_collateral_proto protoreflect.FileDescriptor

var file_morpho_aave_v3_a_token_collateral_proto_rawDesc = []byte{
	0x0a, 0x27, 0x6d, 0x6f, 0x72, 0x70, 0x68, 0x6f, 0x5f, 0x61, 0x61, 0x76, 0x65, 0x5f, 0x76, 0x33,
	0x5f, 0x61, 0x5f, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x5f, 0x63, 0x6f, 0x6c, 0x6c, 0x61, 0x74, 0x65,
	0x72, 0x61, 0x6c, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0a, 0x73, 0x74, 0x65, 0x77, 0x61,
	0x72, 0x64, 0x2e, 0x76, 0x34, 0x1a, 0x0a, 0x62, 0x61, 0x73, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74,
	0x6f, 0x1a, 0x1b, 0x6d, 0x6f, 0x72, 0x70, 0x68, 0x6f, 0x5f, 0x72, 0x65, 0x77, 0x61, 0x72, 0x64,
	0x5f, 0x68, 0x61, 0x6e, 0x64, 0x6c, 0x65, 0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x9f,
	0x05, 0x0a, 0x25, 0x4d, 0x6f, 0x72, 0x70, 0x68, 0x6f, 0x41, 0x61, 0x76, 0x65, 0x56, 0x33, 0x41,
	0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x43, 0x6f, 0x6c, 0x6c, 0x61, 0x74, 0x65, 0x72, 0x61, 0x6c, 0x41,
	0x64, 0x61, 0x70, 0x74, 0x6f, 0x72, 0x56, 0x31, 0x12, 0x45, 0x0a, 0x0f, 0x72, 0x65, 0x76, 0x6f,
	0x6b, 0x65, 0x5f, 0x61, 0x70, 0x70, 0x72, 0x6f, 0x76, 0x61, 0x6c, 0x18, 0x01, 0x20, 0x01, 0x28,
	0x0b, 0x32, 0x1a, 0x2e, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e, 0x76, 0x34, 0x2e, 0x52,
	0x65, 0x76, 0x6f, 0x6b, 0x65, 0x41, 0x70, 0x70, 0x72, 0x6f, 0x76, 0x61, 0x6c, 0x48, 0x00, 0x52,
	0x0e, 0x72, 0x65, 0x76, 0x6f, 0x6b, 0x65, 0x41, 0x70, 0x70, 0x72, 0x6f, 0x76, 0x61, 0x6c, 0x12,
	0x83, 0x01, 0x0a, 0x19, 0x64, 0x65, 0x70, 0x6f, 0x73, 0x69, 0x74, 0x5f, 0x74, 0x6f, 0x5f, 0x61,
	0x61, 0x76, 0x65, 0x5f, 0x76, 0x33, 0x5f, 0x6d, 0x6f, 0x72, 0x70, 0x68, 0x6f, 0x18, 0x02, 0x20,
	0x01, 0x28, 0x0b, 0x32, 0x47, 0x2e, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e, 0x76, 0x34,
	0x2e, 0x4d, 0x6f, 0x72, 0x70, 0x68, 0x6f, 0x41, 0x61, 0x76, 0x65, 0x56, 0x33, 0x41, 0x54, 0x6f,
	0x6b, 0x65, 0x6e, 0x43, 0x6f, 0x6c, 0x6c, 0x61, 0x74, 0x65, 0x72, 0x61, 0x6c, 0x41, 0x64, 0x61,
	0x70, 0x74, 0x6f, 0x72, 0x56, 0x31, 0x2e, 0x44, 0x65, 0x70, 0x6f, 0x73, 0x69, 0x74, 0x54, 0x6f,
	0x41, 0x61, 0x76, 0x65, 0x56, 0x33, 0x4d, 0x6f, 0x72, 0x70, 0x68, 0x6f, 0x48, 0x00, 0x52, 0x15,
	0x64, 0x65, 0x70, 0x6f, 0x73, 0x69, 0x74, 0x54, 0x6f, 0x41, 0x61, 0x76, 0x65, 0x56, 0x33, 0x4d,
	0x6f, 0x72, 0x70, 0x68, 0x6f, 0x12, 0x8c, 0x01, 0x0a, 0x1c, 0x77, 0x69, 0x74, 0x68, 0x64, 0x72,
	0x61, 0x77, 0x5f, 0x66, 0x72, 0x6f, 0x6d, 0x5f, 0x61, 0x61, 0x76, 0x65, 0x5f, 0x76, 0x33, 0x5f,
	0x6d, 0x6f, 0x72, 0x70, 0x68, 0x6f, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x4a, 0x2e, 0x73,
	0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e, 0x76, 0x34, 0x2e, 0x4d, 0x6f, 0x72, 0x70, 0x68, 0x6f,
	0x41, 0x61, 0x76, 0x65, 0x56, 0x33, 0x41, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x43, 0x6f, 0x6c, 0x6c,
	0x61, 0x74, 0x65, 0x72, 0x61, 0x6c, 0x41, 0x64, 0x61, 0x70, 0x74, 0x6f, 0x72, 0x56, 0x31, 0x2e,
	0x57, 0x69, 0x74, 0x68, 0x64, 0x72, 0x61, 0x77, 0x46, 0x72, 0x6f, 0x6d, 0x41, 0x61, 0x76, 0x65,
	0x56, 0x33, 0x4d, 0x6f, 0x72, 0x70, 0x68, 0x6f, 0x48, 0x00, 0x52, 0x18, 0x77, 0x69, 0x74, 0x68,
	0x64, 0x72, 0x61, 0x77, 0x46, 0x72, 0x6f, 0x6d, 0x41, 0x61, 0x76, 0x65, 0x56, 0x33, 0x4d, 0x6f,
	0x72, 0x70, 0x68, 0x6f, 0x12, 0x29, 0x0a, 0x05, 0x63, 0x6c, 0x61, 0x69, 0x6d, 0x18, 0x04, 0x20,
	0x01, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e, 0x76, 0x34,
	0x2e, 0x43, 0x6c, 0x61, 0x69, 0x6d, 0x48, 0x00, 0x52, 0x05, 0x63, 0x6c, 0x61, 0x69, 0x6d, 0x1a,
	0x6d, 0x0a, 0x15, 0x44, 0x65, 0x70, 0x6f, 0x73, 0x69, 0x74, 0x54, 0x6f, 0x41, 0x61, 0x76, 0x65,
	0x56, 0x33, 0x4d, 0x6f, 0x72, 0x70, 0x68, 0x6f, 0x12, 0x28, 0x0a, 0x10, 0x74, 0x6f, 0x6b, 0x65,
	0x6e, 0x5f, 0x74, 0x6f, 0x5f, 0x64, 0x65, 0x70, 0x6f, 0x73, 0x69, 0x74, 0x18, 0x01, 0x20, 0x01,
	0x28, 0x09, 0x52, 0x0e, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x54, 0x6f, 0x44, 0x65, 0x70, 0x6f, 0x73,
	0x69, 0x74, 0x12, 0x2a, 0x0a, 0x11, 0x61, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x5f, 0x74, 0x6f, 0x5f,
	0x64, 0x65, 0x70, 0x6f, 0x73, 0x69, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0f, 0x61,
	0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x54, 0x6f, 0x44, 0x65, 0x70, 0x6f, 0x73, 0x69, 0x74, 0x1a, 0x74,
	0x0a, 0x18, 0x57, 0x69, 0x74, 0x68, 0x64, 0x72, 0x61, 0x77, 0x46, 0x72, 0x6f, 0x6d, 0x41, 0x61,
	0x76, 0x65, 0x56, 0x33, 0x4d, 0x6f, 0x72, 0x70, 0x68, 0x6f, 0x12, 0x2a, 0x0a, 0x11, 0x74, 0x6f,
	0x6b, 0x65, 0x6e, 0x5f, 0x74, 0x6f, 0x5f, 0x77, 0x69, 0x74, 0x68, 0x64, 0x72, 0x61, 0x77, 0x18,
	0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0f, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x54, 0x6f, 0x57, 0x69,
	0x74, 0x68, 0x64, 0x72, 0x61, 0x77, 0x12, 0x2c, 0x0a, 0x12, 0x61, 0x6d, 0x6f, 0x75, 0x6e, 0x74,
	0x5f, 0x74, 0x6f, 0x5f, 0x77, 0x69, 0x74, 0x68, 0x64, 0x72, 0x61, 0x77, 0x18, 0x02, 0x20, 0x01,
	0x28, 0x09, 0x52, 0x10, 0x61, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x54, 0x6f, 0x57, 0x69, 0x74, 0x68,
	0x64, 0x72, 0x61, 0x77, 0x42, 0x0a, 0x0a, 0x08, 0x66, 0x75, 0x6e, 0x63, 0x74, 0x69, 0x6f, 0x6e,
	0x22, 0x75, 0x0a, 0x2a, 0x4d, 0x6f, 0x72, 0x70, 0x68, 0x6f, 0x41, 0x61, 0x76, 0x65, 0x56, 0x33,
	0x41, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x43, 0x6f, 0x6c, 0x6c, 0x61, 0x74, 0x65, 0x72, 0x61, 0x6c,
	0x41, 0x64, 0x61, 0x70, 0x74, 0x6f, 0x72, 0x56, 0x31, 0x43, 0x61, 0x6c, 0x6c, 0x73, 0x12, 0x47,
	0x0a, 0x05, 0x63, 0x61, 0x6c, 0x6c, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x31, 0x2e,
	0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e, 0x76, 0x34, 0x2e, 0x4d, 0x6f, 0x72, 0x70, 0x68,
	0x6f, 0x41, 0x61, 0x76, 0x65, 0x56, 0x33, 0x41, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x43, 0x6f, 0x6c,
	0x6c, 0x61, 0x74, 0x65, 0x72, 0x61, 0x6c, 0x41, 0x64, 0x61, 0x70, 0x74, 0x6f, 0x72, 0x56, 0x31,
	0x52, 0x05, 0x63, 0x61, 0x6c, 0x6c, 0x73, 0x42, 0x10, 0x5a, 0x0e, 0x2f, 0x73, 0x74, 0x65, 0x77,
	0x61, 0x72, 0x64, 0x5f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f,
	0x33,
}

var (
	file_morpho_aave_v3_a_token_collateral_proto_rawDescOnce sync.Once
	file_morpho_aave_v3_a_token_collateral_proto_rawDescData = file_morpho_aave_v3_a_token_collateral_proto_rawDesc
)

func file_morpho_aave_v3_a_token_collateral_proto_rawDescGZIP() []byte {
	file_morpho_aave_v3_a_token_collateral_proto_rawDescOnce.Do(func() {
		file_morpho_aave_v3_a_token_collateral_proto_rawDescData = protoimpl.X.CompressGZIP(file_morpho_aave_v3_a_token_collateral_proto_rawDescData)
	})
	return file_morpho_aave_v3_a_token_collateral_proto_rawDescData
}

var file_morpho_aave_v3_a_token_collateral_proto_msgTypes = make([]protoimpl.MessageInfo, 4)
var file_morpho_aave_v3_a_token_collateral_proto_goTypes = []interface{}{
	(*MorphoAaveV3ATokenCollateralAdaptorV1)(nil),                          // 0: steward.v4.MorphoAaveV3ATokenCollateralAdaptorV1
	(*MorphoAaveV3ATokenCollateralAdaptorV1Calls)(nil),                     // 1: steward.v4.MorphoAaveV3ATokenCollateralAdaptorV1Calls
	(*MorphoAaveV3ATokenCollateralAdaptorV1_DepositToAaveV3Morpho)(nil),    // 2: steward.v4.MorphoAaveV3ATokenCollateralAdaptorV1.DepositToAaveV3Morpho
	(*MorphoAaveV3ATokenCollateralAdaptorV1_WithdrawFromAaveV3Morpho)(nil), // 3: steward.v4.MorphoAaveV3ATokenCollateralAdaptorV1.WithdrawFromAaveV3Morpho
	(*RevokeApproval)(nil), // 4: steward.v4.RevokeApproval
	(*Claim)(nil),          // 5: steward.v4.Claim
}
var file_morpho_aave_v3_a_token_collateral_proto_depIdxs = []int32{
	4, // 0: steward.v4.MorphoAaveV3ATokenCollateralAdaptorV1.revoke_approval:type_name -> steward.v4.RevokeApproval
	2, // 1: steward.v4.MorphoAaveV3ATokenCollateralAdaptorV1.deposit_to_aave_v3_morpho:type_name -> steward.v4.MorphoAaveV3ATokenCollateralAdaptorV1.DepositToAaveV3Morpho
	3, // 2: steward.v4.MorphoAaveV3ATokenCollateralAdaptorV1.withdraw_from_aave_v3_morpho:type_name -> steward.v4.MorphoAaveV3ATokenCollateralAdaptorV1.WithdrawFromAaveV3Morpho
	5, // 3: steward.v4.MorphoAaveV3ATokenCollateralAdaptorV1.claim:type_name -> steward.v4.Claim
	0, // 4: steward.v4.MorphoAaveV3ATokenCollateralAdaptorV1Calls.calls:type_name -> steward.v4.MorphoAaveV3ATokenCollateralAdaptorV1
	5, // [5:5] is the sub-list for method output_type
	5, // [5:5] is the sub-list for method input_type
	5, // [5:5] is the sub-list for extension type_name
	5, // [5:5] is the sub-list for extension extendee
	0, // [0:5] is the sub-list for field type_name
}

func init() { file_morpho_aave_v3_a_token_collateral_proto_init() }
func file_morpho_aave_v3_a_token_collateral_proto_init() {
	if File_morpho_aave_v3_a_token_collateral_proto != nil {
		return
	}
	file_base_proto_init()
	file_morpho_reward_handler_proto_init()
	if !protoimpl.UnsafeEnabled {
		file_morpho_aave_v3_a_token_collateral_proto_msgTypes[0].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*MorphoAaveV3ATokenCollateralAdaptorV1); i {
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
		file_morpho_aave_v3_a_token_collateral_proto_msgTypes[1].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*MorphoAaveV3ATokenCollateralAdaptorV1Calls); i {
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
		file_morpho_aave_v3_a_token_collateral_proto_msgTypes[2].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*MorphoAaveV3ATokenCollateralAdaptorV1_DepositToAaveV3Morpho); i {
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
		file_morpho_aave_v3_a_token_collateral_proto_msgTypes[3].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*MorphoAaveV3ATokenCollateralAdaptorV1_WithdrawFromAaveV3Morpho); i {
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
	file_morpho_aave_v3_a_token_collateral_proto_msgTypes[0].OneofWrappers = []interface{}{
		(*MorphoAaveV3ATokenCollateralAdaptorV1_RevokeApproval)(nil),
		(*MorphoAaveV3ATokenCollateralAdaptorV1_DepositToAaveV3Morpho_)(nil),
		(*MorphoAaveV3ATokenCollateralAdaptorV1_WithdrawFromAaveV3Morpho_)(nil),
		(*MorphoAaveV3ATokenCollateralAdaptorV1_Claim)(nil),
	}
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: file_morpho_aave_v3_a_token_collateral_proto_rawDesc,
			NumEnums:      0,
			NumMessages:   4,
			NumExtensions: 0,
			NumServices:   0,
		},
		GoTypes:           file_morpho_aave_v3_a_token_collateral_proto_goTypes,
		DependencyIndexes: file_morpho_aave_v3_a_token_collateral_proto_depIdxs,
		MessageInfos:      file_morpho_aave_v3_a_token_collateral_proto_msgTypes,
	}.Build()
	File_morpho_aave_v3_a_token_collateral_proto = out.File
	file_morpho_aave_v3_a_token_collateral_proto_rawDesc = nil
	file_morpho_aave_v3_a_token_collateral_proto_goTypes = nil
	file_morpho_aave_v3_a_token_collateral_proto_depIdxs = nil
}
