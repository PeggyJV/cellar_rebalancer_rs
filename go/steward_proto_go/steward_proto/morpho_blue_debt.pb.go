//
// Protos for function calls to the Morpho Blue Debt adaptor.

// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.33.0
// 	protoc        v5.27.1
// source: morpho_blue_debt.proto

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

// Represents call data for the Morpho Blue Debt adaptor.
type MorphoBlueDebtAdaptorV1 struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// Types that are assignable to Function:
	//
	//	*MorphoBlueDebtAdaptorV1_RevokeApproval
	//	*MorphoBlueDebtAdaptorV1_BorrowFromMorphoBlue_
	//	*MorphoBlueDebtAdaptorV1_RepayMorphoBlueDebt_
	Function isMorphoBlueDebtAdaptorV1_Function `protobuf_oneof:"function"`
}

func (x *MorphoBlueDebtAdaptorV1) Reset() {
	*x = MorphoBlueDebtAdaptorV1{}
	if protoimpl.UnsafeEnabled {
		mi := &file_morpho_blue_debt_proto_msgTypes[0]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *MorphoBlueDebtAdaptorV1) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*MorphoBlueDebtAdaptorV1) ProtoMessage() {}

func (x *MorphoBlueDebtAdaptorV1) ProtoReflect() protoreflect.Message {
	mi := &file_morpho_blue_debt_proto_msgTypes[0]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use MorphoBlueDebtAdaptorV1.ProtoReflect.Descriptor instead.
func (*MorphoBlueDebtAdaptorV1) Descriptor() ([]byte, []int) {
	return file_morpho_blue_debt_proto_rawDescGZIP(), []int{0}
}

func (m *MorphoBlueDebtAdaptorV1) GetFunction() isMorphoBlueDebtAdaptorV1_Function {
	if m != nil {
		return m.Function
	}
	return nil
}

func (x *MorphoBlueDebtAdaptorV1) GetRevokeApproval() *RevokeApproval {
	if x, ok := x.GetFunction().(*MorphoBlueDebtAdaptorV1_RevokeApproval); ok {
		return x.RevokeApproval
	}
	return nil
}

func (x *MorphoBlueDebtAdaptorV1) GetBorrowFromMorphoBlue() *MorphoBlueDebtAdaptorV1_BorrowFromMorphoBlue {
	if x, ok := x.GetFunction().(*MorphoBlueDebtAdaptorV1_BorrowFromMorphoBlue_); ok {
		return x.BorrowFromMorphoBlue
	}
	return nil
}

func (x *MorphoBlueDebtAdaptorV1) GetRepayMorphoBlueDebt() *MorphoBlueDebtAdaptorV1_RepayMorphoBlueDebt {
	if x, ok := x.GetFunction().(*MorphoBlueDebtAdaptorV1_RepayMorphoBlueDebt_); ok {
		return x.RepayMorphoBlueDebt
	}
	return nil
}

type isMorphoBlueDebtAdaptorV1_Function interface {
	isMorphoBlueDebtAdaptorV1_Function()
}

type MorphoBlueDebtAdaptorV1_RevokeApproval struct {
	// Represents function `revokeApproval(ERC20 asset, address spender)`
	RevokeApproval *RevokeApproval `protobuf:"bytes,1,opt,name=revoke_approval,json=revokeApproval,proto3,oneof"`
}

type MorphoBlueDebtAdaptorV1_BorrowFromMorphoBlue_ struct {
	// Represents function `borrowFromMorphoBlue(MarketParams memory _market, uint256 _amountToBorrow)`
	BorrowFromMorphoBlue *MorphoBlueDebtAdaptorV1_BorrowFromMorphoBlue `protobuf:"bytes,2,opt,name=borrow_from_morpho_blue,json=borrowFromMorphoBlue,proto3,oneof"`
}

type MorphoBlueDebtAdaptorV1_RepayMorphoBlueDebt_ struct {
	// Represents function `repayMorphoBlueDebt(MarketParams memory _market, uint256 _debtTokenRepayAmount)`
	RepayMorphoBlueDebt *MorphoBlueDebtAdaptorV1_RepayMorphoBlueDebt `protobuf:"bytes,3,opt,name=repay_morpho_blue_debt,json=repayMorphoBlueDebt,proto3,oneof"`
}

func (*MorphoBlueDebtAdaptorV1_RevokeApproval) isMorphoBlueDebtAdaptorV1_Function() {}

func (*MorphoBlueDebtAdaptorV1_BorrowFromMorphoBlue_) isMorphoBlueDebtAdaptorV1_Function() {}

func (*MorphoBlueDebtAdaptorV1_RepayMorphoBlueDebt_) isMorphoBlueDebtAdaptorV1_Function() {}

type MorphoBlueDebtAdaptorV1Calls struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Calls []*MorphoBlueDebtAdaptorV1 `protobuf:"bytes,1,rep,name=calls,proto3" json:"calls,omitempty"`
}

func (x *MorphoBlueDebtAdaptorV1Calls) Reset() {
	*x = MorphoBlueDebtAdaptorV1Calls{}
	if protoimpl.UnsafeEnabled {
		mi := &file_morpho_blue_debt_proto_msgTypes[1]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *MorphoBlueDebtAdaptorV1Calls) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*MorphoBlueDebtAdaptorV1Calls) ProtoMessage() {}

func (x *MorphoBlueDebtAdaptorV1Calls) ProtoReflect() protoreflect.Message {
	mi := &file_morpho_blue_debt_proto_msgTypes[1]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use MorphoBlueDebtAdaptorV1Calls.ProtoReflect.Descriptor instead.
func (*MorphoBlueDebtAdaptorV1Calls) Descriptor() ([]byte, []int) {
	return file_morpho_blue_debt_proto_rawDescGZIP(), []int{1}
}

func (x *MorphoBlueDebtAdaptorV1Calls) GetCalls() []*MorphoBlueDebtAdaptorV1 {
	if x != nil {
		return x.Calls
	}
	return nil
}

// Allows strategists borrow a specific amount of an asset on Morpho Blue
//
// Represents function `borrowFromMorphoBlue(MarketParams memory _market, uint256 _amountToBorrow)`
type MorphoBlueDebtAdaptorV1_BorrowFromMorphoBlue struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// Identifier of a Morpho Blue Market
	Market *MarketParams `protobuf:"bytes,1,opt,name=market,proto3" json:"market,omitempty"`
	// The amount of the debt token to borrow
	AmountToBorrow string `protobuf:"bytes,2,opt,name=amount_to_borrow,json=amountToBorrow,proto3" json:"amount_to_borrow,omitempty"`
}

func (x *MorphoBlueDebtAdaptorV1_BorrowFromMorphoBlue) Reset() {
	*x = MorphoBlueDebtAdaptorV1_BorrowFromMorphoBlue{}
	if protoimpl.UnsafeEnabled {
		mi := &file_morpho_blue_debt_proto_msgTypes[2]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *MorphoBlueDebtAdaptorV1_BorrowFromMorphoBlue) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*MorphoBlueDebtAdaptorV1_BorrowFromMorphoBlue) ProtoMessage() {}

func (x *MorphoBlueDebtAdaptorV1_BorrowFromMorphoBlue) ProtoReflect() protoreflect.Message {
	mi := &file_morpho_blue_debt_proto_msgTypes[2]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use MorphoBlueDebtAdaptorV1_BorrowFromMorphoBlue.ProtoReflect.Descriptor instead.
func (*MorphoBlueDebtAdaptorV1_BorrowFromMorphoBlue) Descriptor() ([]byte, []int) {
	return file_morpho_blue_debt_proto_rawDescGZIP(), []int{0, 0}
}

func (x *MorphoBlueDebtAdaptorV1_BorrowFromMorphoBlue) GetMarket() *MarketParams {
	if x != nil {
		return x.Market
	}
	return nil
}

func (x *MorphoBlueDebtAdaptorV1_BorrowFromMorphoBlue) GetAmountToBorrow() string {
	if x != nil {
		return x.AmountToBorrow
	}
	return ""
}

// Allows strategists to repay loan debt on Morph Blue Lending Market. Make sure to call addInterest() beforehand to ensure we are repaying what is required.
//
// Represents function `repayMorphoBlueDebt(MarketParams memory _market, uint256 _debtTokenRepayAmount)`
type MorphoBlueDebtAdaptorV1_RepayMorphoBlueDebt struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// Identifier of a Morpho Blue Market
	Market *MarketParams `protobuf:"bytes,1,opt,name=market,proto3" json:"market,omitempty"`
	// The amount of the debt token to repay
	DebtTokenRepayAmount string `protobuf:"bytes,2,opt,name=debt_token_repay_amount,json=debtTokenRepayAmount,proto3" json:"debt_token_repay_amount,omitempty"`
}

func (x *MorphoBlueDebtAdaptorV1_RepayMorphoBlueDebt) Reset() {
	*x = MorphoBlueDebtAdaptorV1_RepayMorphoBlueDebt{}
	if protoimpl.UnsafeEnabled {
		mi := &file_morpho_blue_debt_proto_msgTypes[3]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *MorphoBlueDebtAdaptorV1_RepayMorphoBlueDebt) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*MorphoBlueDebtAdaptorV1_RepayMorphoBlueDebt) ProtoMessage() {}

func (x *MorphoBlueDebtAdaptorV1_RepayMorphoBlueDebt) ProtoReflect() protoreflect.Message {
	mi := &file_morpho_blue_debt_proto_msgTypes[3]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use MorphoBlueDebtAdaptorV1_RepayMorphoBlueDebt.ProtoReflect.Descriptor instead.
func (*MorphoBlueDebtAdaptorV1_RepayMorphoBlueDebt) Descriptor() ([]byte, []int) {
	return file_morpho_blue_debt_proto_rawDescGZIP(), []int{0, 1}
}

func (x *MorphoBlueDebtAdaptorV1_RepayMorphoBlueDebt) GetMarket() *MarketParams {
	if x != nil {
		return x.Market
	}
	return nil
}

func (x *MorphoBlueDebtAdaptorV1_RepayMorphoBlueDebt) GetDebtTokenRepayAmount() string {
	if x != nil {
		return x.DebtTokenRepayAmount
	}
	return ""
}

var File_morpho_blue_debt_proto protoreflect.FileDescriptor

var file_morpho_blue_debt_proto_rawDesc = []byte{
	0x0a, 0x16, 0x6d, 0x6f, 0x72, 0x70, 0x68, 0x6f, 0x5f, 0x62, 0x6c, 0x75, 0x65, 0x5f, 0x64, 0x65,
	0x62, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0a, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72,
	0x64, 0x2e, 0x76, 0x34, 0x1a, 0x0a, 0x62, 0x61, 0x73, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
	0x1a, 0x0c, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xc3,
	0x04, 0x0a, 0x17, 0x4d, 0x6f, 0x72, 0x70, 0x68, 0x6f, 0x42, 0x6c, 0x75, 0x65, 0x44, 0x65, 0x62,
	0x74, 0x41, 0x64, 0x61, 0x70, 0x74, 0x6f, 0x72, 0x56, 0x31, 0x12, 0x45, 0x0a, 0x0f, 0x72, 0x65,
	0x76, 0x6f, 0x6b, 0x65, 0x5f, 0x61, 0x70, 0x70, 0x72, 0x6f, 0x76, 0x61, 0x6c, 0x18, 0x01, 0x20,
	0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e, 0x76, 0x34,
	0x2e, 0x52, 0x65, 0x76, 0x6f, 0x6b, 0x65, 0x41, 0x70, 0x70, 0x72, 0x6f, 0x76, 0x61, 0x6c, 0x48,
	0x00, 0x52, 0x0e, 0x72, 0x65, 0x76, 0x6f, 0x6b, 0x65, 0x41, 0x70, 0x70, 0x72, 0x6f, 0x76, 0x61,
	0x6c, 0x12, 0x71, 0x0a, 0x17, 0x62, 0x6f, 0x72, 0x72, 0x6f, 0x77, 0x5f, 0x66, 0x72, 0x6f, 0x6d,
	0x5f, 0x6d, 0x6f, 0x72, 0x70, 0x68, 0x6f, 0x5f, 0x62, 0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x01,
	0x28, 0x0b, 0x32, 0x38, 0x2e, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e, 0x76, 0x34, 0x2e,
	0x4d, 0x6f, 0x72, 0x70, 0x68, 0x6f, 0x42, 0x6c, 0x75, 0x65, 0x44, 0x65, 0x62, 0x74, 0x41, 0x64,
	0x61, 0x70, 0x74, 0x6f, 0x72, 0x56, 0x31, 0x2e, 0x42, 0x6f, 0x72, 0x72, 0x6f, 0x77, 0x46, 0x72,
	0x6f, 0x6d, 0x4d, 0x6f, 0x72, 0x70, 0x68, 0x6f, 0x42, 0x6c, 0x75, 0x65, 0x48, 0x00, 0x52, 0x14,
	0x62, 0x6f, 0x72, 0x72, 0x6f, 0x77, 0x46, 0x72, 0x6f, 0x6d, 0x4d, 0x6f, 0x72, 0x70, 0x68, 0x6f,
	0x42, 0x6c, 0x75, 0x65, 0x12, 0x6e, 0x0a, 0x16, 0x72, 0x65, 0x70, 0x61, 0x79, 0x5f, 0x6d, 0x6f,
	0x72, 0x70, 0x68, 0x6f, 0x5f, 0x62, 0x6c, 0x75, 0x65, 0x5f, 0x64, 0x65, 0x62, 0x74, 0x18, 0x03,
	0x20, 0x01, 0x28, 0x0b, 0x32, 0x37, 0x2e, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e, 0x76,
	0x34, 0x2e, 0x4d, 0x6f, 0x72, 0x70, 0x68, 0x6f, 0x42, 0x6c, 0x75, 0x65, 0x44, 0x65, 0x62, 0x74,
	0x41, 0x64, 0x61, 0x70, 0x74, 0x6f, 0x72, 0x56, 0x31, 0x2e, 0x52, 0x65, 0x70, 0x61, 0x79, 0x4d,
	0x6f, 0x72, 0x70, 0x68, 0x6f, 0x42, 0x6c, 0x75, 0x65, 0x44, 0x65, 0x62, 0x74, 0x48, 0x00, 0x52,
	0x13, 0x72, 0x65, 0x70, 0x61, 0x79, 0x4d, 0x6f, 0x72, 0x70, 0x68, 0x6f, 0x42, 0x6c, 0x75, 0x65,
	0x44, 0x65, 0x62, 0x74, 0x1a, 0x72, 0x0a, 0x14, 0x42, 0x6f, 0x72, 0x72, 0x6f, 0x77, 0x46, 0x72,
	0x6f, 0x6d, 0x4d, 0x6f, 0x72, 0x70, 0x68, 0x6f, 0x42, 0x6c, 0x75, 0x65, 0x12, 0x30, 0x0a, 0x06,
	0x6d, 0x61, 0x72, 0x6b, 0x65, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x18, 0x2e, 0x73,
	0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e, 0x76, 0x34, 0x2e, 0x4d, 0x61, 0x72, 0x6b, 0x65, 0x74,
	0x50, 0x61, 0x72, 0x61, 0x6d, 0x73, 0x52, 0x06, 0x6d, 0x61, 0x72, 0x6b, 0x65, 0x74, 0x12, 0x28,
	0x0a, 0x10, 0x61, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x5f, 0x74, 0x6f, 0x5f, 0x62, 0x6f, 0x72, 0x72,
	0x6f, 0x77, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0e, 0x61, 0x6d, 0x6f, 0x75, 0x6e, 0x74,
	0x54, 0x6f, 0x42, 0x6f, 0x72, 0x72, 0x6f, 0x77, 0x1a, 0x7e, 0x0a, 0x13, 0x52, 0x65, 0x70, 0x61,
	0x79, 0x4d, 0x6f, 0x72, 0x70, 0x68, 0x6f, 0x42, 0x6c, 0x75, 0x65, 0x44, 0x65, 0x62, 0x74, 0x12,
	0x30, 0x0a, 0x06, 0x6d, 0x61, 0x72, 0x6b, 0x65, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32,
	0x18, 0x2e, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e, 0x76, 0x34, 0x2e, 0x4d, 0x61, 0x72,
	0x6b, 0x65, 0x74, 0x50, 0x61, 0x72, 0x61, 0x6d, 0x73, 0x52, 0x06, 0x6d, 0x61, 0x72, 0x6b, 0x65,
	0x74, 0x12, 0x35, 0x0a, 0x17, 0x64, 0x65, 0x62, 0x74, 0x5f, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x5f,
	0x72, 0x65, 0x70, 0x61, 0x79, 0x5f, 0x61, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x02, 0x20, 0x01,
	0x28, 0x09, 0x52, 0x14, 0x64, 0x65, 0x62, 0x74, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x52, 0x65, 0x70,
	0x61, 0x79, 0x41, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x42, 0x0a, 0x0a, 0x08, 0x66, 0x75, 0x6e, 0x63,
	0x74, 0x69, 0x6f, 0x6e, 0x22, 0x59, 0x0a, 0x1c, 0x4d, 0x6f, 0x72, 0x70, 0x68, 0x6f, 0x42, 0x6c,
	0x75, 0x65, 0x44, 0x65, 0x62, 0x74, 0x41, 0x64, 0x61, 0x70, 0x74, 0x6f, 0x72, 0x56, 0x31, 0x43,
	0x61, 0x6c, 0x6c, 0x73, 0x12, 0x39, 0x0a, 0x05, 0x63, 0x61, 0x6c, 0x6c, 0x73, 0x18, 0x01, 0x20,
	0x03, 0x28, 0x0b, 0x32, 0x23, 0x2e, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e, 0x76, 0x34,
	0x2e, 0x4d, 0x6f, 0x72, 0x70, 0x68, 0x6f, 0x42, 0x6c, 0x75, 0x65, 0x44, 0x65, 0x62, 0x74, 0x41,
	0x64, 0x61, 0x70, 0x74, 0x6f, 0x72, 0x56, 0x31, 0x52, 0x05, 0x63, 0x61, 0x6c, 0x6c, 0x73, 0x42,
	0x10, 0x5a, 0x0e, 0x2f, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x5f, 0x70, 0x72, 0x6f, 0x74,
	0x6f, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
}

var (
	file_morpho_blue_debt_proto_rawDescOnce sync.Once
	file_morpho_blue_debt_proto_rawDescData = file_morpho_blue_debt_proto_rawDesc
)

func file_morpho_blue_debt_proto_rawDescGZIP() []byte {
	file_morpho_blue_debt_proto_rawDescOnce.Do(func() {
		file_morpho_blue_debt_proto_rawDescData = protoimpl.X.CompressGZIP(file_morpho_blue_debt_proto_rawDescData)
	})
	return file_morpho_blue_debt_proto_rawDescData
}

var file_morpho_blue_debt_proto_msgTypes = make([]protoimpl.MessageInfo, 4)
var file_morpho_blue_debt_proto_goTypes = []interface{}{
	(*MorphoBlueDebtAdaptorV1)(nil),                      // 0: steward.v4.MorphoBlueDebtAdaptorV1
	(*MorphoBlueDebtAdaptorV1Calls)(nil),                 // 1: steward.v4.MorphoBlueDebtAdaptorV1Calls
	(*MorphoBlueDebtAdaptorV1_BorrowFromMorphoBlue)(nil), // 2: steward.v4.MorphoBlueDebtAdaptorV1.BorrowFromMorphoBlue
	(*MorphoBlueDebtAdaptorV1_RepayMorphoBlueDebt)(nil),  // 3: steward.v4.MorphoBlueDebtAdaptorV1.RepayMorphoBlueDebt
	(*RevokeApproval)(nil),                               // 4: steward.v4.RevokeApproval
	(*MarketParams)(nil),                                 // 5: steward.v4.MarketParams
}
var file_morpho_blue_debt_proto_depIdxs = []int32{
	4, // 0: steward.v4.MorphoBlueDebtAdaptorV1.revoke_approval:type_name -> steward.v4.RevokeApproval
	2, // 1: steward.v4.MorphoBlueDebtAdaptorV1.borrow_from_morpho_blue:type_name -> steward.v4.MorphoBlueDebtAdaptorV1.BorrowFromMorphoBlue
	3, // 2: steward.v4.MorphoBlueDebtAdaptorV1.repay_morpho_blue_debt:type_name -> steward.v4.MorphoBlueDebtAdaptorV1.RepayMorphoBlueDebt
	0, // 3: steward.v4.MorphoBlueDebtAdaptorV1Calls.calls:type_name -> steward.v4.MorphoBlueDebtAdaptorV1
	5, // 4: steward.v4.MorphoBlueDebtAdaptorV1.BorrowFromMorphoBlue.market:type_name -> steward.v4.MarketParams
	5, // 5: steward.v4.MorphoBlueDebtAdaptorV1.RepayMorphoBlueDebt.market:type_name -> steward.v4.MarketParams
	6, // [6:6] is the sub-list for method output_type
	6, // [6:6] is the sub-list for method input_type
	6, // [6:6] is the sub-list for extension type_name
	6, // [6:6] is the sub-list for extension extendee
	0, // [0:6] is the sub-list for field type_name
}

func init() { file_morpho_blue_debt_proto_init() }
func file_morpho_blue_debt_proto_init() {
	if File_morpho_blue_debt_proto != nil {
		return
	}
	file_base_proto_init()
	file_common_proto_init()
	if !protoimpl.UnsafeEnabled {
		file_morpho_blue_debt_proto_msgTypes[0].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*MorphoBlueDebtAdaptorV1); i {
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
		file_morpho_blue_debt_proto_msgTypes[1].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*MorphoBlueDebtAdaptorV1Calls); i {
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
		file_morpho_blue_debt_proto_msgTypes[2].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*MorphoBlueDebtAdaptorV1_BorrowFromMorphoBlue); i {
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
		file_morpho_blue_debt_proto_msgTypes[3].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*MorphoBlueDebtAdaptorV1_RepayMorphoBlueDebt); i {
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
	file_morpho_blue_debt_proto_msgTypes[0].OneofWrappers = []interface{}{
		(*MorphoBlueDebtAdaptorV1_RevokeApproval)(nil),
		(*MorphoBlueDebtAdaptorV1_BorrowFromMorphoBlue_)(nil),
		(*MorphoBlueDebtAdaptorV1_RepayMorphoBlueDebt_)(nil),
	}
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: file_morpho_blue_debt_proto_rawDesc,
			NumEnums:      0,
			NumMessages:   4,
			NumExtensions: 0,
			NumServices:   0,
		},
		GoTypes:           file_morpho_blue_debt_proto_goTypes,
		DependencyIndexes: file_morpho_blue_debt_proto_depIdxs,
		MessageInfos:      file_morpho_blue_debt_proto_msgTypes,
	}.Build()
	File_morpho_blue_debt_proto = out.File
	file_morpho_blue_debt_proto_rawDesc = nil
	file_morpho_blue_debt_proto_goTypes = nil
	file_morpho_blue_debt_proto_depIdxs = nil
}
