package integration_tests

import (
	"bytes"
	"context"
	"crypto/tls"
	"crypto/x509"
	"encoding/base64"
	"encoding/hex"
	"encoding/pem"
	"fmt"
	"io/ioutil"
	"math/big"
	"time"

	"github.com/cosmos/cosmos-sdk/client"
	"github.com/cosmos/cosmos-sdk/types/query"
	"github.com/ethereum/go-ethereum"
	"github.com/ethereum/go-ethereum/accounts/abi"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/crypto"
	"github.com/ethereum/go-ethereum/ethclient"
	gravityTypes "github.com/peggyjv/gravity-bridge/module/v5/x/gravity/types"
	corktypesv2 "github.com/peggyjv/sommelier/v8/x/cork/types/v2"
	"github.com/peggyjv/steward/steward_proto_go/steward_proto"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials"
)

// Steward/Cork integration tests follow the same basic pattern:
// 1. Query the cork module to ensure the relevant cellar ID is approved
// 2. Wait for the next cork vote period to begin
// 3. Construct and send the relevant cork request to each steward instance
// 4. Wait for the voting period to end so the cork will be submitted to the gravity module as a ContractCallTx
// 5. Query the gravity module to ensure the ContractCallTx was created
// 6. Wait for the LogicCallSubmitted event to occur in the gravity contract, indicating that the orchestrators
//	  relayed the ContractCallTx successfully.
// 7. Wait for the presumed Cellar contract event indicating a successful decoding of the function call corked by Steward.

const ZERO_ADDRESS = "0x0000000000000000000000000000000000000000"
const ONE_ADDRESS = "0x1111111111111111111111111111111111111111"

// AaveV2Stablecoin rebalance test values used in multiple methods
const T_AAVE_DAI = "0x6B175474E89094C44Da98b954EedeAC495271d0F"
const T_AAVE_POOL = "0xE340B25fE32B1011616bb8EC495A4d503e322177"
const T_AAVE_USDC = "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48"

// 1*10^18
const T_AAVE_MIN_ASSETS_OUT = "1000000000000000000"

func (s *IntegrationTestSuite) TestAaveV2Stablecoin() {
	s.Run("Submit rebalance request to MockAaveV2Stablecoin", func() {
		s.checkCellarExists(aaveCellar)

		cellarId := aaveCellar.String()

		val := s.chain.validators[0]
		kb, err := val.keyring()
		s.Require().NoError(err)
		clientCtx, err := s.chain.clientContext("tcp://localhost:26657", &kb, "val", val.address())
		s.Require().NoError(err)
		currentHeight, err := s.GetLatestBlockHeight(clientCtx)
		s.Require().NoError(err)
		scheduledHeight := currentHeight + 10

		// Create the cork request to send to Steward
		route := []string{T_AAVE_DAI, T_AAVE_POOL, T_AAVE_USDC, ZERO_ADDRESS, ZERO_ADDRESS, ZERO_ADDRESS, ZERO_ADDRESS, ZERO_ADDRESS, ZERO_ADDRESS}
		swapParams := []*steward_proto.AaveV2Stablecoin_Rebalance_SwapParams{
			{InIndex: 0, OutIndex: 2, SwapType: 1},
			{InIndex: 0, OutIndex: 0, SwapType: 0},
			{InIndex: 0, OutIndex: 0, SwapType: 0},
			{InIndex: 0, OutIndex: 0, SwapType: 0},
		}
		request := &steward_proto.ScheduleRequest{
			ChainId:  1,
			CellarId: cellarId,
			CallData: &steward_proto.ScheduleRequest_AaveV2Stablecoin{
				AaveV2Stablecoin: &steward_proto.AaveV2Stablecoin{
					Function: &steward_proto.AaveV2Stablecoin_Rebalance_{
						Rebalance: &steward_proto.AaveV2Stablecoin_Rebalance{
							Route:        route,
							SwapParams:   swapParams,
							MinAssetsOut: T_AAVE_MIN_ASSETS_OUT,
						},
					},
				},
			},
			BlockHeight: uint64(scheduledHeight),
		}

		s.executeStewardCalls(request)
		s.waitForScheduledHeight(clientCtx, scheduledHeight)

		// Construct invalidation scope and nonce for gravity query
		aave_abi, err := AaveV2MetaData.GetAbi()
		s.Require().NoError(err)

		methodName := "rebalance"
		zeroAddress := common.HexToAddress(ZERO_ADDRESS)
		solRoute := [9]common.Address{common.HexToAddress(T_AAVE_DAI), common.HexToAddress(T_AAVE_POOL), common.HexToAddress(T_AAVE_USDC), zeroAddress, zeroAddress, zeroAddress, zeroAddress, zeroAddress, zeroAddress}
		zero := big.NewInt(0)
		solSwapParams := &[4][3]*big.Int{{zero, big.NewInt(2), big.NewInt(1)}, {zero, zero, zero}, {zero, zero, zero}, {zero, zero, zero}}
		minAssetsOut := big.NewInt(1000000000000000000)
		method, err := aave_abi.Pack(methodName, solRoute, solSwapParams, minAssetsOut)
		addr := common.HexToAddress(aaveCellar.String())
		invalidationScope := crypto.Keccak256Hash(
			bytes.Join(
				[][]byte{addr.Bytes(), method},
				[]byte{},
			)).Bytes()
		invalidationNonce := 1
		s.Require().NoError(err)

		s.queryLogicCallTransaction(clientCtx, invalidationScope, invalidationNonce)

		s.waitForGravityLogicCallEvent(invalidationScope, invalidationNonce)

		s.T().Logf("checking for cellar event")
		s.Require().Eventuallyf(func() bool {
			s.T().Log("querying cellar events...")
			ethClient, err := ethclient.Dial(fmt.Sprintf("http://%s", s.ethResource.GetHostPort("8545/tcp")))
			if err != nil {
				return false
			}

			// For non-anonymous events, the first log topic is a keccak256 hash of the
			// event signature.
			eventSignature := []byte("mockRebalance(address[9],uint256[3][4],uint256)")
			mockEventSignatureTopic := crypto.Keccak256Hash(eventSignature)
			query := ethereum.FilterQuery{
				FromBlock: nil,
				ToBlock:   nil,
				Addresses: []common.Address{
					aaveCellar,
				},
				Topics: [][]common.Hash{
					{
						mockEventSignatureTopic,
					},
				},
			}

			logs, err := ethClient.FilterLogs(context.Background(), query)
			if err != nil {
				ethClient.Close()
				return false
			}

			s.T().Logf("got %v logs", len(logs))
			if len(logs) == 1 {
				s.T().Log("saw mock function event!")

				log := logs[0]
				if len(log.Data) > 0 {
					var event AaveV2MockRebalance
					err := aave_abi.UnpackIntoInterface(&event, "mockRebalance", log.Data)
					s.Require().NoError(err, "failed to unpack mockRebalance event from log data")
					s.Require().Equal(event.MinAssetsOut, minAssetsOut)
					return true
				}
			}

			return false
		}, 2*time.Minute, 5*time.Second, "cellar event never seen")
	})
}

func (s *IntegrationTestSuite) TestCellarV1() {
	s.Run("Submit rebalance to MockCellar", func() {
		s.checkCellarExists(vaultCellar)

		val := s.chain.validators[0]
		kb, err := val.keyring()
		s.Require().NoError(err)
		clientCtx, err := s.chain.clientContext("tcp://localhost:26657", &kb, "val", val.address())
		s.Require().NoError(err)

		// Create the cork requests to send to Steward
		from := ZERO_ADDRESS
		to := ONE_ADDRESS
		assetsFrom := "1000"
		cellarId := vaultCellar.String()
		swapParamsUniV2 := &steward_proto.SwapParams{
			Params: &steward_proto.SwapParams_Univ2Params{
				Univ2Params: &steward_proto.UniV2SwapParams{
					Path:         []string{ZERO_ADDRESS, ONE_ADDRESS},
					Amount:       "1000",
					AmountOutMin: "2000",
				},
			},
		}
		requestUniV2 := &steward_proto.ScheduleRequest{
			ChainId:  1,
			CellarId: cellarId,
			CallData: &steward_proto.ScheduleRequest_CellarV1{
				CellarV1: &steward_proto.CellarV1{
					Function: &steward_proto.CellarV1_Rebalance_{
						Rebalance: &steward_proto.CellarV1_Rebalance{
							FromPosition: from,
							ToPosition:   to,
							AssetsFrom:   assetsFrom,
							Exchange:     steward_proto.Exchange_EXCHANGE_UNIV2,
							Params:       swapParamsUniV2,
						},
					},
				},
			},
		}
		swapParamsUniV3 := &steward_proto.SwapParams{
			Params: &steward_proto.SwapParams_Univ3Params{
				Univ3Params: &steward_proto.UniV3SwapParams{
					Path:         []string{ZERO_ADDRESS, ONE_ADDRESS},
					PoolFees:     []uint32{0, 0},
					Amount:       "1000",
					AmountOutMin: "2000",
				},
			},
		}
		requestUniV3 := &steward_proto.ScheduleRequest{
			ChainId:  1,
			CellarId: cellarId,
			CallData: &steward_proto.ScheduleRequest_CellarV1{
				CellarV1: &steward_proto.CellarV1{
					Function: &steward_proto.CellarV1_Rebalance_{
						Rebalance: &steward_proto.CellarV1_Rebalance{
							FromPosition: from,
							ToPosition:   to,
							AssetsFrom:   assetsFrom,
							Exchange:     steward_proto.Exchange_EXCHANGE_UNIV3,
							Params:       swapParamsUniV3,
						},
					},
				},
			},
		}

		s.T().Log("running through two sequences to test rebalance with both UniV2 and Univ3 swap params")
		for sequence, request := range []*steward_proto.ScheduleRequest{requestUniV2, requestUniV3} {
			s.T().Log("starting sequence")
			currentHeight, err := s.GetLatestBlockHeight(clientCtx)
			s.Require().NoError(err)
			scheduledHeight := currentHeight + 10
			request.BlockHeight = uint64(scheduledHeight)
			s.executeStewardCalls(request)
			s.waitForScheduledHeight(clientCtx, scheduledHeight)

			// Construct invalidation scope and nonce for gravity query
			vault_abi, err := CellarMetaData.GetAbi()
			s.Require().NoError(err)

			methodName := "rebalance"
			addressArrayT, _ := abi.NewType("address[]", "address[]", nil)
			feeArrayT, _ := abi.NewType("uint32[]", "uint32[]", nil)
			uint256T, _ := abi.NewType("uint256", "uint256", nil)

			var args abi.Arguments
			var params []byte
			if sequence == 0 {
				args = abi.Arguments{
					{
						Type: addressArrayT,
					},
					{
						Type: uint256T,
					},
					{
						Type: uint256T,
					},
				}

				params, err = args.Pack(
					[]common.Address{
						common.HexToAddress(ZERO_ADDRESS),
						common.HexToAddress(ONE_ADDRESS),
					},
					big.NewInt(1000),
					big.NewInt(2000),
				)
				s.Require().NoError(err)
			} else {
				args = abi.Arguments{
					{
						Type: addressArrayT,
					},
					{
						Type: feeArrayT,
					},
					{
						Type: uint256T,
					},
					{
						Type: uint256T,
					},
				}

				params, err = args.Pack(
					[]common.Address{
						common.HexToAddress(ZERO_ADDRESS),
						common.HexToAddress(ONE_ADDRESS),
					},
					[]uint32{0, 0},
					big.NewInt(1000),
					big.NewInt(2000),
				)
				s.Require().NoError(err)
			}
			method, err := vault_abi.Pack(methodName, common.HexToAddress(from), common.HexToAddress(to), big.NewInt(1000), uint8(sequence), params)
			s.Require().NoError(err)

			addr := common.HexToAddress(vaultCellar.String())
			invalidationScope := crypto.Keccak256Hash(
				bytes.Join(
					[][]byte{addr.Bytes(), method},
					[]byte{},
				)).Bytes()
			invalidationNonce := sequence + 1
			s.queryLogicCallTransaction(clientCtx, invalidationScope, invalidationNonce)

			s.waitForGravityLogicCallEvent(invalidationScope, invalidationNonce)

			s.T().Logf("checking for cellar event")
			s.Require().Eventuallyf(func() bool {
				s.T().Log("querying cellar events...")
				ethClient, err := ethclient.Dial(fmt.Sprintf("http://%s", s.ethResource.GetHostPort("8545/tcp")))
				if err != nil {
					return false
				}

				// For non-anonymous events, the first log topic is a keccak256 hash of the
				// event signature.
				eventSignature := []byte("Rebalance(address,address,uint256,uint8)")
				mockEventSignatureTopic := crypto.Keccak256Hash(eventSignature)
				query := ethereum.FilterQuery{
					FromBlock: nil,
					ToBlock:   nil,
					Addresses: []common.Address{
						vaultCellar,
					},
					Topics: [][]common.Hash{
						{
							mockEventSignatureTopic,
						},
					},
				}

				logs, err := ethClient.FilterLogs(context.Background(), query)
				if err != nil {
					ethClient.Close()
					return false
				}

				s.T().Logf("got %v logs: %v", len(logs), logs)
				if len(logs) > 0 {
					for _, log := range logs {
						if len(log.Data) > 0 {
							var event CellarRebalance
							err := vault_abi.UnpackIntoInterface(&event, "Rebalance", log.Data)
							s.Require().NoError(err, "failed to unpack Rebalance event from log data")

							if sequence == 0 {
								if event.Exchange == 0 {
									s.T().Log("saw mock function event!")
									return true
								}
							} else {
								if event.Exchange == 1 {
									s.T().Log("saw mock function event!")
									return true
								}
							}
						}
					}
				}

				return false
			}, 2*time.Minute, 10*time.Second, "cellar event never seen")

			s.T().Log("sequence complete!")
		}
	})
}

func (s *IntegrationTestSuite) TestCellarV2() {
	s.Run("Submit callOnAdaptor to MockCellar", func() {
		s.checkCellarExists(vaultCellar)

		val := s.chain.validators[0]
		kb, err := val.keyring()
		s.Require().NoError(err)
		clientCtx, err := s.chain.clientContext("tcp://localhost:26657", &kb, "val", val.address())
		s.Require().NoError(err)

		// Create the cork requests to send to Steward
		cellarId := vaultCellar.String()
		swapParamsUniV2 := &steward_proto.SwapParams{
			Params: &steward_proto.SwapParams_Univ2Params{
				Univ2Params: &steward_proto.UniV2SwapParams{
					Path:         []string{ZERO_ADDRESS, ONE_ADDRESS},
					Amount:       "1000",
					AmountOutMin: "2000",
				},
			},
		}

		// Contains two adaptor calls, the first of which has two function calls inside of it, for a total of three function calls.
		request := &steward_proto.ScheduleRequest{
			ChainId:  1,
			CellarId: cellarId,
			CallData: &steward_proto.ScheduleRequest_CellarV2{
				CellarV2: &steward_proto.CellarV2{
					Function: &steward_proto.CellarV2_CallOnAdaptor_{
						CallOnAdaptor: &steward_proto.CellarV2_CallOnAdaptor{
							Data: []*steward_proto.AdaptorCall{
								{
									Adaptor: adaptorContract.Hex(),
									CallData: &steward_proto.AdaptorCall_AaveDebtTokenV1Calls{
										AaveDebtTokenV1Calls: &steward_proto.AaveDebtTokenAdaptorV1Calls{
											Calls: []*steward_proto.AaveDebtTokenAdaptorV1{
												{
													Function: &steward_proto.AaveDebtTokenAdaptorV1_BorrowFromAave_{
														BorrowFromAave: &steward_proto.AaveDebtTokenAdaptorV1_BorrowFromAave{
															Token:  "0x2222222222222222222222222222222222222222",
															Amount: "250000",
														},
													},
												},
												{
													Function: &steward_proto.AaveDebtTokenAdaptorV1_SwapAndRepay_{
														SwapAndRepay: &steward_proto.AaveDebtTokenAdaptorV1_SwapAndRepay{
															TokenIn:      "0x3333333333333333333333333333333333333333",
															TokenToRepay: "0x4444444444444444444444444444444444444444",
															AmountIn:     "5000",
															Exchange:     steward_proto.Exchange_EXCHANGE_UNIV2,
															Params:       swapParamsUniV2,
														},
													},
												},
											},
										},
									},
								},
								{
									Adaptor: adaptorContract.Hex(),
									CallData: &steward_proto.AdaptorCall_ZeroXV1Calls{
										ZeroXV1Calls: &steward_proto.ZeroXAdaptorV1Calls{
											Calls: []*steward_proto.ZeroXAdaptorV1{
												{
													Function: &steward_proto.ZeroXAdaptorV1_SwapWith_0X{
														SwapWith_0X: &steward_proto.ZeroXAdaptorV1_SwapWith0X{
															TokenIn:      "0x5555555555555555555555555555555555555555",
															TokenOut:     "0x6666666666666666666666666666666666666666",
															Amount:       "6000",
															SwapCallData: []byte("0xdeadbeef"),
														},
													},
												},
											},
										},
									},
								},
							},
						},
					},
				},
			},
		}

		currentHeight, err := s.GetLatestBlockHeight(clientCtx)
		s.Require().NoError(err)
		scheduledHeight := currentHeight + 10
		request.BlockHeight = uint64(scheduledHeight)
		s.executeStewardCalls(request)
		s.waitForScheduledHeight(clientCtx, scheduledHeight)
		s.queryLogicCallTransactionByAddress(clientCtx, vaultCellar.Hex())

		// Construct invalidation scope and nonce for gravity query
		// vault_abi, err := CellarMetaData.GetAbi()
		adaptor_abi, err := AdaptorMetaData.GetAbi()
		s.Require().NoError(err)

		s.T().Logf("checking for BorrowFromAave event")
		s.Require().Eventuallyf(func() bool {
			s.T().Log("querying cellar events...")
			ethClient, err := ethclient.Dial(fmt.Sprintf("http://%s", s.ethResource.GetHostPort("8545/tcp")))
			if err != nil {
				return false
			}

			// For non-anonymous events, the first log topic is a keccak256 hash of the
			// event signature.
			eventSignature := []byte("BorrowFromAave(address,uint256)")
			mockEventSignatureTopic := crypto.Keccak256Hash(eventSignature)
			query := ethereum.FilterQuery{
				FromBlock: nil,
				ToBlock:   nil,
				Addresses: []common.Address{
					vaultCellar,
				},
				Topics: [][]common.Hash{
					{
						mockEventSignatureTopic,
					},
				},
			}

			logs, err := ethClient.FilterLogs(context.Background(), query)
			if err != nil {
				ethClient.Close()
				return false
			}

			s.T().Logf("got %v logs: %v", len(logs), logs)
			if len(logs) > 0 {
				for _, log := range logs {
					if len(log.Data) > 0 {
						var event AdaptorBorrowFromAave
						err := adaptor_abi.UnpackIntoInterface(&event, "BorrowFromAave", log.Data)
						s.Require().NoError(err, "failed to unpack BorrowFromAave event from log data")
						s.Require().Equal(common.HexToAddress("0x2222222222222222222222222222222222222222"), event.DebtTokenToBorrow)

						s.T().Log("Saw BorrowFromAave event!")
						return true
					}
				}
			}

			return false
		}, 3*time.Minute, 20*time.Second, "cellar event never seen")

		s.T().Logf("checking for SwapWithZeroX event")
		s.Require().Eventuallyf(func() bool {
			s.T().Log("querying cellar events...")
			ethClient, err := ethclient.Dial(fmt.Sprintf("http://%s", s.ethResource.GetHostPort("8545/tcp")))
			if err != nil {
				return false
			}

			// For non-anonymous events, the first log topic is a keccak256 hash of the
			// event signature.
			eventSignature := []byte("SwapWithZeroX(address,address,uint256,bytes)")
			mockEventSignatureTopic := crypto.Keccak256Hash(eventSignature)
			query := ethereum.FilterQuery{
				FromBlock: nil,
				ToBlock:   nil,
				Addresses: []common.Address{
					vaultCellar,
				},
				Topics: [][]common.Hash{
					{
						mockEventSignatureTopic,
					},
				},
			}

			logs, err := ethClient.FilterLogs(context.Background(), query)
			if err != nil {
				ethClient.Close()
				return false
			}

			s.T().Logf("got %v logs: %v", len(logs), logs)
			if len(logs) > 0 {
				for _, log := range logs {
					if len(log.Data) > 0 {
						var event AdaptorSwapWithZeroX
						err := adaptor_abi.UnpackIntoInterface(&event, "SwapWithZeroX", log.Data)
						s.Require().NoError(err, "failed to unpack SwapWithZeroX event from log data")
						// 1 == UniswapV3
						s.Require().Equal(big.NewInt(6000), event.Amount)

						s.T().Log("Saw SwapWithZeroX event!")
						return true
					}
				}
			}

			return false
		}, 3*time.Minute, 20*time.Second, "cellar event never seen")

		s.T().Logf("checking for SwapAndRepay event")
		s.Require().Eventuallyf(func() bool {
			s.T().Log("querying cellar events...")
			ethClient, err := ethclient.Dial(fmt.Sprintf("http://%s", s.ethResource.GetHostPort("8545/tcp")))
			if err != nil {
				return false
			}

			// For non-anonymous events, the first log topic is a keccak256 hash of the
			// event signature.
			eventSignature := []byte("SwapAndRepay(address,address,uint256,uint8,bytes)")
			mockEventSignatureTopic := crypto.Keccak256Hash(eventSignature)
			query := ethereum.FilterQuery{
				FromBlock: nil,
				ToBlock:   nil,
				Addresses: []common.Address{
					vaultCellar,
				},
				Topics: [][]common.Hash{
					{
						mockEventSignatureTopic,
					},
				},
			}

			logs, err := ethClient.FilterLogs(context.Background(), query)
			if err != nil {
				ethClient.Close()
				return false
			}

			s.T().Logf("got %v logs: %v", len(logs), logs)
			if len(logs) > 0 {
				for _, log := range logs {
					if len(log.Data) > 0 {
						var event AdaptorSwapAndRepay
						err := adaptor_abi.UnpackIntoInterface(&event, "SwapAndRepay", log.Data)
						s.Require().NoError(err, "failed to unpack SwapAndRepay event from log data")
						// 0 == UniswapV2
						s.Require().Equal(uint8(0), event.Exchange)

						s.T().Log("Saw SwapAndRepay event!")
						return true
					}
				}
			}

			return false
		}, 3*time.Minute, 20*time.Second, "cellar event never seen")
	})
}

func (s *IntegrationTestSuite) TestCellarV2_2() {
	s.Run("Submit callOnAdaptor to MockCellar", func() {
		s.checkCellarExists(v2_2Cellar)

		val := s.chain.validators[0]
		kb, err := val.keyring()
		s.Require().NoError(err)
		clientCtx, err := s.chain.clientContext("tcp://localhost:26657", &kb, "val", val.address())
		s.Require().NoError(err)

		// Create the cork requests to send to Steward
		cellarId := v2_2Cellar.Hex()

		// Contains two adaptor calls, the first of which has two function calls inside of it, for a total of three function calls.
		request := &steward_proto.ScheduleRequest{
			ChainId:  1,
			CellarId: cellarId,
			CallData: &steward_proto.ScheduleRequest_CellarV2_2{
				CellarV2_2: &steward_proto.CellarV2_2{
					CallType: &steward_proto.CellarV2_2_Multicall_{
						Multicall: &steward_proto.CellarV2_2_Multicall{
							FunctionCalls: []*steward_proto.CellarV2_2_FunctionCall{
								{
									Function: &steward_proto.CellarV2_2_FunctionCall_CallOnAdaptor{
										CallOnAdaptor: &steward_proto.CellarV2_2_CallOnAdaptor{
											Data: []*steward_proto.AdaptorCall{
												{
													Adaptor: adaptorContract.Hex(),
													CallData: &steward_proto.AdaptorCall_SwapWithUniswapV1Calls{
														SwapWithUniswapV1Calls: &steward_proto.SwapWithUniswapAdaptorV1Calls{
															Calls: []*steward_proto.SwapWithUniswapAdaptorV1{
																{
																	Function: &steward_proto.SwapWithUniswapAdaptorV1_SwapWithUniV3_{
																		SwapWithUniV3: &steward_proto.SwapWithUniswapAdaptorV1_SwapWithUniV3{
																			Path:         []string{"0x1111111111111111111111111111111111111111", "0xAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"},
																			PoolFees:     []uint32{1000, 2000},
																			Amount:       "2",
																			AmountOutMin: "2",
																		},
																	},
																},
															},
														},
													},
												},
											},
										},
									},
								},
								{
									Function: &steward_proto.CellarV2_2_FunctionCall_CallOnAdaptor{
										CallOnAdaptor: &steward_proto.CellarV2_2_CallOnAdaptor{
											Data: []*steward_proto.AdaptorCall{
												{
													Adaptor: adaptorContract.Hex(),
													CallData: &steward_proto.AdaptorCall_SwapWithUniswapV1Calls{
														SwapWithUniswapV1Calls: &steward_proto.SwapWithUniswapAdaptorV1Calls{
															Calls: []*steward_proto.SwapWithUniswapAdaptorV1{
																{
																	Function: &steward_proto.SwapWithUniswapAdaptorV1_SwapWithUniV3_{
																		SwapWithUniV3: &steward_proto.SwapWithUniswapAdaptorV1_SwapWithUniV3{
																			Path:         []string{"0x1111111111111111111111111111111111111111", "0xAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"},
																			PoolFees:     []uint32{1000, 2000},
																			Amount:       "3",
																			AmountOutMin: "3",
																		},
																	},
																},
															},
														},
													},
												},
											},
										},
									},
								},
							},
						},
					},
				},
			},
		}

		currentHeight, err := s.GetLatestBlockHeight(clientCtx)
		s.Require().NoError(err)
		scheduledHeight := currentHeight + 10
		request.BlockHeight = uint64(scheduledHeight)
		s.executeStewardCalls(request)
		s.waitForScheduledHeight(clientCtx, scheduledHeight)
		s.queryLogicCallTransactionByAddress(clientCtx, v2_2Cellar.Hex())

		adaptor_abi, err := AdaptorMetaData.GetAbi()
		s.Require().NoError(err)

		s.T().Logf("checking for SwapWithUniV3 event")
		s.Require().Eventuallyf(func() bool {
			s.T().Log("querying cellar events...")
			ethClient, err := ethclient.Dial(fmt.Sprintf("http://%s", s.ethResource.GetHostPort("8545/tcp")))
			if err != nil {
				return false
			}

			// For non-anonymous events, the first log topic is a keccak256 hash of the
			// event signature.
			eventSignature := []byte("SwapWithUniV3(address[],uint24[],uint256,uint256)")
			mockEventSignatureTopic := crypto.Keccak256Hash(eventSignature)
			query := ethereum.FilterQuery{
				FromBlock: nil,
				ToBlock:   nil,
				Addresses: []common.Address{
					v2_2Cellar,
				},
				Topics: [][]common.Hash{
					{
						mockEventSignatureTopic,
					},
				},
			}

			logs, err := ethClient.FilterLogs(context.Background(), query)
			if err != nil {
				ethClient.Close()
				return false
			}

			s.T().Logf("got %v logs: %v", len(logs), logs)
			if len(logs) > 0 {
				for _, log := range logs {
					if len(log.Data) > 0 {
						var event AdaptorSwapWithUniV3
						err := adaptor_abi.UnpackIntoInterface(&event, "SwapWithUniV3", log.Data)
						s.Require().NoError(err, "failed to unpack SwapWithUniV3 event from log data")

						if event.AmountOutMin.Cmp(big.NewInt(2)) == 0 {
							continue
						}

						s.Require().Equal(big.NewInt(3), event.AmountOutMin)

						s.T().Log("Saw SwapWithUniV3 event!")
						return true
					}
				}
			}

			return false
		}, 3*time.Minute, 20*time.Second, "cellar event never seen")
	})
}

func (s *IntegrationTestSuite) TestBoringVaultManager() {
	s.Run("Set the manage root of the Manager contract", func() {
		s.checkCellarExists(managerContract)

		cellarId := managerContract.String()

		val := s.chain.validators[0]
		kb, err := val.keyring()
		s.Require().NoError(err)
		clientCtx, err := s.chain.clientContext("tcp://localhost:26657", &kb, "val", val.address())
		s.Require().NoError(err)
		currentHeight, err := s.GetLatestBlockHeight(clientCtx)
		s.Require().NoError(err)
		scheduledHeight := currentHeight + 10

		// Create the cork request to send to Steward
		manageRoot := "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
		strategist := "0x1111111111111111111111111111111111111111"
		request := &steward_proto.ScheduleRequest{
			ChainId:  1,
			CellarId: cellarId,
			CallData: &steward_proto.ScheduleRequest_BoringVaultManagerWithMerkleVerification{
				BoringVaultManagerWithMerkleVerification: &steward_proto.BoringVaultManagerWithMerkleVerification{
					Function: &steward_proto.BoringVaultManagerWithMerkleVerification_SetManageRoot_{
						SetManageRoot: &steward_proto.BoringVaultManagerWithMerkleVerification_SetManageRoot{
							ManageRoot: manageRoot,
							Strategist: strategist,
						},
					},
				},
			},
			BlockHeight: uint64(scheduledHeight),
		}

		s.executeStewardCalls(request)
		s.waitForScheduledHeight(clientCtx, scheduledHeight)

		// Construct invalidation scope and nonce for gravity query
		managerABI, err := ManagerWithMerkleVerificationMetaData.GetAbi()
		s.Require().NoError(err)

		methodName := "setManageRoot"
		var manageRootBytes [32]byte
		manageRootHex, err := hex.DecodeString(manageRoot)
		s.Require().NoError(err)
		copy(manageRootBytes[:], manageRootHex)
		method, err := managerABI.Pack(methodName, common.HexToAddress(strategist), manageRootBytes)
		s.Require().NoError(err)
		addr := common.HexToAddress(cellarId)
		invalidationScope := crypto.Keccak256Hash(
			bytes.Join(
				[][]byte{addr.Bytes(), method},
				[]byte{},
			)).Bytes()
		invalidationNonce := 1

		s.queryLogicCallTransaction(clientCtx, invalidationScope, invalidationNonce)

		// For non-anonymous events, the first log topic is a keccak256 hash o	f the
		// event signature.
		s.waitForGravityLogicCallEvent(invalidationScope, invalidationNonce)

		s.T().Logf("checking for contract event")
		s.Require().Eventuallyf(func() bool {
			s.T().Log("querying contract events...")
			ethClient, err := ethclient.Dial(fmt.Sprintf("http://%s", s.ethResource.GetHostPort("8545/tcp")))
			if err != nil {
				return false
			}

			// For non-anonymous events, the first log topic is a keccak256 hash of the
			// event signature.
			eventSignature := []byte("ManageRootUpdated(address,bytes32,bytes32)")
			manageRootUpdatedEventSignatureTopic := crypto.Keccak256Hash(eventSignature)
			query := ethereum.FilterQuery{
				FromBlock: nil,
				ToBlock:   nil,
				Addresses: []common.Address{
					managerContract,
				},
				Topics: [][]common.Hash{
					{
						manageRootUpdatedEventSignatureTopic,
					},
				},
			}

			logs, err := ethClient.FilterLogs(context.Background(), query)
			if err != nil {
				ethClient.Close()
				return false
			}
			s.T().Logf("got %v logs", len(logs))
			if len(logs) == 1 {
				s.T().Log("saw ManageRootUpdated event!")

				log := logs[0]
				if len(log.Topics) >= 2 { // First topic is event signature, second is indexed strategist
					strategistAddr := common.HexToAddress(log.Topics[1].Hex())
					var event ManagerWithMerkleVerificationManageRootUpdated
					err := managerABI.UnpackIntoInterface(&event, "ManageRootUpdated", log.Data)
					s.Require().NoError(err, "failed to unpack ManageRootUpdated event from log data")
					s.Require().Equal(strategist, strategistAddr.Hex())
					s.Require().Equal(manageRoot, hex.EncodeToString(event.NewRoot[:]))
					return true
				}
			}

			return false
		}, 2*time.Minute, 5*time.Second, "contract event never seen")
	})
}

func (s *IntegrationTestSuite) checkCellarExists(cellar common.Address) {
	s.T().Logf("checking that cellar %s exists in the chain", cellar.String())
	queryClient, err := s.chain.validators[0].GetQueryClient()
	s.Require().NoError(err, "error getting query client")
	s.Require().Eventuallyf(func() bool {
		res, err := queryClient.QueryCellarIDs(context.Background(), &corktypesv2.QueryCellarIDsRequest{})
		if err != nil || res == nil {
			return false
		}

		for _, c := range res.CellarIds {
			if c == cellar.String() {
				return true
			}
		}

		return false
	}, 60*time.Second, 5*time.Second, "cellar %s not found in approved cellars list", cellar.String())
}

func (s *IntegrationTestSuite) waitForScheduledHeight(clientCtx *client.Context, height int64) {
	s.T().Logf("wait for height %d", height)
	startingHeight, err := s.GetLatestBlockHeight(clientCtx)
	s.Require().NoError(err)
	delta := height - startingHeight
	s.Require().Eventuallyf(func() bool {
		currentHeight, err := s.GetLatestBlockHeight(clientCtx)
		s.Require().NoError(err)
		return currentHeight >= height
		// block time in tests is usually ~1 second so this gives some cushion
	}, time.Duration(delta*3)*time.Second, 1*time.Second, "scheduled height never reached")
}

func (s *IntegrationTestSuite) executeStewardCalls(request *steward_proto.ScheduleRequest) {
	s.T().Logf("submit contract call data to steward")
	s.Require().Eventually(func() bool {
		// Load in client TLS config
		// client cert
		clientCert, err := tls.LoadX509KeyPair(
			"integration_tests/tls/client/test_client.crt",
			"integration_tests/tls/client/test_client_key_pkcs8.pem",
		)
		s.Require().NoError(err)

		// server CA
		rootPool := x509.NewCertPool()
		r, err := ioutil.ReadFile("integration_tests/tls/server/test_server_ca.crt")
		s.Require().NoError(err)

		block, _ := pem.Decode(r)
		rootCert, err := x509.ParseCertificate(block.Bytes)
		s.Require().NoError(err)

		rootPool.AddCert(rootCert)

		tlsConfig := &tls.Config{
			Certificates: []tls.Certificate{clientCert},
			RootCAs:      rootPool,
		}
		creds := credentials.NewTLS(tlsConfig)

		// make it rain
		for i := range s.chain.stewards {
			addr := fmt.Sprintf("localhost:%v", STEWARD_PORT+i)
			conn, err := grpc.Dial(
				addr,
				grpc.WithBlock(),
				grpc.WithTransportCredentials(creds),
			)
			s.Require().NoError(err)
			defer conn.Close()

			ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
			defer cancel()

			c := steward_proto.NewContractCallServiceClient(conn)
			s.T().Logf("sending request to %s", s.chain.validators[i].address())
			_, err = c.Schedule(ctx, request)
			s.Require().NoError(err)
		}

		return true
	}, 100*time.Second, 1*time.Second, "Cork request took too long")
}

func (s *IntegrationTestSuite) queryLogicCallTransaction(clientCtx *client.Context, invalidationScope []byte, invalidationNonce int) {
	s.T().Logf("querying gravity module for logic call transaction with invalidation scope %s and nonce %d", base64.StdEncoding.EncodeToString(invalidationScope), invalidationNonce)
	gravityQueryClient := gravityTypes.NewQueryClient(clientCtx)
	s.Require().Eventuallyf(func() bool {
		request := &gravityTypes.ContractCallTxsRequest{
			Pagination: &query.PageRequest{},
		}
		response, _ := gravityQueryClient.ContractCallTxs(context.Background(), request)
		completedRequest := &gravityTypes.CompletedContractCallTxsRequest{}
		completedResponse, _ := gravityQueryClient.CompletedContractCallTxs(context.Background(), completedRequest)
		if response != nil {
			for _, call := range response.Calls {
				if bytes.Equal(call.InvalidationScope, invalidationScope) && call.InvalidationNonce == uint64(invalidationNonce) {
					s.T().Log("logic call found in the gravity module!")
					return true
				}
			}
		}
		if completedResponse != nil {
			for _, call := range completedResponse.CompletedContractCallTxs {
				if bytes.Equal(call.InvalidationScope, invalidationScope) && call.InvalidationNonce == uint64(invalidationNonce) {
					s.T().Log("completed logic call found in the gravity module!")
					return true
				}
			}
		}

		return false
	}, 2*time.Minute, 10*time.Second, "logic call transaction never seen")
	time.Sleep(time.Duration(2000000000))
}

func (s *IntegrationTestSuite) queryLogicCallTransactionByAddress(clientCtx *client.Context, address string) {
	s.T().Log("querying gravity module for logic call transaction")
	gravityQueryClient := gravityTypes.NewQueryClient(clientCtx)
	s.Require().Eventuallyf(func() bool {
		request := &gravityTypes.ContractCallTxsRequest{
			Pagination: &query.PageRequest{},
		}
		response, _ := gravityQueryClient.ContractCallTxs(context.Background(), request)
		if response != nil {
			for _, call := range response.Calls {
				if call.Address == address {
					s.T().Logf("logic call to %s found in the gravity module!", address)
					return true
				}
			}
		}

		return false
	}, 1*time.Minute, 5*time.Second, "logic call transaction never seen")
	time.Sleep(time.Duration(2000000000))
}

func (s *IntegrationTestSuite) waitForGravityLogicCallEvent(invalidationScope []byte, invalidationNonce int) {
	s.T().Logf("waiting for gravity to submit call to cellar")
	eventSignature := []byte("LogicCallEvent(bytes32,uint256,bytes,uint256)")
	topic := crypto.Keccak256Hash(eventSignature)
	gravity_abi, err := GravityMetaData.GetAbi()
	s.Require().NoError(err)
	s.Require().Eventuallyf(func() bool {
		s.T().Log("querying gravity logic call events...")
		ethClient, err := ethclient.Dial(fmt.Sprintf("http://%s", s.ethResource.GetHostPort("8545/tcp")))
		if err != nil {
			return false
		}

		// Get the latest block number
		latestBlock, err := ethClient.BlockNumber(context.Background())
		if err != nil {
			ethClient.Close()
			return false
		}
		fromBlock := uint64(1)

		query := ethereum.FilterQuery{
			FromBlock: big.NewInt(int64(fromBlock)),
			ToBlock:   big.NewInt(int64(latestBlock)),
			Addresses: []common.Address{
				gravityContract,
			},
			Topics: [][]common.Hash{
				{
					topic,
				},
			},
		}
		logs, err := ethClient.FilterLogs(context.Background(), query)
		if err != nil {
			ethClient.Close()
			return false
		}

		s.T().Logf("got %v LogicCallEvent logs", len(logs))
		if len(logs) == 0 {
			return false
		}

		var event GravityLogicCallEvent
		for _, log := range logs {
			if len(log.Data) > 0 {
				err := gravity_abi.UnpackIntoInterface(&event, "LogicCallEvent", log.Data)
				if err != nil {
					continue
				}

				eventInvalidationId := event.InvalidationId[:]
				if bytes.Equal(eventInvalidationId, invalidationScope) && int(event.InvalidationNonce.Int64()) == invalidationNonce {
					s.T().Log("logic call executed!")
					return true
				}
			} else {
				s.T().Log("no data in log")
			}
		}
		return false
	}, 2*time.Minute, 10*time.Second, "LogicCallEvent never seen")
}
