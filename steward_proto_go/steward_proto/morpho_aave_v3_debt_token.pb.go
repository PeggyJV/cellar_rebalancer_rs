//
// Protos for function calls to the Morpho Aave V3 Debt Token adaptor.

// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.26.0
// 	protoc        v4.25.1
// source: adaptors/morpho/morpho_aave_v3_debt_token.proto

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

// Represents call data for the Morpho Aave V3 Debt Token adaptor.
type MorphoAaveV3DebtTokenAdaptorV1 struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// Types that are assignable to Function:
	//	*MorphoAaveV3DebtTokenAdaptorV1_RevokeApproval
	//	*MorphoAaveV3DebtTokenAdaptorV1_BorrowFromAaveV3Morpho_
	//	*MorphoAaveV3DebtTokenAdaptorV1_RepayAaveV3MorphoDebt_
	Function isMorphoAaveV3DebtTokenAdaptorV1_Function `protobuf_oneof:"function"`
}

func (x *MorphoAaveV3DebtTokenAdaptorV1) Reset() {
	*x = MorphoAaveV3DebtTokenAdaptorV1{}
	if protoimpl.UnsafeEnabled {
		mi := &file_adaptors_morpho_morpho_aave_v3_debt_token_proto_msgTypes[0]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *MorphoAaveV3DebtTokenAdaptorV1) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*MorphoAaveV3DebtTokenAdaptorV1) ProtoMessage() {}

func (x *MorphoAaveV3DebtTokenAdaptorV1) ProtoReflect() protoreflect.Message {
	mi := &file_adaptors_morpho_morpho_aave_v3_debt_token_proto_msgTypes[0]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use MorphoAaveV3DebtTokenAdaptorV1.ProtoReflect.Descriptor instead.
func (*MorphoAaveV3DebtTokenAdaptorV1) Descriptor() ([]byte, []int) {
	return file_adaptors_morpho_morpho_aave_v3_debt_token_proto_rawDescGZIP(), []int{0}
}

func (m *MorphoAaveV3DebtTokenAdaptorV1) GetFunction() isMorphoAaveV3DebtTokenAdaptorV1_Function {
	if m != nil {
		return m.Function
	}
	return nil
}

func (x *MorphoAaveV3DebtTokenAdaptorV1) GetRevokeApproval() *RevokeApproval {
	if x, ok := x.GetFunction().(*MorphoAaveV3DebtTokenAdaptorV1_RevokeApproval); ok {
		return x.RevokeApproval
	}
	return nil
}

func (x *MorphoAaveV3DebtTokenAdaptorV1) GetBorrowFromAaveV3Morpho() *MorphoAaveV3DebtTokenAdaptorV1_BorrowFromAaveV3Morpho {
	if x, ok := x.GetFunction().(*MorphoAaveV3DebtTokenAdaptorV1_BorrowFromAaveV3Morpho_); ok {
		return x.BorrowFromAaveV3Morpho
	}
	return nil
}

func (x *MorphoAaveV3DebtTokenAdaptorV1) GetRepayAaveV3MorphoDebt() *MorphoAaveV3DebtTokenAdaptorV1_RepayAaveV3MorphoDebt {
	if x, ok := x.GetFunction().(*MorphoAaveV3DebtTokenAdaptorV1_RepayAaveV3MorphoDebt_); ok {
		return x.RepayAaveV3MorphoDebt
	}
	return nil
}

type isMorphoAaveV3DebtTokenAdaptorV1_Function interface {
	isMorphoAaveV3DebtTokenAdaptorV1_Function()
}

type MorphoAaveV3DebtTokenAdaptorV1_RevokeApproval struct {
	// Represents function `revokeApproval(ERC20 asset, address spender)`
	RevokeApproval *RevokeApproval `protobuf:"bytes,1,opt,name=revoke_approval,json=revokeApproval,proto3,oneof"`
}

type MorphoAaveV3DebtTokenAdaptorV1_BorrowFromAaveV3Morpho_ struct {
	// Represents function `borrowFromAaveV3Morpho(address underlying, uint256 amountToBorrow, uint256 maxIterations)`
	BorrowFromAaveV3Morpho *MorphoAaveV3DebtTokenAdaptorV1_BorrowFromAaveV3Morpho `protobuf:"bytes,2,opt,name=borrow_from_aave_v3_morpho,json=borrowFromAaveV3Morpho,proto3,oneof"`
}

type MorphoAaveV3DebtTokenAdaptorV1_RepayAaveV3MorphoDebt_ struct {
	// Represents function `repayAaveV3MorphoDebt(ERC20 tokenToRepay, uint256 amountToRepay)`
	RepayAaveV3MorphoDebt *MorphoAaveV3DebtTokenAdaptorV1_RepayAaveV3MorphoDebt `protobuf:"bytes,3,opt,name=repay_aave_v3_morpho_debt,json=repayAaveV3MorphoDebt,proto3,oneof"`
}

func (*MorphoAaveV3DebtTokenAdaptorV1_RevokeApproval) isMorphoAaveV3DebtTokenAdaptorV1_Function() {}

func (*MorphoAaveV3DebtTokenAdaptorV1_BorrowFromAaveV3Morpho_) isMorphoAaveV3DebtTokenAdaptorV1_Function() {
}

func (*MorphoAaveV3DebtTokenAdaptorV1_RepayAaveV3MorphoDebt_) isMorphoAaveV3DebtTokenAdaptorV1_Function() {
}

type MorphoAaveV3DebtTokenAdaptorV1Calls struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Calls []*MorphoAaveV3DebtTokenAdaptorV1 `protobuf:"bytes,1,rep,name=calls,proto3" json:"calls,omitempty"`
}

func (x *MorphoAaveV3DebtTokenAdaptorV1Calls) Reset() {
	*x = MorphoAaveV3DebtTokenAdaptorV1Calls{}
	if protoimpl.UnsafeEnabled {
		mi := &file_adaptors_morpho_morpho_aave_v3_debt_token_proto_msgTypes[1]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *MorphoAaveV3DebtTokenAdaptorV1Calls) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*MorphoAaveV3DebtTokenAdaptorV1Calls) ProtoMessage() {}

func (x *MorphoAaveV3DebtTokenAdaptorV1Calls) ProtoReflect() protoreflect.Message {
	mi := &file_adaptors_morpho_morpho_aave_v3_debt_token_proto_msgTypes[1]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use MorphoAaveV3DebtTokenAdaptorV1Calls.ProtoReflect.Descriptor instead.
func (*MorphoAaveV3DebtTokenAdaptorV1Calls) Descriptor() ([]byte, []int) {
	return file_adaptors_morpho_morpho_aave_v3_debt_token_proto_rawDescGZIP(), []int{1}
}

func (x *MorphoAaveV3DebtTokenAdaptorV1Calls) GetCalls() []*MorphoAaveV3DebtTokenAdaptorV1 {
	if x != nil {
		return x.Calls
	}
	return nil
}

//
// Allows strategists to borrow assets from Morpho
//
// Represents function `borrowFromAaveV3Morpho(address underlying, uint256 amountToBorrow, uint256 maxIterations)`
type MorphoAaveV3DebtTokenAdaptorV1_BorrowFromAaveV3Morpho struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// The underlying asset to borrow
	Underlying string `protobuf:"bytes,1,opt,name=underlying,proto3" json:"underlying,omitempty"`
	// The amount of the underlying asset to borrow
	AmountToBorrow string `protobuf:"bytes,2,opt,name=amount_to_borrow,json=amountToBorrow,proto3" json:"amount_to_borrow,omitempty"`
	// The maximum number of iterations to perform
	MaxIterations string `protobuf:"bytes,3,opt,name=max_iterations,json=maxIterations,proto3" json:"max_iterations,omitempty"`
}

func (x *MorphoAaveV3DebtTokenAdaptorV1_BorrowFromAaveV3Morpho) Reset() {
	*x = MorphoAaveV3DebtTokenAdaptorV1_BorrowFromAaveV3Morpho{}
	if protoimpl.UnsafeEnabled {
		mi := &file_adaptors_morpho_morpho_aave_v3_debt_token_proto_msgTypes[2]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *MorphoAaveV3DebtTokenAdaptorV1_BorrowFromAaveV3Morpho) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*MorphoAaveV3DebtTokenAdaptorV1_BorrowFromAaveV3Morpho) ProtoMessage() {}

func (x *MorphoAaveV3DebtTokenAdaptorV1_BorrowFromAaveV3Morpho) ProtoReflect() protoreflect.Message {
	mi := &file_adaptors_morpho_morpho_aave_v3_debt_token_proto_msgTypes[2]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use MorphoAaveV3DebtTokenAdaptorV1_BorrowFromAaveV3Morpho.ProtoReflect.Descriptor instead.
func (*MorphoAaveV3DebtTokenAdaptorV1_BorrowFromAaveV3Morpho) Descriptor() ([]byte, []int) {
	return file_adaptors_morpho_morpho_aave_v3_debt_token_proto_rawDescGZIP(), []int{0, 0}
}

func (x *MorphoAaveV3DebtTokenAdaptorV1_BorrowFromAaveV3Morpho) GetUnderlying() string {
	if x != nil {
		return x.Underlying
	}
	return ""
}

func (x *MorphoAaveV3DebtTokenAdaptorV1_BorrowFromAaveV3Morpho) GetAmountToBorrow() string {
	if x != nil {
		return x.AmountToBorrow
	}
	return ""
}

func (x *MorphoAaveV3DebtTokenAdaptorV1_BorrowFromAaveV3Morpho) GetMaxIterations() string {
	if x != nil {
		return x.MaxIterations
	}
	return ""
}

//
// Allows strategists to repay loan debt on Morpho
//
// Represents function `repayAaveV3MorphoDebt(ERC20 tokenToRepay, uint256 amountToRepay)`
type MorphoAaveV3DebtTokenAdaptorV1_RepayAaveV3MorphoDebt struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// The token to repay
	TokenToRepay string `protobuf:"bytes,1,opt,name=token_to_repay,json=tokenToRepay,proto3" json:"token_to_repay,omitempty"`
	// The amount of the token to repay
	AmountToRepay string `protobuf:"bytes,2,opt,name=amount_to_repay,json=amountToRepay,proto3" json:"amount_to_repay,omitempty"`
}

func (x *MorphoAaveV3DebtTokenAdaptorV1_RepayAaveV3MorphoDebt) Reset() {
	*x = MorphoAaveV3DebtTokenAdaptorV1_RepayAaveV3MorphoDebt{}
	if protoimpl.UnsafeEnabled {
		mi := &file_adaptors_morpho_morpho_aave_v3_debt_token_proto_msgTypes[3]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *MorphoAaveV3DebtTokenAdaptorV1_RepayAaveV3MorphoDebt) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*MorphoAaveV3DebtTokenAdaptorV1_RepayAaveV3MorphoDebt) ProtoMessage() {}

func (x *MorphoAaveV3DebtTokenAdaptorV1_RepayAaveV3MorphoDebt) ProtoReflect() protoreflect.Message {
	mi := &file_adaptors_morpho_morpho_aave_v3_debt_token_proto_msgTypes[3]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use MorphoAaveV3DebtTokenAdaptorV1_RepayAaveV3MorphoDebt.ProtoReflect.Descriptor instead.
func (*MorphoAaveV3DebtTokenAdaptorV1_RepayAaveV3MorphoDebt) Descriptor() ([]byte, []int) {
	return file_adaptors_morpho_morpho_aave_v3_debt_token_proto_rawDescGZIP(), []int{0, 1}
}

func (x *MorphoAaveV3DebtTokenAdaptorV1_RepayAaveV3MorphoDebt) GetTokenToRepay() string {
	if x != nil {
		return x.TokenToRepay
	}
	return ""
}

func (x *MorphoAaveV3DebtTokenAdaptorV1_RepayAaveV3MorphoDebt) GetAmountToRepay() string {
	if x != nil {
		return x.AmountToRepay
	}
	return ""
}

var File_adaptors_morpho_morpho_aave_v3_debt_token_proto protoreflect.FileDescriptor

var file_adaptors_morpho_morpho_aave_v3_debt_token_proto_rawDesc = []byte{
	0x0a, 0x2f, 0x61, 0x64, 0x61, 0x70, 0x74, 0x6f, 0x72, 0x73, 0x2f, 0x6d, 0x6f, 0x72, 0x70, 0x68,
	0x6f, 0x2f, 0x6d, 0x6f, 0x72, 0x70, 0x68, 0x6f, 0x5f, 0x61, 0x61, 0x76, 0x65, 0x5f, 0x76, 0x33,
	0x5f, 0x64, 0x65, 0x62, 0x74, 0x5f, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x2e, 0x70, 0x72, 0x6f, 0x74,
	0x6f, 0x12, 0x0a, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e, 0x76, 0x33, 0x1a, 0x13, 0x61,
	0x64, 0x61, 0x70, 0x74, 0x6f, 0x72, 0x73, 0x2f, 0x62, 0x61, 0x73, 0x65, 0x2e, 0x70, 0x72, 0x6f,
	0x74, 0x6f, 0x22, 0xe5, 0x04, 0x0a, 0x1e, 0x4d, 0x6f, 0x72, 0x70, 0x68, 0x6f, 0x41, 0x61, 0x76,
	0x65, 0x56, 0x33, 0x44, 0x65, 0x62, 0x74, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x41, 0x64, 0x61, 0x70,
	0x74, 0x6f, 0x72, 0x56, 0x31, 0x12, 0x45, 0x0a, 0x0f, 0x72, 0x65, 0x76, 0x6f, 0x6b, 0x65, 0x5f,
	0x61, 0x70, 0x70, 0x72, 0x6f, 0x76, 0x61, 0x6c, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a,
	0x2e, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e, 0x76, 0x33, 0x2e, 0x52, 0x65, 0x76, 0x6f,
	0x6b, 0x65, 0x41, 0x70, 0x70, 0x72, 0x6f, 0x76, 0x61, 0x6c, 0x48, 0x00, 0x52, 0x0e, 0x72, 0x65,
	0x76, 0x6f, 0x6b, 0x65, 0x41, 0x70, 0x70, 0x72, 0x6f, 0x76, 0x61, 0x6c, 0x12, 0x7f, 0x0a, 0x1a,
	0x62, 0x6f, 0x72, 0x72, 0x6f, 0x77, 0x5f, 0x66, 0x72, 0x6f, 0x6d, 0x5f, 0x61, 0x61, 0x76, 0x65,
	0x5f, 0x76, 0x33, 0x5f, 0x6d, 0x6f, 0x72, 0x70, 0x68, 0x6f, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b,
	0x32, 0x41, 0x2e, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e, 0x76, 0x33, 0x2e, 0x4d, 0x6f,
	0x72, 0x70, 0x68, 0x6f, 0x41, 0x61, 0x76, 0x65, 0x56, 0x33, 0x44, 0x65, 0x62, 0x74, 0x54, 0x6f,
	0x6b, 0x65, 0x6e, 0x41, 0x64, 0x61, 0x70, 0x74, 0x6f, 0x72, 0x56, 0x31, 0x2e, 0x42, 0x6f, 0x72,
	0x72, 0x6f, 0x77, 0x46, 0x72, 0x6f, 0x6d, 0x41, 0x61, 0x76, 0x65, 0x56, 0x33, 0x4d, 0x6f, 0x72,
	0x70, 0x68, 0x6f, 0x48, 0x00, 0x52, 0x16, 0x62, 0x6f, 0x72, 0x72, 0x6f, 0x77, 0x46, 0x72, 0x6f,
	0x6d, 0x41, 0x61, 0x76, 0x65, 0x56, 0x33, 0x4d, 0x6f, 0x72, 0x70, 0x68, 0x6f, 0x12, 0x7c, 0x0a,
	0x19, 0x72, 0x65, 0x70, 0x61, 0x79, 0x5f, 0x61, 0x61, 0x76, 0x65, 0x5f, 0x76, 0x33, 0x5f, 0x6d,
	0x6f, 0x72, 0x70, 0x68, 0x6f, 0x5f, 0x64, 0x65, 0x62, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b,
	0x32, 0x40, 0x2e, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e, 0x76, 0x33, 0x2e, 0x4d, 0x6f,
	0x72, 0x70, 0x68, 0x6f, 0x41, 0x61, 0x76, 0x65, 0x56, 0x33, 0x44, 0x65, 0x62, 0x74, 0x54, 0x6f,
	0x6b, 0x65, 0x6e, 0x41, 0x64, 0x61, 0x70, 0x74, 0x6f, 0x72, 0x56, 0x31, 0x2e, 0x52, 0x65, 0x70,
	0x61, 0x79, 0x41, 0x61, 0x76, 0x65, 0x56, 0x33, 0x4d, 0x6f, 0x72, 0x70, 0x68, 0x6f, 0x44, 0x65,
	0x62, 0x74, 0x48, 0x00, 0x52, 0x15, 0x72, 0x65, 0x70, 0x61, 0x79, 0x41, 0x61, 0x76, 0x65, 0x56,
	0x33, 0x4d, 0x6f, 0x72, 0x70, 0x68, 0x6f, 0x44, 0x65, 0x62, 0x74, 0x1a, 0x89, 0x01, 0x0a, 0x16,
	0x42, 0x6f, 0x72, 0x72, 0x6f, 0x77, 0x46, 0x72, 0x6f, 0x6d, 0x41, 0x61, 0x76, 0x65, 0x56, 0x33,
	0x4d, 0x6f, 0x72, 0x70, 0x68, 0x6f, 0x12, 0x1e, 0x0a, 0x0a, 0x75, 0x6e, 0x64, 0x65, 0x72, 0x6c,
	0x79, 0x69, 0x6e, 0x67, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0a, 0x75, 0x6e, 0x64, 0x65,
	0x72, 0x6c, 0x79, 0x69, 0x6e, 0x67, 0x12, 0x28, 0x0a, 0x10, 0x61, 0x6d, 0x6f, 0x75, 0x6e, 0x74,
	0x5f, 0x74, 0x6f, 0x5f, 0x62, 0x6f, 0x72, 0x72, 0x6f, 0x77, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09,
	0x52, 0x0e, 0x61, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x54, 0x6f, 0x42, 0x6f, 0x72, 0x72, 0x6f, 0x77,
	0x12, 0x25, 0x0a, 0x0e, 0x6d, 0x61, 0x78, 0x5f, 0x69, 0x74, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f,
	0x6e, 0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0d, 0x6d, 0x61, 0x78, 0x49, 0x74, 0x65,
	0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x1a, 0x65, 0x0a, 0x15, 0x52, 0x65, 0x70, 0x61, 0x79,
	0x41, 0x61, 0x76, 0x65, 0x56, 0x33, 0x4d, 0x6f, 0x72, 0x70, 0x68, 0x6f, 0x44, 0x65, 0x62, 0x74,
	0x12, 0x24, 0x0a, 0x0e, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x5f, 0x74, 0x6f, 0x5f, 0x72, 0x65, 0x70,
	0x61, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0c, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x54,
	0x6f, 0x52, 0x65, 0x70, 0x61, 0x79, 0x12, 0x26, 0x0a, 0x0f, 0x61, 0x6d, 0x6f, 0x75, 0x6e, 0x74,
	0x5f, 0x74, 0x6f, 0x5f, 0x72, 0x65, 0x70, 0x61, 0x79, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52,
	0x0d, 0x61, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x54, 0x6f, 0x52, 0x65, 0x70, 0x61, 0x79, 0x42, 0x0a,
	0x0a, 0x08, 0x66, 0x75, 0x6e, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x22, 0x67, 0x0a, 0x23, 0x4d, 0x6f,
	0x72, 0x70, 0x68, 0x6f, 0x41, 0x61, 0x76, 0x65, 0x56, 0x33, 0x44, 0x65, 0x62, 0x74, 0x54, 0x6f,
	0x6b, 0x65, 0x6e, 0x41, 0x64, 0x61, 0x70, 0x74, 0x6f, 0x72, 0x56, 0x31, 0x43, 0x61, 0x6c, 0x6c,
	0x73, 0x12, 0x40, 0x0a, 0x05, 0x63, 0x61, 0x6c, 0x6c, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b,
	0x32, 0x2a, 0x2e, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e, 0x76, 0x33, 0x2e, 0x4d, 0x6f,
	0x72, 0x70, 0x68, 0x6f, 0x41, 0x61, 0x76, 0x65, 0x56, 0x33, 0x44, 0x65, 0x62, 0x74, 0x54, 0x6f,
	0x6b, 0x65, 0x6e, 0x41, 0x64, 0x61, 0x70, 0x74, 0x6f, 0x72, 0x56, 0x31, 0x52, 0x05, 0x63, 0x61,
	0x6c, 0x6c, 0x73, 0x42, 0x10, 0x5a, 0x0e, 0x2f, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x5f,
	0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
}

var (
	file_adaptors_morpho_morpho_aave_v3_debt_token_proto_rawDescOnce sync.Once
	file_adaptors_morpho_morpho_aave_v3_debt_token_proto_rawDescData = file_adaptors_morpho_morpho_aave_v3_debt_token_proto_rawDesc
)

func file_adaptors_morpho_morpho_aave_v3_debt_token_proto_rawDescGZIP() []byte {
	file_adaptors_morpho_morpho_aave_v3_debt_token_proto_rawDescOnce.Do(func() {
		file_adaptors_morpho_morpho_aave_v3_debt_token_proto_rawDescData = protoimpl.X.CompressGZIP(file_adaptors_morpho_morpho_aave_v3_debt_token_proto_rawDescData)
	})
	return file_adaptors_morpho_morpho_aave_v3_debt_token_proto_rawDescData
}

var file_adaptors_morpho_morpho_aave_v3_debt_token_proto_msgTypes = make([]protoimpl.MessageInfo, 4)
var file_adaptors_morpho_morpho_aave_v3_debt_token_proto_goTypes = []interface{}{
	(*MorphoAaveV3DebtTokenAdaptorV1)(nil),                        // 0: steward.v3.MorphoAaveV3DebtTokenAdaptorV1
	(*MorphoAaveV3DebtTokenAdaptorV1Calls)(nil),                   // 1: steward.v3.MorphoAaveV3DebtTokenAdaptorV1Calls
	(*MorphoAaveV3DebtTokenAdaptorV1_BorrowFromAaveV3Morpho)(nil), // 2: steward.v3.MorphoAaveV3DebtTokenAdaptorV1.BorrowFromAaveV3Morpho
	(*MorphoAaveV3DebtTokenAdaptorV1_RepayAaveV3MorphoDebt)(nil),  // 3: steward.v3.MorphoAaveV3DebtTokenAdaptorV1.RepayAaveV3MorphoDebt
	(*RevokeApproval)(nil),                                        // 4: steward.v3.RevokeApproval
}
var file_adaptors_morpho_morpho_aave_v3_debt_token_proto_depIdxs = []int32{
	4, // 0: steward.v3.MorphoAaveV3DebtTokenAdaptorV1.revoke_approval:type_name -> steward.v3.RevokeApproval
	2, // 1: steward.v3.MorphoAaveV3DebtTokenAdaptorV1.borrow_from_aave_v3_morpho:type_name -> steward.v3.MorphoAaveV3DebtTokenAdaptorV1.BorrowFromAaveV3Morpho
	3, // 2: steward.v3.MorphoAaveV3DebtTokenAdaptorV1.repay_aave_v3_morpho_debt:type_name -> steward.v3.MorphoAaveV3DebtTokenAdaptorV1.RepayAaveV3MorphoDebt
	0, // 3: steward.v3.MorphoAaveV3DebtTokenAdaptorV1Calls.calls:type_name -> steward.v3.MorphoAaveV3DebtTokenAdaptorV1
	4, // [4:4] is the sub-list for method output_type
	4, // [4:4] is the sub-list for method input_type
	4, // [4:4] is the sub-list for extension type_name
	4, // [4:4] is the sub-list for extension extendee
	0, // [0:4] is the sub-list for field type_name
}

func init() { file_adaptors_morpho_morpho_aave_v3_debt_token_proto_init() }
func file_adaptors_morpho_morpho_aave_v3_debt_token_proto_init() {
	if File_adaptors_morpho_morpho_aave_v3_debt_token_proto != nil {
		return
	}
	file_adaptors_base_proto_init()
	if !protoimpl.UnsafeEnabled {
		file_adaptors_morpho_morpho_aave_v3_debt_token_proto_msgTypes[0].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*MorphoAaveV3DebtTokenAdaptorV1); i {
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
		file_adaptors_morpho_morpho_aave_v3_debt_token_proto_msgTypes[1].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*MorphoAaveV3DebtTokenAdaptorV1Calls); i {
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
		file_adaptors_morpho_morpho_aave_v3_debt_token_proto_msgTypes[2].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*MorphoAaveV3DebtTokenAdaptorV1_BorrowFromAaveV3Morpho); i {
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
		file_adaptors_morpho_morpho_aave_v3_debt_token_proto_msgTypes[3].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*MorphoAaveV3DebtTokenAdaptorV1_RepayAaveV3MorphoDebt); i {
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
	file_adaptors_morpho_morpho_aave_v3_debt_token_proto_msgTypes[0].OneofWrappers = []interface{}{
		(*MorphoAaveV3DebtTokenAdaptorV1_RevokeApproval)(nil),
		(*MorphoAaveV3DebtTokenAdaptorV1_BorrowFromAaveV3Morpho_)(nil),
		(*MorphoAaveV3DebtTokenAdaptorV1_RepayAaveV3MorphoDebt_)(nil),
	}
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: file_adaptors_morpho_morpho_aave_v3_debt_token_proto_rawDesc,
			NumEnums:      0,
			NumMessages:   4,
			NumExtensions: 0,
			NumServices:   0,
		},
		GoTypes:           file_adaptors_morpho_morpho_aave_v3_debt_token_proto_goTypes,
		DependencyIndexes: file_adaptors_morpho_morpho_aave_v3_debt_token_proto_depIdxs,
		MessageInfos:      file_adaptors_morpho_morpho_aave_v3_debt_token_proto_msgTypes,
	}.Build()
	File_adaptors_morpho_morpho_aave_v3_debt_token_proto = out.File
	file_adaptors_morpho_morpho_aave_v3_debt_token_proto_rawDesc = nil
	file_adaptors_morpho_morpho_aave_v3_debt_token_proto_goTypes = nil
	file_adaptors_morpho_morpho_aave_v3_debt_token_proto_depIdxs = nil
}
