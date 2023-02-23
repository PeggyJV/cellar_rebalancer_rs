//
// Protos for function calls to the simple Vesting adaptor.

// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.26.0
// 	protoc        v3.21.12
// source: vesting_simple.proto

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

// Represents call data for the Vesting Simple adaptor
type VestingSimpleAdaptor struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// Types that are assignable to Function:
	//	*VestingSimpleAdaptor_Swap
	//	*VestingSimpleAdaptor_OracleSwap
	//	*VestingSimpleAdaptor_RevokeApproval
	//	*VestingSimpleAdaptor_DepositToVesting_
	//	*VestingSimpleAdaptor_WithdrawFromVesting_
	//	*VestingSimpleAdaptor_WithdrawAnyFromVesting_
	//	*VestingSimpleAdaptor_WithdrawAllFromVesting_
	Function isVestingSimpleAdaptor_Function `protobuf_oneof:"function"`
}

func (x *VestingSimpleAdaptor) Reset() {
	*x = VestingSimpleAdaptor{}
	if protoimpl.UnsafeEnabled {
		mi := &file_vesting_simple_proto_msgTypes[0]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *VestingSimpleAdaptor) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*VestingSimpleAdaptor) ProtoMessage() {}

func (x *VestingSimpleAdaptor) ProtoReflect() protoreflect.Message {
	mi := &file_vesting_simple_proto_msgTypes[0]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use VestingSimpleAdaptor.ProtoReflect.Descriptor instead.
func (*VestingSimpleAdaptor) Descriptor() ([]byte, []int) {
	return file_vesting_simple_proto_rawDescGZIP(), []int{0}
}

func (m *VestingSimpleAdaptor) GetFunction() isVestingSimpleAdaptor_Function {
	if m != nil {
		return m.Function
	}
	return nil
}

func (x *VestingSimpleAdaptor) GetSwap() *Swap {
	if x, ok := x.GetFunction().(*VestingSimpleAdaptor_Swap); ok {
		return x.Swap
	}
	return nil
}

func (x *VestingSimpleAdaptor) GetOracleSwap() *OracleSwap {
	if x, ok := x.GetFunction().(*VestingSimpleAdaptor_OracleSwap); ok {
		return x.OracleSwap
	}
	return nil
}

func (x *VestingSimpleAdaptor) GetRevokeApproval() *RevokeApproval {
	if x, ok := x.GetFunction().(*VestingSimpleAdaptor_RevokeApproval); ok {
		return x.RevokeApproval
	}
	return nil
}

func (x *VestingSimpleAdaptor) GetDepositToVesting() *VestingSimpleAdaptor_DepositToVesting {
	if x, ok := x.GetFunction().(*VestingSimpleAdaptor_DepositToVesting_); ok {
		return x.DepositToVesting
	}
	return nil
}

func (x *VestingSimpleAdaptor) GetWithdrawFromVesting() *VestingSimpleAdaptor_WithdrawFromVesting {
	if x, ok := x.GetFunction().(*VestingSimpleAdaptor_WithdrawFromVesting_); ok {
		return x.WithdrawFromVesting
	}
	return nil
}

func (x *VestingSimpleAdaptor) GetWithdrawAnyFromVesting() *VestingSimpleAdaptor_WithdrawAnyFromVesting {
	if x, ok := x.GetFunction().(*VestingSimpleAdaptor_WithdrawAnyFromVesting_); ok {
		return x.WithdrawAnyFromVesting
	}
	return nil
}

func (x *VestingSimpleAdaptor) GetWithdrawAllFromVesting() *VestingSimpleAdaptor_WithdrawAllFromVesting {
	if x, ok := x.GetFunction().(*VestingSimpleAdaptor_WithdrawAllFromVesting_); ok {
		return x.WithdrawAllFromVesting
	}
	return nil
}

type isVestingSimpleAdaptor_Function interface {
	isVestingSimpleAdaptor_Function()
}

type VestingSimpleAdaptor_Swap struct {
	// Represents function `swap(ERC20 assetIn, ERC20 assetOut, uint256 amountIn, SwapRouter.Exchange exchange, bytes memory params)`
	Swap *Swap `protobuf:"bytes,1,opt,name=swap,proto3,oneof"`
}

type VestingSimpleAdaptor_OracleSwap struct {
	// Represents function `oracleSwap(ERC20 assetIn, ERC20 assetOut, uint256 amountIn, SwapRouter.Exchange exchange, bytes memory params, uint64 slippage)`
	OracleSwap *OracleSwap `protobuf:"bytes,2,opt,name=oracle_swap,json=oracleSwap,proto3,oneof"`
}

type VestingSimpleAdaptor_RevokeApproval struct {
	// Represents function `revokeApproval(ERC20 asset, address spender)`
	RevokeApproval *RevokeApproval `protobuf:"bytes,3,opt,name=revoke_approval,json=revokeApproval,proto3,oneof"`
}

type VestingSimpleAdaptor_DepositToVesting_ struct {
	// Represents function `depositToVesting(VestingSimple vestingContract, uint256 amountToDeposit)`
	DepositToVesting *VestingSimpleAdaptor_DepositToVesting `protobuf:"bytes,4,opt,name=deposit_to_vesting,json=depositToVesting,proto3,oneof"`
}

type VestingSimpleAdaptor_WithdrawFromVesting_ struct {
	// Represents function `withdrawFromVesting(VestingSimple vestingContract, uint256 depositId, uint256 amountToWithdraw)`
	WithdrawFromVesting *VestingSimpleAdaptor_WithdrawFromVesting `protobuf:"bytes,5,opt,name=withdraw_from_vesting,json=withdrawFromVesting,proto3,oneof"`
}

type VestingSimpleAdaptor_WithdrawAnyFromVesting_ struct {
	// Represents function `withdrawAnyFromVesting(VestingSimple vestingContract, uint256 amountToWithdraw)`
	WithdrawAnyFromVesting *VestingSimpleAdaptor_WithdrawAnyFromVesting `protobuf:"bytes,6,opt,name=withdraw_any_from_vesting,json=withdrawAnyFromVesting,proto3,oneof"`
}

type VestingSimpleAdaptor_WithdrawAllFromVesting_ struct {
	// Represents function `withdrawAllFromVesting(VestingSimple vestingContract)`
	WithdrawAllFromVesting *VestingSimpleAdaptor_WithdrawAllFromVesting `protobuf:"bytes,7,opt,name=withdraw_all_from_vesting,json=withdrawAllFromVesting,proto3,oneof"`
}

func (*VestingSimpleAdaptor_Swap) isVestingSimpleAdaptor_Function() {}

func (*VestingSimpleAdaptor_OracleSwap) isVestingSimpleAdaptor_Function() {}

func (*VestingSimpleAdaptor_RevokeApproval) isVestingSimpleAdaptor_Function() {}

func (*VestingSimpleAdaptor_DepositToVesting_) isVestingSimpleAdaptor_Function() {}

func (*VestingSimpleAdaptor_WithdrawFromVesting_) isVestingSimpleAdaptor_Function() {}

func (*VestingSimpleAdaptor_WithdrawAnyFromVesting_) isVestingSimpleAdaptor_Function() {}

func (*VestingSimpleAdaptor_WithdrawAllFromVesting_) isVestingSimpleAdaptor_Function() {}

type VestingSimpleAdaptorCalls struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Calls []*VestingSimpleAdaptor `protobuf:"bytes,1,rep,name=calls,proto3" json:"calls,omitempty"`
}

func (x *VestingSimpleAdaptorCalls) Reset() {
	*x = VestingSimpleAdaptorCalls{}
	if protoimpl.UnsafeEnabled {
		mi := &file_vesting_simple_proto_msgTypes[1]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *VestingSimpleAdaptorCalls) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*VestingSimpleAdaptorCalls) ProtoMessage() {}

func (x *VestingSimpleAdaptorCalls) ProtoReflect() protoreflect.Message {
	mi := &file_vesting_simple_proto_msgTypes[1]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use VestingSimpleAdaptorCalls.ProtoReflect.Descriptor instead.
func (*VestingSimpleAdaptorCalls) Descriptor() ([]byte, []int) {
	return file_vesting_simple_proto_rawDescGZIP(), []int{1}
}

func (x *VestingSimpleAdaptorCalls) GetCalls() []*VestingSimpleAdaptor {
	if x != nil {
		return x.Calls
	}
	return nil
}

//
// Allows strategists to deposit tokens to the vesting contract. By passing a max uint256 for amountToDeposit, the cellar will
// deposit its entire balance (appropriate in most cases).
//
// Represents function `depositToVesting(VestingSimple vestingContract, uint256 amountToDeposit)`
type VestingSimpleAdaptor_DepositToVesting struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	VestingContract string `protobuf:"bytes,1,opt,name=vesting_contract,json=vestingContract,proto3" json:"vesting_contract,omitempty"`
	Amount          string `protobuf:"bytes,2,opt,name=amount,proto3" json:"amount,omitempty"`
}

func (x *VestingSimpleAdaptor_DepositToVesting) Reset() {
	*x = VestingSimpleAdaptor_DepositToVesting{}
	if protoimpl.UnsafeEnabled {
		mi := &file_vesting_simple_proto_msgTypes[2]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *VestingSimpleAdaptor_DepositToVesting) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*VestingSimpleAdaptor_DepositToVesting) ProtoMessage() {}

func (x *VestingSimpleAdaptor_DepositToVesting) ProtoReflect() protoreflect.Message {
	mi := &file_vesting_simple_proto_msgTypes[2]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use VestingSimpleAdaptor_DepositToVesting.ProtoReflect.Descriptor instead.
func (*VestingSimpleAdaptor_DepositToVesting) Descriptor() ([]byte, []int) {
	return file_vesting_simple_proto_rawDescGZIP(), []int{0, 0}
}

func (x *VestingSimpleAdaptor_DepositToVesting) GetVestingContract() string {
	if x != nil {
		return x.VestingContract
	}
	return ""
}

func (x *VestingSimpleAdaptor_DepositToVesting) GetAmount() string {
	if x != nil {
		return x.Amount
	}
	return ""
}

//
// Allows strategists to deposit tokens to the vesting contract. By passing a max uint256 for amountToDeposit, the cellar will
// deposit its entire balance (appropriate in most cases).
//
// Represents function `withdrawFromVesting(VestingSimple vestingContract, uint256 depositId, uint256 amountToWithdraw)`
type VestingSimpleAdaptor_WithdrawFromVesting struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	VestingContract string `protobuf:"bytes,1,opt,name=vesting_contract,json=vestingContract,proto3" json:"vesting_contract,omitempty"`
	DepositId       string `protobuf:"bytes,2,opt,name=deposit_id,json=depositId,proto3" json:"deposit_id,omitempty"`
	Amount          string `protobuf:"bytes,3,opt,name=amount,proto3" json:"amount,omitempty"`
}

func (x *VestingSimpleAdaptor_WithdrawFromVesting) Reset() {
	*x = VestingSimpleAdaptor_WithdrawFromVesting{}
	if protoimpl.UnsafeEnabled {
		mi := &file_vesting_simple_proto_msgTypes[3]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *VestingSimpleAdaptor_WithdrawFromVesting) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*VestingSimpleAdaptor_WithdrawFromVesting) ProtoMessage() {}

func (x *VestingSimpleAdaptor_WithdrawFromVesting) ProtoReflect() protoreflect.Message {
	mi := &file_vesting_simple_proto_msgTypes[3]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use VestingSimpleAdaptor_WithdrawFromVesting.ProtoReflect.Descriptor instead.
func (*VestingSimpleAdaptor_WithdrawFromVesting) Descriptor() ([]byte, []int) {
	return file_vesting_simple_proto_rawDescGZIP(), []int{0, 1}
}

func (x *VestingSimpleAdaptor_WithdrawFromVesting) GetVestingContract() string {
	if x != nil {
		return x.VestingContract
	}
	return ""
}

func (x *VestingSimpleAdaptor_WithdrawFromVesting) GetDepositId() string {
	if x != nil {
		return x.DepositId
	}
	return ""
}

func (x *VestingSimpleAdaptor_WithdrawFromVesting) GetAmount() string {
	if x != nil {
		return x.Amount
	}
	return ""
}

//
// Withdraw a single deposit from vesting. This will not affect the cellar's TVL because any deposit must already have vested, and
// will be reported in balanceOf. Will revert if not enough tokens are available based on amountToWithdraw.
//
// Represents function `withdrawAnyFromVesting(VestingSimple vestingContract, uint256 amountToWithdraw)`
type VestingSimpleAdaptor_WithdrawAnyFromVesting struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	VestingContract string `protobuf:"bytes,1,opt,name=vesting_contract,json=vestingContract,proto3" json:"vesting_contract,omitempty"`
	Amount          string `protobuf:"bytes,2,opt,name=amount,proto3" json:"amount,omitempty"`
}

func (x *VestingSimpleAdaptor_WithdrawAnyFromVesting) Reset() {
	*x = VestingSimpleAdaptor_WithdrawAnyFromVesting{}
	if protoimpl.UnsafeEnabled {
		mi := &file_vesting_simple_proto_msgTypes[4]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *VestingSimpleAdaptor_WithdrawAnyFromVesting) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*VestingSimpleAdaptor_WithdrawAnyFromVesting) ProtoMessage() {}

func (x *VestingSimpleAdaptor_WithdrawAnyFromVesting) ProtoReflect() protoreflect.Message {
	mi := &file_vesting_simple_proto_msgTypes[4]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use VestingSimpleAdaptor_WithdrawAnyFromVesting.ProtoReflect.Descriptor instead.
func (*VestingSimpleAdaptor_WithdrawAnyFromVesting) Descriptor() ([]byte, []int) {
	return file_vesting_simple_proto_rawDescGZIP(), []int{0, 2}
}

func (x *VestingSimpleAdaptor_WithdrawAnyFromVesting) GetVestingContract() string {
	if x != nil {
		return x.VestingContract
	}
	return ""
}

func (x *VestingSimpleAdaptor_WithdrawAnyFromVesting) GetAmount() string {
	if x != nil {
		return x.Amount
	}
	return ""
}

//
// Withdraw a certain amount of tokens from vesting, from any deposit. This will not affect the cellar's TVL because any deposit must
// already have vested, and will be reported in balanceOf. Will revert if not enough tokens are available based on amountToWithdraw.
//
// Represents function `withdrawAllFromVesting(VestingSimple vestingContract)`
type VestingSimpleAdaptor_WithdrawAllFromVesting struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	VestingContract string `protobuf:"bytes,1,opt,name=vesting_contract,json=vestingContract,proto3" json:"vesting_contract,omitempty"`
}

func (x *VestingSimpleAdaptor_WithdrawAllFromVesting) Reset() {
	*x = VestingSimpleAdaptor_WithdrawAllFromVesting{}
	if protoimpl.UnsafeEnabled {
		mi := &file_vesting_simple_proto_msgTypes[5]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *VestingSimpleAdaptor_WithdrawAllFromVesting) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*VestingSimpleAdaptor_WithdrawAllFromVesting) ProtoMessage() {}

func (x *VestingSimpleAdaptor_WithdrawAllFromVesting) ProtoReflect() protoreflect.Message {
	mi := &file_vesting_simple_proto_msgTypes[5]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use VestingSimpleAdaptor_WithdrawAllFromVesting.ProtoReflect.Descriptor instead.
func (*VestingSimpleAdaptor_WithdrawAllFromVesting) Descriptor() ([]byte, []int) {
	return file_vesting_simple_proto_rawDescGZIP(), []int{0, 3}
}

func (x *VestingSimpleAdaptor_WithdrawAllFromVesting) GetVestingContract() string {
	if x != nil {
		return x.VestingContract
	}
	return ""
}

var File_vesting_simple_proto protoreflect.FileDescriptor

var file_vesting_simple_proto_rawDesc = []byte{
	0x0a, 0x14, 0x76, 0x65, 0x73, 0x74, 0x69, 0x6e, 0x67, 0x5f, 0x73, 0x69, 0x6d, 0x70, 0x6c, 0x65,
	0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0a, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e,
	0x76, 0x33, 0x1a, 0x0a, 0x62, 0x61, 0x73, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xf9,
	0x07, 0x0a, 0x14, 0x56, 0x65, 0x73, 0x74, 0x69, 0x6e, 0x67, 0x53, 0x69, 0x6d, 0x70, 0x6c, 0x65,
	0x41, 0x64, 0x61, 0x70, 0x74, 0x6f, 0x72, 0x12, 0x26, 0x0a, 0x04, 0x73, 0x77, 0x61, 0x70, 0x18,
	0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e,
	0x76, 0x33, 0x2e, 0x53, 0x77, 0x61, 0x70, 0x48, 0x00, 0x52, 0x04, 0x73, 0x77, 0x61, 0x70, 0x12,
	0x39, 0x0a, 0x0b, 0x6f, 0x72, 0x61, 0x63, 0x6c, 0x65, 0x5f, 0x73, 0x77, 0x61, 0x70, 0x18, 0x02,
	0x20, 0x01, 0x28, 0x0b, 0x32, 0x16, 0x2e, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e, 0x76,
	0x33, 0x2e, 0x4f, 0x72, 0x61, 0x63, 0x6c, 0x65, 0x53, 0x77, 0x61, 0x70, 0x48, 0x00, 0x52, 0x0a,
	0x6f, 0x72, 0x61, 0x63, 0x6c, 0x65, 0x53, 0x77, 0x61, 0x70, 0x12, 0x45, 0x0a, 0x0f, 0x72, 0x65,
	0x76, 0x6f, 0x6b, 0x65, 0x5f, 0x61, 0x70, 0x70, 0x72, 0x6f, 0x76, 0x61, 0x6c, 0x18, 0x03, 0x20,
	0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e, 0x76, 0x33,
	0x2e, 0x52, 0x65, 0x76, 0x6f, 0x6b, 0x65, 0x41, 0x70, 0x70, 0x72, 0x6f, 0x76, 0x61, 0x6c, 0x48,
	0x00, 0x52, 0x0e, 0x72, 0x65, 0x76, 0x6f, 0x6b, 0x65, 0x41, 0x70, 0x70, 0x72, 0x6f, 0x76, 0x61,
	0x6c, 0x12, 0x61, 0x0a, 0x12, 0x64, 0x65, 0x70, 0x6f, 0x73, 0x69, 0x74, 0x5f, 0x74, 0x6f, 0x5f,
	0x76, 0x65, 0x73, 0x74, 0x69, 0x6e, 0x67, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x31, 0x2e,
	0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e, 0x76, 0x33, 0x2e, 0x56, 0x65, 0x73, 0x74, 0x69,
	0x6e, 0x67, 0x53, 0x69, 0x6d, 0x70, 0x6c, 0x65, 0x41, 0x64, 0x61, 0x70, 0x74, 0x6f, 0x72, 0x2e,
	0x44, 0x65, 0x70, 0x6f, 0x73, 0x69, 0x74, 0x54, 0x6f, 0x56, 0x65, 0x73, 0x74, 0x69, 0x6e, 0x67,
	0x48, 0x00, 0x52, 0x10, 0x64, 0x65, 0x70, 0x6f, 0x73, 0x69, 0x74, 0x54, 0x6f, 0x56, 0x65, 0x73,
	0x74, 0x69, 0x6e, 0x67, 0x12, 0x6a, 0x0a, 0x15, 0x77, 0x69, 0x74, 0x68, 0x64, 0x72, 0x61, 0x77,
	0x5f, 0x66, 0x72, 0x6f, 0x6d, 0x5f, 0x76, 0x65, 0x73, 0x74, 0x69, 0x6e, 0x67, 0x18, 0x05, 0x20,
	0x01, 0x28, 0x0b, 0x32, 0x34, 0x2e, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e, 0x76, 0x33,
	0x2e, 0x56, 0x65, 0x73, 0x74, 0x69, 0x6e, 0x67, 0x53, 0x69, 0x6d, 0x70, 0x6c, 0x65, 0x41, 0x64,
	0x61, 0x70, 0x74, 0x6f, 0x72, 0x2e, 0x57, 0x69, 0x74, 0x68, 0x64, 0x72, 0x61, 0x77, 0x46, 0x72,
	0x6f, 0x6d, 0x56, 0x65, 0x73, 0x74, 0x69, 0x6e, 0x67, 0x48, 0x00, 0x52, 0x13, 0x77, 0x69, 0x74,
	0x68, 0x64, 0x72, 0x61, 0x77, 0x46, 0x72, 0x6f, 0x6d, 0x56, 0x65, 0x73, 0x74, 0x69, 0x6e, 0x67,
	0x12, 0x74, 0x0a, 0x19, 0x77, 0x69, 0x74, 0x68, 0x64, 0x72, 0x61, 0x77, 0x5f, 0x61, 0x6e, 0x79,
	0x5f, 0x66, 0x72, 0x6f, 0x6d, 0x5f, 0x76, 0x65, 0x73, 0x74, 0x69, 0x6e, 0x67, 0x18, 0x06, 0x20,
	0x01, 0x28, 0x0b, 0x32, 0x37, 0x2e, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e, 0x76, 0x33,
	0x2e, 0x56, 0x65, 0x73, 0x74, 0x69, 0x6e, 0x67, 0x53, 0x69, 0x6d, 0x70, 0x6c, 0x65, 0x41, 0x64,
	0x61, 0x70, 0x74, 0x6f, 0x72, 0x2e, 0x57, 0x69, 0x74, 0x68, 0x64, 0x72, 0x61, 0x77, 0x41, 0x6e,
	0x79, 0x46, 0x72, 0x6f, 0x6d, 0x56, 0x65, 0x73, 0x74, 0x69, 0x6e, 0x67, 0x48, 0x00, 0x52, 0x16,
	0x77, 0x69, 0x74, 0x68, 0x64, 0x72, 0x61, 0x77, 0x41, 0x6e, 0x79, 0x46, 0x72, 0x6f, 0x6d, 0x56,
	0x65, 0x73, 0x74, 0x69, 0x6e, 0x67, 0x12, 0x74, 0x0a, 0x19, 0x77, 0x69, 0x74, 0x68, 0x64, 0x72,
	0x61, 0x77, 0x5f, 0x61, 0x6c, 0x6c, 0x5f, 0x66, 0x72, 0x6f, 0x6d, 0x5f, 0x76, 0x65, 0x73, 0x74,
	0x69, 0x6e, 0x67, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x37, 0x2e, 0x73, 0x74, 0x65, 0x77,
	0x61, 0x72, 0x64, 0x2e, 0x76, 0x33, 0x2e, 0x56, 0x65, 0x73, 0x74, 0x69, 0x6e, 0x67, 0x53, 0x69,
	0x6d, 0x70, 0x6c, 0x65, 0x41, 0x64, 0x61, 0x70, 0x74, 0x6f, 0x72, 0x2e, 0x57, 0x69, 0x74, 0x68,
	0x64, 0x72, 0x61, 0x77, 0x41, 0x6c, 0x6c, 0x46, 0x72, 0x6f, 0x6d, 0x56, 0x65, 0x73, 0x74, 0x69,
	0x6e, 0x67, 0x48, 0x00, 0x52, 0x16, 0x77, 0x69, 0x74, 0x68, 0x64, 0x72, 0x61, 0x77, 0x41, 0x6c,
	0x6c, 0x46, 0x72, 0x6f, 0x6d, 0x56, 0x65, 0x73, 0x74, 0x69, 0x6e, 0x67, 0x1a, 0x55, 0x0a, 0x10,
	0x44, 0x65, 0x70, 0x6f, 0x73, 0x69, 0x74, 0x54, 0x6f, 0x56, 0x65, 0x73, 0x74, 0x69, 0x6e, 0x67,
	0x12, 0x29, 0x0a, 0x10, 0x76, 0x65, 0x73, 0x74, 0x69, 0x6e, 0x67, 0x5f, 0x63, 0x6f, 0x6e, 0x74,
	0x72, 0x61, 0x63, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0f, 0x76, 0x65, 0x73, 0x74,
	0x69, 0x6e, 0x67, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63, 0x74, 0x12, 0x16, 0x0a, 0x06, 0x61,
	0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x61, 0x6d, 0x6f,
	0x75, 0x6e, 0x74, 0x1a, 0x77, 0x0a, 0x13, 0x57, 0x69, 0x74, 0x68, 0x64, 0x72, 0x61, 0x77, 0x46,
	0x72, 0x6f, 0x6d, 0x56, 0x65, 0x73, 0x74, 0x69, 0x6e, 0x67, 0x12, 0x29, 0x0a, 0x10, 0x76, 0x65,
	0x73, 0x74, 0x69, 0x6e, 0x67, 0x5f, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63, 0x74, 0x18, 0x01,
	0x20, 0x01, 0x28, 0x09, 0x52, 0x0f, 0x76, 0x65, 0x73, 0x74, 0x69, 0x6e, 0x67, 0x43, 0x6f, 0x6e,
	0x74, 0x72, 0x61, 0x63, 0x74, 0x12, 0x1d, 0x0a, 0x0a, 0x64, 0x65, 0x70, 0x6f, 0x73, 0x69, 0x74,
	0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x09, 0x64, 0x65, 0x70, 0x6f, 0x73,
	0x69, 0x74, 0x49, 0x64, 0x12, 0x16, 0x0a, 0x06, 0x61, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x03,
	0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x61, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x1a, 0x5b, 0x0a, 0x16,
	0x57, 0x69, 0x74, 0x68, 0x64, 0x72, 0x61, 0x77, 0x41, 0x6e, 0x79, 0x46, 0x72, 0x6f, 0x6d, 0x56,
	0x65, 0x73, 0x74, 0x69, 0x6e, 0x67, 0x12, 0x29, 0x0a, 0x10, 0x76, 0x65, 0x73, 0x74, 0x69, 0x6e,
	0x67, 0x5f, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09,
	0x52, 0x0f, 0x76, 0x65, 0x73, 0x74, 0x69, 0x6e, 0x67, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63,
	0x74, 0x12, 0x16, 0x0a, 0x06, 0x61, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28,
	0x09, 0x52, 0x06, 0x61, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x1a, 0x43, 0x0a, 0x16, 0x57, 0x69, 0x74,
	0x68, 0x64, 0x72, 0x61, 0x77, 0x41, 0x6c, 0x6c, 0x46, 0x72, 0x6f, 0x6d, 0x56, 0x65, 0x73, 0x74,
	0x69, 0x6e, 0x67, 0x12, 0x29, 0x0a, 0x10, 0x76, 0x65, 0x73, 0x74, 0x69, 0x6e, 0x67, 0x5f, 0x63,
	0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0f, 0x76,
	0x65, 0x73, 0x74, 0x69, 0x6e, 0x67, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63, 0x74, 0x42, 0x0a,
	0x0a, 0x08, 0x66, 0x75, 0x6e, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x22, 0x53, 0x0a, 0x19, 0x56, 0x65,
	0x73, 0x74, 0x69, 0x6e, 0x67, 0x53, 0x69, 0x6d, 0x70, 0x6c, 0x65, 0x41, 0x64, 0x61, 0x70, 0x74,
	0x6f, 0x72, 0x43, 0x61, 0x6c, 0x6c, 0x73, 0x12, 0x36, 0x0a, 0x05, 0x63, 0x61, 0x6c, 0x6c, 0x73,
	0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x20, 0x2e, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64,
	0x2e, 0x76, 0x33, 0x2e, 0x56, 0x65, 0x73, 0x74, 0x69, 0x6e, 0x67, 0x53, 0x69, 0x6d, 0x70, 0x6c,
	0x65, 0x41, 0x64, 0x61, 0x70, 0x74, 0x6f, 0x72, 0x52, 0x05, 0x63, 0x61, 0x6c, 0x6c, 0x73, 0x42,
	0x10, 0x5a, 0x0e, 0x2f, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x5f, 0x70, 0x72, 0x6f, 0x74,
	0x6f, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
}

var (
	file_vesting_simple_proto_rawDescOnce sync.Once
	file_vesting_simple_proto_rawDescData = file_vesting_simple_proto_rawDesc
)

func file_vesting_simple_proto_rawDescGZIP() []byte {
	file_vesting_simple_proto_rawDescOnce.Do(func() {
		file_vesting_simple_proto_rawDescData = protoimpl.X.CompressGZIP(file_vesting_simple_proto_rawDescData)
	})
	return file_vesting_simple_proto_rawDescData
}

var file_vesting_simple_proto_msgTypes = make([]protoimpl.MessageInfo, 6)
var file_vesting_simple_proto_goTypes = []interface{}{
	(*VestingSimpleAdaptor)(nil),                        // 0: steward.v3.VestingSimpleAdaptor
	(*VestingSimpleAdaptorCalls)(nil),                   // 1: steward.v3.VestingSimpleAdaptorCalls
	(*VestingSimpleAdaptor_DepositToVesting)(nil),       // 2: steward.v3.VestingSimpleAdaptor.DepositToVesting
	(*VestingSimpleAdaptor_WithdrawFromVesting)(nil),    // 3: steward.v3.VestingSimpleAdaptor.WithdrawFromVesting
	(*VestingSimpleAdaptor_WithdrawAnyFromVesting)(nil), // 4: steward.v3.VestingSimpleAdaptor.WithdrawAnyFromVesting
	(*VestingSimpleAdaptor_WithdrawAllFromVesting)(nil), // 5: steward.v3.VestingSimpleAdaptor.WithdrawAllFromVesting
	(*Swap)(nil),           // 6: steward.v3.Swap
	(*OracleSwap)(nil),     // 7: steward.v3.OracleSwap
	(*RevokeApproval)(nil), // 8: steward.v3.RevokeApproval
}
var file_vesting_simple_proto_depIdxs = []int32{
	6, // 0: steward.v3.VestingSimpleAdaptor.swap:type_name -> steward.v3.Swap
	7, // 1: steward.v3.VestingSimpleAdaptor.oracle_swap:type_name -> steward.v3.OracleSwap
	8, // 2: steward.v3.VestingSimpleAdaptor.revoke_approval:type_name -> steward.v3.RevokeApproval
	2, // 3: steward.v3.VestingSimpleAdaptor.deposit_to_vesting:type_name -> steward.v3.VestingSimpleAdaptor.DepositToVesting
	3, // 4: steward.v3.VestingSimpleAdaptor.withdraw_from_vesting:type_name -> steward.v3.VestingSimpleAdaptor.WithdrawFromVesting
	4, // 5: steward.v3.VestingSimpleAdaptor.withdraw_any_from_vesting:type_name -> steward.v3.VestingSimpleAdaptor.WithdrawAnyFromVesting
	5, // 6: steward.v3.VestingSimpleAdaptor.withdraw_all_from_vesting:type_name -> steward.v3.VestingSimpleAdaptor.WithdrawAllFromVesting
	0, // 7: steward.v3.VestingSimpleAdaptorCalls.calls:type_name -> steward.v3.VestingSimpleAdaptor
	8, // [8:8] is the sub-list for method output_type
	8, // [8:8] is the sub-list for method input_type
	8, // [8:8] is the sub-list for extension type_name
	8, // [8:8] is the sub-list for extension extendee
	0, // [0:8] is the sub-list for field type_name
}

func init() { file_vesting_simple_proto_init() }
func file_vesting_simple_proto_init() {
	if File_vesting_simple_proto != nil {
		return
	}
	file_base_proto_init()
	if !protoimpl.UnsafeEnabled {
		file_vesting_simple_proto_msgTypes[0].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*VestingSimpleAdaptor); i {
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
		file_vesting_simple_proto_msgTypes[1].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*VestingSimpleAdaptorCalls); i {
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
		file_vesting_simple_proto_msgTypes[2].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*VestingSimpleAdaptor_DepositToVesting); i {
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
		file_vesting_simple_proto_msgTypes[3].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*VestingSimpleAdaptor_WithdrawFromVesting); i {
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
		file_vesting_simple_proto_msgTypes[4].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*VestingSimpleAdaptor_WithdrawAnyFromVesting); i {
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
		file_vesting_simple_proto_msgTypes[5].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*VestingSimpleAdaptor_WithdrawAllFromVesting); i {
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
	file_vesting_simple_proto_msgTypes[0].OneofWrappers = []interface{}{
		(*VestingSimpleAdaptor_Swap)(nil),
		(*VestingSimpleAdaptor_OracleSwap)(nil),
		(*VestingSimpleAdaptor_RevokeApproval)(nil),
		(*VestingSimpleAdaptor_DepositToVesting_)(nil),
		(*VestingSimpleAdaptor_WithdrawFromVesting_)(nil),
		(*VestingSimpleAdaptor_WithdrawAnyFromVesting_)(nil),
		(*VestingSimpleAdaptor_WithdrawAllFromVesting_)(nil),
	}
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: file_vesting_simple_proto_rawDesc,
			NumEnums:      0,
			NumMessages:   6,
			NumExtensions: 0,
			NumServices:   0,
		},
		GoTypes:           file_vesting_simple_proto_goTypes,
		DependencyIndexes: file_vesting_simple_proto_depIdxs,
		MessageInfos:      file_vesting_simple_proto_msgTypes,
	}.Build()
	File_vesting_simple_proto = out.File
	file_vesting_simple_proto_rawDesc = nil
	file_vesting_simple_proto_goTypes = nil
	file_vesting_simple_proto_depIdxs = nil
}
