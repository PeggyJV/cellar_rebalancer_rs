// Code generated - DO NOT EDIT.
// This file is a generated binding and any manual changes will be lost.

package integration_tests

import (
	"errors"
	"math/big"
	"strings"

	ethereum "github.com/ethereum/go-ethereum"
	"github.com/ethereum/go-ethereum/accounts/abi"
	"github.com/ethereum/go-ethereum/accounts/abi/bind"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/event"
)

// Reference imports to suppress errors if they are not otherwise used.
var (
	_ = errors.New
	_ = big.NewInt
	_ = strings.NewReader
	_ = ethereum.NotFound
	_ = bind.Bind
	_ = common.Big1
	_ = types.BloomLookup
	_ = event.NewSubscription
)

// ManagerWithMerkleVerificationMetaData contains all meta data concerning the ManagerWithMerkleVerification contract.
var ManagerWithMerkleVerificationMetaData = &bind.MetaData{
	ABI: "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_owner\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"_vault\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"_balancerVault\",\"type\":\"address\"}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\"},{\"inputs\":[],\"name\":\"ManagerWithMerkleVerification__BadFlashLoanIntentHash\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"target\",\"type\":\"address\"},{\"internalType\":\"bytes\",\"name\":\"targetData\",\"type\":\"bytes\"},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\"}],\"name\":\"ManagerWithMerkleVerification__FailedToVerifyManageProof\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"ManagerWithMerkleVerification__FlashLoanNotExecuted\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"ManagerWithMerkleVerification__FlashLoanNotInProgress\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"ManagerWithMerkleVerification__InvalidDecodersAndSanitizersLength\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"ManagerWithMerkleVerification__InvalidManageProofLength\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"ManagerWithMerkleVerification__InvalidTargetDataLength\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"ManagerWithMerkleVerification__InvalidValuesLength\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"ManagerWithMerkleVerification__OnlyCallableByBalancerVault\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"ManagerWithMerkleVerification__OnlyCallableByBoringVault\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"ManagerWithMerkleVerification__Paused\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"ManagerWithMerkleVerification__TotalSupplyMustRemainConstantDuringPlatform\",\"type\":\"error\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"callsMade\",\"type\":\"uint256\"}],\"name\":\"BoringVaultManaged\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"strategist\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"bytes32\",\"name\":\"oldRoot\",\"type\":\"bytes32\"},{\"indexed\":false,\"internalType\":\"bytes32\",\"name\":\"newRoot\",\"type\":\"bytes32\"}],\"name\":\"ManageRootUpdated\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\"}],\"name\":\"OwnerUpdated\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[],\"name\":\"Paused\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[],\"name\":\"Unpaused\",\"type\":\"event\"},{\"inputs\":[],\"name\":\"isPaused\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"name\":\"manageRoot\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"pause\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"strategist\",\"type\":\"address\"},{\"internalType\":\"bytes32\",\"name\":\"_manageRoot\",\"type\":\"bytes32\"}],\"name\":\"setManageRoot\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\"}],\"name\":\"setOwner\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"unpause\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"vault\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"}]",
}

// ManagerWithMerkleVerificationABI is the input ABI used to generate the binding from.
// Deprecated: Use ManagerWithMerkleVerificationMetaData.ABI instead.
var ManagerWithMerkleVerificationABI = ManagerWithMerkleVerificationMetaData.ABI

// ManagerWithMerkleVerification is an auto generated Go binding around an Ethereum contract.
type ManagerWithMerkleVerification struct {
	ManagerWithMerkleVerificationCaller     // Read-only binding to the contract
	ManagerWithMerkleVerificationTransactor // Write-only binding to the contract
	ManagerWithMerkleVerificationFilterer   // Log filterer for contract events
}

// ManagerWithMerkleVerificationCaller is an auto generated read-only Go binding around an Ethereum contract.
type ManagerWithMerkleVerificationCaller struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// ManagerWithMerkleVerificationTransactor is an auto generated write-only Go binding around an Ethereum contract.
type ManagerWithMerkleVerificationTransactor struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// ManagerWithMerkleVerificationFilterer is an auto generated log filtering Go binding around an Ethereum contract events.
type ManagerWithMerkleVerificationFilterer struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// ManagerWithMerkleVerificationSession is an auto generated Go binding around an Ethereum contract,
// with pre-set call and transact options.
type ManagerWithMerkleVerificationSession struct {
	Contract     *ManagerWithMerkleVerification // Generic contract binding to set the session for
	CallOpts     bind.CallOpts                  // Call options to use throughout this session
	TransactOpts bind.TransactOpts              // Transaction auth options to use throughout this session
}

// ManagerWithMerkleVerificationCallerSession is an auto generated read-only Go binding around an Ethereum contract,
// with pre-set call options.
type ManagerWithMerkleVerificationCallerSession struct {
	Contract *ManagerWithMerkleVerificationCaller // Generic contract caller binding to set the session for
	CallOpts bind.CallOpts                        // Call options to use throughout this session
}

// ManagerWithMerkleVerificationTransactorSession is an auto generated write-only Go binding around an Ethereum contract,
// with pre-set transact options.
type ManagerWithMerkleVerificationTransactorSession struct {
	Contract     *ManagerWithMerkleVerificationTransactor // Generic contract transactor binding to set the session for
	TransactOpts bind.TransactOpts                        // Transaction auth options to use throughout this session
}

// ManagerWithMerkleVerificationRaw is an auto generated low-level Go binding around an Ethereum contract.
type ManagerWithMerkleVerificationRaw struct {
	Contract *ManagerWithMerkleVerification // Generic contract binding to access the raw methods on
}

// ManagerWithMerkleVerificationCallerRaw is an auto generated low-level read-only Go binding around an Ethereum contract.
type ManagerWithMerkleVerificationCallerRaw struct {
	Contract *ManagerWithMerkleVerificationCaller // Generic read-only contract binding to access the raw methods on
}

// ManagerWithMerkleVerificationTransactorRaw is an auto generated low-level write-only Go binding around an Ethereum contract.
type ManagerWithMerkleVerificationTransactorRaw struct {
	Contract *ManagerWithMerkleVerificationTransactor // Generic write-only contract binding to access the raw methods on
}

// NewManagerWithMerkleVerification creates a new instance of ManagerWithMerkleVerification, bound to a specific deployed contract.
func NewManagerWithMerkleVerification(address common.Address, backend bind.ContractBackend) (*ManagerWithMerkleVerification, error) {
	contract, err := bindManagerWithMerkleVerification(address, backend, backend, backend)
	if err != nil {
		return nil, err
	}
	return &ManagerWithMerkleVerification{ManagerWithMerkleVerificationCaller: ManagerWithMerkleVerificationCaller{contract: contract}, ManagerWithMerkleVerificationTransactor: ManagerWithMerkleVerificationTransactor{contract: contract}, ManagerWithMerkleVerificationFilterer: ManagerWithMerkleVerificationFilterer{contract: contract}}, nil
}

// NewManagerWithMerkleVerificationCaller creates a new read-only instance of ManagerWithMerkleVerification, bound to a specific deployed contract.
func NewManagerWithMerkleVerificationCaller(address common.Address, caller bind.ContractCaller) (*ManagerWithMerkleVerificationCaller, error) {
	contract, err := bindManagerWithMerkleVerification(address, caller, nil, nil)
	if err != nil {
		return nil, err
	}
	return &ManagerWithMerkleVerificationCaller{contract: contract}, nil
}

// NewManagerWithMerkleVerificationTransactor creates a new write-only instance of ManagerWithMerkleVerification, bound to a specific deployed contract.
func NewManagerWithMerkleVerificationTransactor(address common.Address, transactor bind.ContractTransactor) (*ManagerWithMerkleVerificationTransactor, error) {
	contract, err := bindManagerWithMerkleVerification(address, nil, transactor, nil)
	if err != nil {
		return nil, err
	}
	return &ManagerWithMerkleVerificationTransactor{contract: contract}, nil
}

// NewManagerWithMerkleVerificationFilterer creates a new log filterer instance of ManagerWithMerkleVerification, bound to a specific deployed contract.
func NewManagerWithMerkleVerificationFilterer(address common.Address, filterer bind.ContractFilterer) (*ManagerWithMerkleVerificationFilterer, error) {
	contract, err := bindManagerWithMerkleVerification(address, nil, nil, filterer)
	if err != nil {
		return nil, err
	}
	return &ManagerWithMerkleVerificationFilterer{contract: contract}, nil
}

// bindManagerWithMerkleVerification binds a generic wrapper to an already deployed contract.
func bindManagerWithMerkleVerification(address common.Address, caller bind.ContractCaller, transactor bind.ContractTransactor, filterer bind.ContractFilterer) (*bind.BoundContract, error) {
	parsed, err := abi.JSON(strings.NewReader(ManagerWithMerkleVerificationABI))
	if err != nil {
		return nil, err
	}
	return bind.NewBoundContract(address, parsed, caller, transactor, filterer), nil
}

// Call invokes the (constant) contract method with params as input values and
// sets the output to result. The result type might be a single field for simple
// returns, a slice of interfaces for anonymous returns and a struct for named
// returns.
func (_ManagerWithMerkleVerification *ManagerWithMerkleVerificationRaw) Call(opts *bind.CallOpts, result *[]interface{}, method string, params ...interface{}) error {
	return _ManagerWithMerkleVerification.Contract.ManagerWithMerkleVerificationCaller.contract.Call(opts, result, method, params...)
}

// Transfer initiates a plain transaction to move funds to the contract, calling
// its default method if one is available.
func (_ManagerWithMerkleVerification *ManagerWithMerkleVerificationRaw) Transfer(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _ManagerWithMerkleVerification.Contract.ManagerWithMerkleVerificationTransactor.contract.Transfer(opts)
}

// Transact invokes the (paid) contract method with params as input values.
func (_ManagerWithMerkleVerification *ManagerWithMerkleVerificationRaw) Transact(opts *bind.TransactOpts, method string, params ...interface{}) (*types.Transaction, error) {
	return _ManagerWithMerkleVerification.Contract.ManagerWithMerkleVerificationTransactor.contract.Transact(opts, method, params...)
}

// Call invokes the (constant) contract method with params as input values and
// sets the output to result. The result type might be a single field for simple
// returns, a slice of interfaces for anonymous returns and a struct for named
// returns.
func (_ManagerWithMerkleVerification *ManagerWithMerkleVerificationCallerRaw) Call(opts *bind.CallOpts, result *[]interface{}, method string, params ...interface{}) error {
	return _ManagerWithMerkleVerification.Contract.contract.Call(opts, result, method, params...)
}

// Transfer initiates a plain transaction to move funds to the contract, calling
// its default method if one is available.
func (_ManagerWithMerkleVerification *ManagerWithMerkleVerificationTransactorRaw) Transfer(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _ManagerWithMerkleVerification.Contract.contract.Transfer(opts)
}

// Transact invokes the (paid) contract method with params as input values.
func (_ManagerWithMerkleVerification *ManagerWithMerkleVerificationTransactorRaw) Transact(opts *bind.TransactOpts, method string, params ...interface{}) (*types.Transaction, error) {
	return _ManagerWithMerkleVerification.Contract.contract.Transact(opts, method, params...)
}

// IsPaused is a free data retrieval call binding the contract method 0xb187bd26.
//
// Solidity: function isPaused() view returns(bool)
func (_ManagerWithMerkleVerification *ManagerWithMerkleVerificationCaller) IsPaused(opts *bind.CallOpts) (bool, error) {
	var out []interface{}
	err := _ManagerWithMerkleVerification.contract.Call(opts, &out, "isPaused")

	if err != nil {
		return *new(bool), err
	}

	out0 := *abi.ConvertType(out[0], new(bool)).(*bool)

	return out0, err

}

// IsPaused is a free data retrieval call binding the contract method 0xb187bd26.
//
// Solidity: function isPaused() view returns(bool)
func (_ManagerWithMerkleVerification *ManagerWithMerkleVerificationSession) IsPaused() (bool, error) {
	return _ManagerWithMerkleVerification.Contract.IsPaused(&_ManagerWithMerkleVerification.CallOpts)
}

// IsPaused is a free data retrieval call binding the contract method 0xb187bd26.
//
// Solidity: function isPaused() view returns(bool)
func (_ManagerWithMerkleVerification *ManagerWithMerkleVerificationCallerSession) IsPaused() (bool, error) {
	return _ManagerWithMerkleVerification.Contract.IsPaused(&_ManagerWithMerkleVerification.CallOpts)
}

// ManageRoot is a free data retrieval call binding the contract method 0x5ca58a99.
//
// Solidity: function manageRoot(address ) view returns(bytes32)
func (_ManagerWithMerkleVerification *ManagerWithMerkleVerificationCaller) ManageRoot(opts *bind.CallOpts, arg0 common.Address) ([32]byte, error) {
	var out []interface{}
	err := _ManagerWithMerkleVerification.contract.Call(opts, &out, "manageRoot", arg0)

	if err != nil {
		return *new([32]byte), err
	}

	out0 := *abi.ConvertType(out[0], new([32]byte)).(*[32]byte)

	return out0, err

}

// ManageRoot is a free data retrieval call binding the contract method 0x5ca58a99.
//
// Solidity: function manageRoot(address ) view returns(bytes32)
func (_ManagerWithMerkleVerification *ManagerWithMerkleVerificationSession) ManageRoot(arg0 common.Address) ([32]byte, error) {
	return _ManagerWithMerkleVerification.Contract.ManageRoot(&_ManagerWithMerkleVerification.CallOpts, arg0)
}

// ManageRoot is a free data retrieval call binding the contract method 0x5ca58a99.
//
// Solidity: function manageRoot(address ) view returns(bytes32)
func (_ManagerWithMerkleVerification *ManagerWithMerkleVerificationCallerSession) ManageRoot(arg0 common.Address) ([32]byte, error) {
	return _ManagerWithMerkleVerification.Contract.ManageRoot(&_ManagerWithMerkleVerification.CallOpts, arg0)
}

// Owner is a free data retrieval call binding the contract method 0x8da5cb5b.
//
// Solidity: function owner() view returns(address)
func (_ManagerWithMerkleVerification *ManagerWithMerkleVerificationCaller) Owner(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _ManagerWithMerkleVerification.contract.Call(opts, &out, "owner")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// Owner is a free data retrieval call binding the contract method 0x8da5cb5b.
//
// Solidity: function owner() view returns(address)
func (_ManagerWithMerkleVerification *ManagerWithMerkleVerificationSession) Owner() (common.Address, error) {
	return _ManagerWithMerkleVerification.Contract.Owner(&_ManagerWithMerkleVerification.CallOpts)
}

// Owner is a free data retrieval call binding the contract method 0x8da5cb5b.
//
// Solidity: function owner() view returns(address)
func (_ManagerWithMerkleVerification *ManagerWithMerkleVerificationCallerSession) Owner() (common.Address, error) {
	return _ManagerWithMerkleVerification.Contract.Owner(&_ManagerWithMerkleVerification.CallOpts)
}

// Vault is a free data retrieval call binding the contract method 0xfbfa77cf.
//
// Solidity: function vault() view returns(address)
func (_ManagerWithMerkleVerification *ManagerWithMerkleVerificationCaller) Vault(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _ManagerWithMerkleVerification.contract.Call(opts, &out, "vault")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// Vault is a free data retrieval call binding the contract method 0xfbfa77cf.
//
// Solidity: function vault() view returns(address)
func (_ManagerWithMerkleVerification *ManagerWithMerkleVerificationSession) Vault() (common.Address, error) {
	return _ManagerWithMerkleVerification.Contract.Vault(&_ManagerWithMerkleVerification.CallOpts)
}

// Vault is a free data retrieval call binding the contract method 0xfbfa77cf.
//
// Solidity: function vault() view returns(address)
func (_ManagerWithMerkleVerification *ManagerWithMerkleVerificationCallerSession) Vault() (common.Address, error) {
	return _ManagerWithMerkleVerification.Contract.Vault(&_ManagerWithMerkleVerification.CallOpts)
}

// Pause is a paid mutator transaction binding the contract method 0x8456cb59.
//
// Solidity: function pause() returns()
func (_ManagerWithMerkleVerification *ManagerWithMerkleVerificationTransactor) Pause(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _ManagerWithMerkleVerification.contract.Transact(opts, "pause")
}

// Pause is a paid mutator transaction binding the contract method 0x8456cb59.
//
// Solidity: function pause() returns()
func (_ManagerWithMerkleVerification *ManagerWithMerkleVerificationSession) Pause() (*types.Transaction, error) {
	return _ManagerWithMerkleVerification.Contract.Pause(&_ManagerWithMerkleVerification.TransactOpts)
}

// Pause is a paid mutator transaction binding the contract method 0x8456cb59.
//
// Solidity: function pause() returns()
func (_ManagerWithMerkleVerification *ManagerWithMerkleVerificationTransactorSession) Pause() (*types.Transaction, error) {
	return _ManagerWithMerkleVerification.Contract.Pause(&_ManagerWithMerkleVerification.TransactOpts)
}

// SetManageRoot is a paid mutator transaction binding the contract method 0x21801a99.
//
// Solidity: function setManageRoot(address strategist, bytes32 _manageRoot) returns()
func (_ManagerWithMerkleVerification *ManagerWithMerkleVerificationTransactor) SetManageRoot(opts *bind.TransactOpts, strategist common.Address, _manageRoot [32]byte) (*types.Transaction, error) {
	return _ManagerWithMerkleVerification.contract.Transact(opts, "setManageRoot", strategist, _manageRoot)
}

// SetManageRoot is a paid mutator transaction binding the contract method 0x21801a99.
//
// Solidity: function setManageRoot(address strategist, bytes32 _manageRoot) returns()
func (_ManagerWithMerkleVerification *ManagerWithMerkleVerificationSession) SetManageRoot(strategist common.Address, _manageRoot [32]byte) (*types.Transaction, error) {
	return _ManagerWithMerkleVerification.Contract.SetManageRoot(&_ManagerWithMerkleVerification.TransactOpts, strategist, _manageRoot)
}

// SetManageRoot is a paid mutator transaction binding the contract method 0x21801a99.
//
// Solidity: function setManageRoot(address strategist, bytes32 _manageRoot) returns()
func (_ManagerWithMerkleVerification *ManagerWithMerkleVerificationTransactorSession) SetManageRoot(strategist common.Address, _manageRoot [32]byte) (*types.Transaction, error) {
	return _ManagerWithMerkleVerification.Contract.SetManageRoot(&_ManagerWithMerkleVerification.TransactOpts, strategist, _manageRoot)
}

// SetOwner is a paid mutator transaction binding the contract method 0x13af4035.
//
// Solidity: function setOwner(address newOwner) returns()
func (_ManagerWithMerkleVerification *ManagerWithMerkleVerificationTransactor) SetOwner(opts *bind.TransactOpts, newOwner common.Address) (*types.Transaction, error) {
	return _ManagerWithMerkleVerification.contract.Transact(opts, "setOwner", newOwner)
}

// SetOwner is a paid mutator transaction binding the contract method 0x13af4035.
//
// Solidity: function setOwner(address newOwner) returns()
func (_ManagerWithMerkleVerification *ManagerWithMerkleVerificationSession) SetOwner(newOwner common.Address) (*types.Transaction, error) {
	return _ManagerWithMerkleVerification.Contract.SetOwner(&_ManagerWithMerkleVerification.TransactOpts, newOwner)
}

// SetOwner is a paid mutator transaction binding the contract method 0x13af4035.
//
// Solidity: function setOwner(address newOwner) returns()
func (_ManagerWithMerkleVerification *ManagerWithMerkleVerificationTransactorSession) SetOwner(newOwner common.Address) (*types.Transaction, error) {
	return _ManagerWithMerkleVerification.Contract.SetOwner(&_ManagerWithMerkleVerification.TransactOpts, newOwner)
}

// Unpause is a paid mutator transaction binding the contract method 0x3f4ba83a.
//
// Solidity: function unpause() returns()
func (_ManagerWithMerkleVerification *ManagerWithMerkleVerificationTransactor) Unpause(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _ManagerWithMerkleVerification.contract.Transact(opts, "unpause")
}

// Unpause is a paid mutator transaction binding the contract method 0x3f4ba83a.
//
// Solidity: function unpause() returns()
func (_ManagerWithMerkleVerification *ManagerWithMerkleVerificationSession) Unpause() (*types.Transaction, error) {
	return _ManagerWithMerkleVerification.Contract.Unpause(&_ManagerWithMerkleVerification.TransactOpts)
}

// Unpause is a paid mutator transaction binding the contract method 0x3f4ba83a.
//
// Solidity: function unpause() returns()
func (_ManagerWithMerkleVerification *ManagerWithMerkleVerificationTransactorSession) Unpause() (*types.Transaction, error) {
	return _ManagerWithMerkleVerification.Contract.Unpause(&_ManagerWithMerkleVerification.TransactOpts)
}

// ManagerWithMerkleVerificationBoringVaultManagedIterator is returned from FilterBoringVaultManaged and is used to iterate over the raw logs and unpacked data for BoringVaultManaged events raised by the ManagerWithMerkleVerification contract.
type ManagerWithMerkleVerificationBoringVaultManagedIterator struct {
	Event *ManagerWithMerkleVerificationBoringVaultManaged // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *ManagerWithMerkleVerificationBoringVaultManagedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ManagerWithMerkleVerificationBoringVaultManaged)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(ManagerWithMerkleVerificationBoringVaultManaged)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *ManagerWithMerkleVerificationBoringVaultManagedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ManagerWithMerkleVerificationBoringVaultManagedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ManagerWithMerkleVerificationBoringVaultManaged represents a BoringVaultManaged event raised by the ManagerWithMerkleVerification contract.
type ManagerWithMerkleVerificationBoringVaultManaged struct {
	CallsMade *big.Int
	Raw       types.Log // Blockchain specific contextual infos
}

// FilterBoringVaultManaged is a free log retrieval operation binding the contract event 0x53d426e7d80bb2c8674d3b45577e2d464d423faad6531b21f95ac11ac18b1cb6.
//
// Solidity: event BoringVaultManaged(uint256 callsMade)
func (_ManagerWithMerkleVerification *ManagerWithMerkleVerificationFilterer) FilterBoringVaultManaged(opts *bind.FilterOpts) (*ManagerWithMerkleVerificationBoringVaultManagedIterator, error) {

	logs, sub, err := _ManagerWithMerkleVerification.contract.FilterLogs(opts, "BoringVaultManaged")
	if err != nil {
		return nil, err
	}
	return &ManagerWithMerkleVerificationBoringVaultManagedIterator{contract: _ManagerWithMerkleVerification.contract, event: "BoringVaultManaged", logs: logs, sub: sub}, nil
}

// WatchBoringVaultManaged is a free log subscription operation binding the contract event 0x53d426e7d80bb2c8674d3b45577e2d464d423faad6531b21f95ac11ac18b1cb6.
//
// Solidity: event BoringVaultManaged(uint256 callsMade)
func (_ManagerWithMerkleVerification *ManagerWithMerkleVerificationFilterer) WatchBoringVaultManaged(opts *bind.WatchOpts, sink chan<- *ManagerWithMerkleVerificationBoringVaultManaged) (event.Subscription, error) {

	logs, sub, err := _ManagerWithMerkleVerification.contract.WatchLogs(opts, "BoringVaultManaged")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ManagerWithMerkleVerificationBoringVaultManaged)
				if err := _ManagerWithMerkleVerification.contract.UnpackLog(event, "BoringVaultManaged", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseBoringVaultManaged is a log parse operation binding the contract event 0x53d426e7d80bb2c8674d3b45577e2d464d423faad6531b21f95ac11ac18b1cb6.
//
// Solidity: event BoringVaultManaged(uint256 callsMade)
func (_ManagerWithMerkleVerification *ManagerWithMerkleVerificationFilterer) ParseBoringVaultManaged(log types.Log) (*ManagerWithMerkleVerificationBoringVaultManaged, error) {
	event := new(ManagerWithMerkleVerificationBoringVaultManaged)
	if err := _ManagerWithMerkleVerification.contract.UnpackLog(event, "BoringVaultManaged", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// ManagerWithMerkleVerificationManageRootUpdatedIterator is returned from FilterManageRootUpdated and is used to iterate over the raw logs and unpacked data for ManageRootUpdated events raised by the ManagerWithMerkleVerification contract.
type ManagerWithMerkleVerificationManageRootUpdatedIterator struct {
	Event *ManagerWithMerkleVerificationManageRootUpdated // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *ManagerWithMerkleVerificationManageRootUpdatedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ManagerWithMerkleVerificationManageRootUpdated)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(ManagerWithMerkleVerificationManageRootUpdated)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *ManagerWithMerkleVerificationManageRootUpdatedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ManagerWithMerkleVerificationManageRootUpdatedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ManagerWithMerkleVerificationManageRootUpdated represents a ManageRootUpdated event raised by the ManagerWithMerkleVerification contract.
type ManagerWithMerkleVerificationManageRootUpdated struct {
	Strategist common.Address
	OldRoot    [32]byte
	NewRoot    [32]byte
	Raw        types.Log // Blockchain specific contextual infos
}

// FilterManageRootUpdated is a free log retrieval operation binding the contract event 0x0b958dec85f1470000479dfb22c365829411f52bcde602d24ea0abf5ac7e8860.
//
// Solidity: event ManageRootUpdated(address indexed strategist, bytes32 oldRoot, bytes32 newRoot)
func (_ManagerWithMerkleVerification *ManagerWithMerkleVerificationFilterer) FilterManageRootUpdated(opts *bind.FilterOpts, strategist []common.Address) (*ManagerWithMerkleVerificationManageRootUpdatedIterator, error) {

	var strategistRule []interface{}
	for _, strategistItem := range strategist {
		strategistRule = append(strategistRule, strategistItem)
	}

	logs, sub, err := _ManagerWithMerkleVerification.contract.FilterLogs(opts, "ManageRootUpdated", strategistRule)
	if err != nil {
		return nil, err
	}
	return &ManagerWithMerkleVerificationManageRootUpdatedIterator{contract: _ManagerWithMerkleVerification.contract, event: "ManageRootUpdated", logs: logs, sub: sub}, nil
}

// WatchManageRootUpdated is a free log subscription operation binding the contract event 0x0b958dec85f1470000479dfb22c365829411f52bcde602d24ea0abf5ac7e8860.
//
// Solidity: event ManageRootUpdated(address indexed strategist, bytes32 oldRoot, bytes32 newRoot)
func (_ManagerWithMerkleVerification *ManagerWithMerkleVerificationFilterer) WatchManageRootUpdated(opts *bind.WatchOpts, sink chan<- *ManagerWithMerkleVerificationManageRootUpdated, strategist []common.Address) (event.Subscription, error) {

	var strategistRule []interface{}
	for _, strategistItem := range strategist {
		strategistRule = append(strategistRule, strategistItem)
	}

	logs, sub, err := _ManagerWithMerkleVerification.contract.WatchLogs(opts, "ManageRootUpdated", strategistRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ManagerWithMerkleVerificationManageRootUpdated)
				if err := _ManagerWithMerkleVerification.contract.UnpackLog(event, "ManageRootUpdated", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseManageRootUpdated is a log parse operation binding the contract event 0x0b958dec85f1470000479dfb22c365829411f52bcde602d24ea0abf5ac7e8860.
//
// Solidity: event ManageRootUpdated(address indexed strategist, bytes32 oldRoot, bytes32 newRoot)
func (_ManagerWithMerkleVerification *ManagerWithMerkleVerificationFilterer) ParseManageRootUpdated(log types.Log) (*ManagerWithMerkleVerificationManageRootUpdated, error) {
	event := new(ManagerWithMerkleVerificationManageRootUpdated)
	if err := _ManagerWithMerkleVerification.contract.UnpackLog(event, "ManageRootUpdated", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// ManagerWithMerkleVerificationOwnerUpdatedIterator is returned from FilterOwnerUpdated and is used to iterate over the raw logs and unpacked data for OwnerUpdated events raised by the ManagerWithMerkleVerification contract.
type ManagerWithMerkleVerificationOwnerUpdatedIterator struct {
	Event *ManagerWithMerkleVerificationOwnerUpdated // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *ManagerWithMerkleVerificationOwnerUpdatedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ManagerWithMerkleVerificationOwnerUpdated)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(ManagerWithMerkleVerificationOwnerUpdated)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *ManagerWithMerkleVerificationOwnerUpdatedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ManagerWithMerkleVerificationOwnerUpdatedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ManagerWithMerkleVerificationOwnerUpdated represents a OwnerUpdated event raised by the ManagerWithMerkleVerification contract.
type ManagerWithMerkleVerificationOwnerUpdated struct {
	User     common.Address
	NewOwner common.Address
	Raw      types.Log // Blockchain specific contextual infos
}

// FilterOwnerUpdated is a free log retrieval operation binding the contract event 0x8292fce18fa69edf4db7b94ea2e58241df0ae57f97e0a6c9b29067028bf92d76.
//
// Solidity: event OwnerUpdated(address indexed user, address indexed newOwner)
func (_ManagerWithMerkleVerification *ManagerWithMerkleVerificationFilterer) FilterOwnerUpdated(opts *bind.FilterOpts, user []common.Address, newOwner []common.Address) (*ManagerWithMerkleVerificationOwnerUpdatedIterator, error) {

	var userRule []interface{}
	for _, userItem := range user {
		userRule = append(userRule, userItem)
	}
	var newOwnerRule []interface{}
	for _, newOwnerItem := range newOwner {
		newOwnerRule = append(newOwnerRule, newOwnerItem)
	}

	logs, sub, err := _ManagerWithMerkleVerification.contract.FilterLogs(opts, "OwnerUpdated", userRule, newOwnerRule)
	if err != nil {
		return nil, err
	}
	return &ManagerWithMerkleVerificationOwnerUpdatedIterator{contract: _ManagerWithMerkleVerification.contract, event: "OwnerUpdated", logs: logs, sub: sub}, nil
}

// WatchOwnerUpdated is a free log subscription operation binding the contract event 0x8292fce18fa69edf4db7b94ea2e58241df0ae57f97e0a6c9b29067028bf92d76.
//
// Solidity: event OwnerUpdated(address indexed user, address indexed newOwner)
func (_ManagerWithMerkleVerification *ManagerWithMerkleVerificationFilterer) WatchOwnerUpdated(opts *bind.WatchOpts, sink chan<- *ManagerWithMerkleVerificationOwnerUpdated, user []common.Address, newOwner []common.Address) (event.Subscription, error) {

	var userRule []interface{}
	for _, userItem := range user {
		userRule = append(userRule, userItem)
	}
	var newOwnerRule []interface{}
	for _, newOwnerItem := range newOwner {
		newOwnerRule = append(newOwnerRule, newOwnerItem)
	}

	logs, sub, err := _ManagerWithMerkleVerification.contract.WatchLogs(opts, "OwnerUpdated", userRule, newOwnerRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ManagerWithMerkleVerificationOwnerUpdated)
				if err := _ManagerWithMerkleVerification.contract.UnpackLog(event, "OwnerUpdated", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseOwnerUpdated is a log parse operation binding the contract event 0x8292fce18fa69edf4db7b94ea2e58241df0ae57f97e0a6c9b29067028bf92d76.
//
// Solidity: event OwnerUpdated(address indexed user, address indexed newOwner)
func (_ManagerWithMerkleVerification *ManagerWithMerkleVerificationFilterer) ParseOwnerUpdated(log types.Log) (*ManagerWithMerkleVerificationOwnerUpdated, error) {
	event := new(ManagerWithMerkleVerificationOwnerUpdated)
	if err := _ManagerWithMerkleVerification.contract.UnpackLog(event, "OwnerUpdated", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// ManagerWithMerkleVerificationPausedIterator is returned from FilterPaused and is used to iterate over the raw logs and unpacked data for Paused events raised by the ManagerWithMerkleVerification contract.
type ManagerWithMerkleVerificationPausedIterator struct {
	Event *ManagerWithMerkleVerificationPaused // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *ManagerWithMerkleVerificationPausedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ManagerWithMerkleVerificationPaused)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(ManagerWithMerkleVerificationPaused)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *ManagerWithMerkleVerificationPausedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ManagerWithMerkleVerificationPausedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ManagerWithMerkleVerificationPaused represents a Paused event raised by the ManagerWithMerkleVerification contract.
type ManagerWithMerkleVerificationPaused struct {
	Raw types.Log // Blockchain specific contextual infos
}

// FilterPaused is a free log retrieval operation binding the contract event 0x9e87fac88ff661f02d44f95383c817fece4bce600a3dab7a54406878b965e752.
//
// Solidity: event Paused()
func (_ManagerWithMerkleVerification *ManagerWithMerkleVerificationFilterer) FilterPaused(opts *bind.FilterOpts) (*ManagerWithMerkleVerificationPausedIterator, error) {

	logs, sub, err := _ManagerWithMerkleVerification.contract.FilterLogs(opts, "Paused")
	if err != nil {
		return nil, err
	}
	return &ManagerWithMerkleVerificationPausedIterator{contract: _ManagerWithMerkleVerification.contract, event: "Paused", logs: logs, sub: sub}, nil
}

// WatchPaused is a free log subscription operation binding the contract event 0x9e87fac88ff661f02d44f95383c817fece4bce600a3dab7a54406878b965e752.
//
// Solidity: event Paused()
func (_ManagerWithMerkleVerification *ManagerWithMerkleVerificationFilterer) WatchPaused(opts *bind.WatchOpts, sink chan<- *ManagerWithMerkleVerificationPaused) (event.Subscription, error) {

	logs, sub, err := _ManagerWithMerkleVerification.contract.WatchLogs(opts, "Paused")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ManagerWithMerkleVerificationPaused)
				if err := _ManagerWithMerkleVerification.contract.UnpackLog(event, "Paused", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParsePaused is a log parse operation binding the contract event 0x9e87fac88ff661f02d44f95383c817fece4bce600a3dab7a54406878b965e752.
//
// Solidity: event Paused()
func (_ManagerWithMerkleVerification *ManagerWithMerkleVerificationFilterer) ParsePaused(log types.Log) (*ManagerWithMerkleVerificationPaused, error) {
	event := new(ManagerWithMerkleVerificationPaused)
	if err := _ManagerWithMerkleVerification.contract.UnpackLog(event, "Paused", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// ManagerWithMerkleVerificationUnpausedIterator is returned from FilterUnpaused and is used to iterate over the raw logs and unpacked data for Unpaused events raised by the ManagerWithMerkleVerification contract.
type ManagerWithMerkleVerificationUnpausedIterator struct {
	Event *ManagerWithMerkleVerificationUnpaused // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *ManagerWithMerkleVerificationUnpausedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ManagerWithMerkleVerificationUnpaused)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(ManagerWithMerkleVerificationUnpaused)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *ManagerWithMerkleVerificationUnpausedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ManagerWithMerkleVerificationUnpausedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ManagerWithMerkleVerificationUnpaused represents a Unpaused event raised by the ManagerWithMerkleVerification contract.
type ManagerWithMerkleVerificationUnpaused struct {
	Raw types.Log // Blockchain specific contextual infos
}

// FilterUnpaused is a free log retrieval operation binding the contract event 0xa45f47fdea8a1efdd9029a5691c7f759c32b7c698632b563573e155625d16933.
//
// Solidity: event Unpaused()
func (_ManagerWithMerkleVerification *ManagerWithMerkleVerificationFilterer) FilterUnpaused(opts *bind.FilterOpts) (*ManagerWithMerkleVerificationUnpausedIterator, error) {

	logs, sub, err := _ManagerWithMerkleVerification.contract.FilterLogs(opts, "Unpaused")
	if err != nil {
		return nil, err
	}
	return &ManagerWithMerkleVerificationUnpausedIterator{contract: _ManagerWithMerkleVerification.contract, event: "Unpaused", logs: logs, sub: sub}, nil
}

// WatchUnpaused is a free log subscription operation binding the contract event 0xa45f47fdea8a1efdd9029a5691c7f759c32b7c698632b563573e155625d16933.
//
// Solidity: event Unpaused()
func (_ManagerWithMerkleVerification *ManagerWithMerkleVerificationFilterer) WatchUnpaused(opts *bind.WatchOpts, sink chan<- *ManagerWithMerkleVerificationUnpaused) (event.Subscription, error) {

	logs, sub, err := _ManagerWithMerkleVerification.contract.WatchLogs(opts, "Unpaused")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ManagerWithMerkleVerificationUnpaused)
				if err := _ManagerWithMerkleVerification.contract.UnpackLog(event, "Unpaused", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseUnpaused is a log parse operation binding the contract event 0xa45f47fdea8a1efdd9029a5691c7f759c32b7c698632b563573e155625d16933.
//
// Solidity: event Unpaused()
func (_ManagerWithMerkleVerification *ManagerWithMerkleVerificationFilterer) ParseUnpaused(log types.Log) (*ManagerWithMerkleVerificationUnpaused, error) {
	event := new(ManagerWithMerkleVerificationUnpaused)
	if err := _ManagerWithMerkleVerification.contract.UnpackLog(event, "Unpaused", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}
