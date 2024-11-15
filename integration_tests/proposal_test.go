package integration_tests

import (
	"context"
	"encoding/hex"
	"fmt"
	"time"

	"cosmossdk.io/math"
	sdk "github.com/cosmos/cosmos-sdk/types"
	govtypesv1beta1 "github.com/cosmos/cosmos-sdk/x/gov/types/v1beta1"
	ethereum "github.com/ethereum/go-ethereum"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/crypto"
	"github.com/ethereum/go-ethereum/ethclient"
	axelarcorktypes "github.com/peggyjv/sommelier/v8/x/axelarcork/types"
	corktypesv2 "github.com/peggyjv/sommelier/v8/x/cork/types/v2"
)

func (s *IntegrationTestSuite) TestScheduledCorkProposal() {
	s.checkCellarExists(vaultCellar)

	orch := s.chain.orchestrators[0]
	orchClientCtx, err := s.chain.clientContext("tcp://localhost:26657", orch.keyring, "orch", orch.address())
	s.Require().NoError(err)
	currentHeight, err := s.GetLatestBlockHeight(orchClientCtx)
	s.Require().NoError(err)
	protoJson := `
	{
		"call": {
			"CellarV1": {
				"function": {
					"TrustPosition" : {
						"position": {
							"Erc20Address": "0x0000000000000000000000000000000000000000"
						}
					}
				}
			}
		}
	}
	`

	targetBlockHeight := currentHeight + 90
	proposal := corktypesv2.NewScheduledCorkProposal(
		"scheduled cork proposal test",
		"description",
		uint64(targetBlockHeight),
		vaultCellar.String(),
		protoJson,
	)

	proposalMsg, err := govtypesv1beta1.NewMsgSubmitProposal(
		proposal,
		sdk.Coins{
			{
				Denom:  testDenom,
				Amount: math.NewInt(1000000),
			},
		},
		orch.address(),
	)
	s.Require().NoError(err, "Unable to create governance proposal")

	s.T().Log("Submit proposal")
	submitProposalResponse, err := s.chain.sendMsgs(*orchClientCtx, proposalMsg)
	s.Require().NoError(err)
	s.Require().Zero(submitProposalResponse.Code, "raw log: %s", submitProposalResponse.RawLog)

	s.T().Log("Check proposal was submitted correctly")
	govQueryClient := govtypesv1beta1.NewQueryClient(orchClientCtx)
	var proposalID uint64
	s.Require().Eventually(func() bool {
		proposalsQueryResponse, err := govQueryClient.Proposals(context.Background(), &govtypesv1beta1.QueryProposalsRequest{})
		if err != nil {
			s.T().Logf("error querying proposals: %e", err)
			return false
		}

		if len(proposalsQueryResponse.Proposals) == 0 {
			return false
		}

		for _, p := range proposalsQueryResponse.Proposals {
			if p.Content.TypeUrl == "/cork.v2.ScheduledCorkProposal" {
				s.Require().Equal(govtypesv1beta1.StatusVotingPeriod, p.Status, "proposal not in voting period")
				proposalID = p.ProposalId
				return true
			}
		}

		return false
	}, time.Second*30, time.Second*5, "proposal submission was never found")

	s.T().Log("Vote for proposal")
	// wait so the client for val0 will be aware of the latest tx sequence
	time.Sleep(time.Second * 10)
	for _, val := range s.chain.validators {
		kr, err := val.keyring()
		s.Require().NoError(err)
		localClientCtx, err := s.chain.clientContext("tcp://localhost:26657", &kr, "val", val.address())
		s.Require().NoError(err)

		voteMsg := govtypesv1beta1.NewMsgVote(val.address(), proposalID, govtypesv1beta1.OptionYes)
		voteResponse, err := s.chain.sendMsgs(*localClientCtx, voteMsg)
		s.Require().NoError(err)
		s.Require().Zero(voteResponse.Code, "Vote error: %s", voteResponse.RawLog)
	}

	s.T().Log("Waiting for proposal to be approved..")
	s.Require().Eventually(func() bool {
		proposalQueryResponse, _ := govQueryClient.Proposal(context.Background(), &govtypesv1beta1.QueryProposalRequest{ProposalId: proposalID})
		return govtypesv1beta1.StatusPassed == proposalQueryResponse.Proposal.Status
	}, time.Second*30, time.Second*5, "proposal was never accepted")
	s.T().Log("Proposal approved!")

	s.T().Log("Waiting for scheduled cork to be created by steward")
	corkQueryClient := corktypesv2.NewQueryClient(orchClientCtx)
	s.Require().Eventually(func() bool {
		proposalQueryResponse, _ := corkQueryClient.QueryScheduledCorks(context.Background(), &corktypesv2.QueryScheduledCorksRequest{})
		return len(proposalQueryResponse.Corks) > 0
	}, time.Second*120, time.Second*2, "corks never scheduled")

	s.T().Log("wait for scheduled height")
	s.Require().Eventuallyf(func() bool {
		currentHeight, err := s.GetLatestBlockHeight(orchClientCtx)
		if err != nil {
			s.T().Logf("error quering latest height (probably transient): %s", err)
			return false
		}
		if currentHeight >= targetBlockHeight {
			return true
		} else {
			res, err := corkQueryClient.QueryScheduledCorks(context.Background(), &corktypesv2.QueryScheduledCorksRequest{})
			if err != nil {
				s.T().Logf("error: %s", err)
				return false
			}

			s.T().Logf("call: %s, height: %d, address: %s", hex.EncodeToString(res.Corks[0].Cork.EncodedContractCall), res.Corks[0].BlockHeight, res.Corks[0].Cork.TargetContractAddress)
			// verify that the scheduled corks have not yet been consumed
			s.Require().Len(res.Corks, len(s.chain.validators))
		}

		return false
	}, 3*time.Minute, 10*time.Second, "never reached scheduled height")

	s.T().Logf("checking for cellar event")
	s.Require().Eventuallyf(func() bool {
		s.T().Log("querying cellar events...")
		ethClient, err := ethclient.Dial(fmt.Sprintf("http://%s", s.ethResource.GetHostPort("8545/tcp")))
		if err != nil {
			return false
		}

		// For non-anonymous events, the first log topic is a keccak256 hash of the
		// event signature.
		eventSignature := []byte("TrustChanged(address,bool)")
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

		vault_abi, err := CellarMetaData.GetAbi()
		s.Require().NoError(err)

		s.T().Logf("got %v logs: %v", len(logs), logs)
		if len(logs) > 0 {
			for _, log := range logs {
				if len(log.Data) > 0 {
					var event CellarTrustChanged
					err := vault_abi.UnpackIntoInterface(&event, "TrustChanged", log.Data)
					s.Require().NoError(err, "failed to unpack TrustChanged event from log data")

					return true
				}
			}
		}

		return false
	}, 3*time.Minute, 10*time.Second, "cellar event never seen")
}

func (s *IntegrationTestSuite) TestScheduledCorkMulticallProposal() {
	s.checkCellarExists(vaultCellar)

	orch := s.chain.orchestrators[0]
	orchClientCtx, err := s.chain.clientContext("tcp://localhost:26657", orch.keyring, "orch", orch.address())
	s.Require().NoError(err)
	currentHeight, err := s.GetLatestBlockHeight(orchClientCtx)
	s.Require().NoError(err)
	protoJson := fmt.Sprintf(`
    {
        "call": {
            "CellarV22": {
                "call_type": {
                    "Multicall": {
                        "function_calls": [
                            {
                                "function": {
                                    "AddAdaptorToCatalogue": {
                                        "adaptor": "%s"
                                    }
                                }
                            },
                            {
                                "function": {
                                    "AddPositionToCatalogue": {
                                        "position_id": 1
                                    }
                                }
                            }
                        ]
                    }
                }
            }
        }
    }
    `, adaptorContract.Hex())

	targetBlockHeight := currentHeight + 90
	proposal := corktypesv2.NewScheduledCorkProposal(
		"scheduled cork proposal test",
		"description",
		uint64(targetBlockHeight),
		v2_2Cellar.String(),
		protoJson,
	)

	proposalMsg, err := govtypesv1beta1.NewMsgSubmitProposal(
		proposal,
		sdk.Coins{
			{
				Denom:  testDenom,
				Amount: math.NewInt(1000000),
			},
		},
		orch.address(),
	)
	s.Require().NoError(err, "Unable to create governance proposal")

	s.T().Log("Submit proposal")
	submitProposalResponse, err := s.chain.sendMsgs(*orchClientCtx, proposalMsg)
	s.Require().NoError(err)
	s.Require().Zero(submitProposalResponse.Code, "raw log: %s", submitProposalResponse.RawLog)

	s.T().Log("Check proposal was submitted correctly")
	govQueryClient := govtypesv1beta1.NewQueryClient(orchClientCtx)
	var proposalID uint64
	s.Require().Eventually(func() bool {
		proposalsQueryResponse, err := govQueryClient.Proposals(context.Background(), &govtypesv1beta1.QueryProposalsRequest{})
		if err != nil {
			s.T().Logf("error querying proposals: %e", err)
			return false
		}

		if len(proposalsQueryResponse.Proposals) == 0 {
			return false
		}

		for _, p := range proposalsQueryResponse.Proposals {
			if p.Content.TypeUrl == "/cork.v2.ScheduledCorkProposal" {
				s.Require().Equal(govtypesv1beta1.StatusVotingPeriod, p.Status, "proposal not in voting period")
				proposalID = p.ProposalId
				return true
			}
		}

		return false
	}, time.Second*30, time.Second*5, "proposal submission was never found")

	s.T().Log("Vote for proposal")
	// wait so the client for val0 will be aware of the latest tx sequence
	time.Sleep(time.Second * 10)
	for _, val := range s.chain.validators {
		kr, err := val.keyring()
		s.Require().NoError(err)
		localClientCtx, err := s.chain.clientContext("tcp://localhost:26657", &kr, "val", val.address())
		s.Require().NoError(err)

		voteMsg := govtypesv1beta1.NewMsgVote(val.address(), proposalID, govtypesv1beta1.OptionYes)
		voteResponse, err := s.chain.sendMsgs(*localClientCtx, voteMsg)
		s.Require().NoError(err)
		s.Require().Zero(voteResponse.Code, "Vote error: %s", voteResponse.RawLog)
	}

	s.T().Log("Waiting for proposal to be approved..")
	s.Require().Eventually(func() bool {
		proposalQueryResponse, _ := govQueryClient.Proposal(context.Background(), &govtypesv1beta1.QueryProposalRequest{ProposalId: proposalID})
		return govtypesv1beta1.StatusPassed == proposalQueryResponse.Proposal.Status
	}, time.Second*30, time.Second*5, "proposal was never accepted")
	s.T().Log("Proposal approved!")

	s.T().Log("Waiting for scheduled cork to be created by steward")
	corkQueryClient := corktypesv2.NewQueryClient(orchClientCtx)
	s.Require().Eventually(func() bool {
		proposalQueryResponse, _ := corkQueryClient.QueryScheduledCorks(context.Background(), &corktypesv2.QueryScheduledCorksRequest{})
		return len(proposalQueryResponse.Corks) > 0
	}, time.Second*120, time.Second*2, "corks never scheduled")

	s.T().Log("wait for scheduled height")
	s.Require().Eventuallyf(func() bool {
		currentHeight, err := s.GetLatestBlockHeight(orchClientCtx)
		if err != nil {
			s.T().Logf("error quering latest height (probably transient): %s", err)
			return false
		}
		if currentHeight >= targetBlockHeight {
			return true
		} else {
			res, err := corkQueryClient.QueryScheduledCorks(context.Background(), &corktypesv2.QueryScheduledCorksRequest{})
			if err != nil {
				s.T().Logf("error: %s", err)
				return false
			}

			s.T().Logf("call: %s, height: %d, address: %s", hex.EncodeToString(res.Corks[0].Cork.EncodedContractCall), res.Corks[0].BlockHeight, res.Corks[0].Cork.TargetContractAddress)
			// verify that the scheduled corks have not yet been consumed
			s.Require().Len(res.Corks, len(s.chain.validators))
		}

		return false
	}, 3*time.Minute, 10*time.Second, "never reached scheduled height")

	s.T().Logf("checking for cellar events")
	s.Require().Eventuallyf(func() bool {
		s.T().Log("querying add adaptor cellar event...")
		ethClient, err := ethclient.Dial(fmt.Sprintf("http://%s", s.ethResource.GetHostPort("8545/tcp")))
		if err != nil {
			return false
		}

		// For non-anonymous events, the first log topic is a keccak256 hash of the
		// event signature.
		eventSignature := []byte("AddAdaptorToCatalogue(address)")
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

		vault_abi, err := CellarV22MetaData.GetAbi()
		s.Require().NoError(err)

		if len(logs) > 0 {
			s.T().Logf("found %d logs!", len(logs))
			for _, log := range logs {
				if len(log.Data) > 0 {
					var event CellarV22AddAdaptorToCatalogue
					err := vault_abi.UnpackIntoInterface(&event, "AddAdaptorToCatalogue", log.Data)
					s.Require().NoError(err, "failed to unpack AddAdaptorToCatalogue event from log data")
					s.Require().Equal(common.HexToAddress(adaptorContract.Hex()), event.Adaptor)

					return true
				}
			}
		}

		return false
	}, 3*time.Minute, 10*time.Second, "add adaptor cellar event never seen")

	s.Require().Eventuallyf(func() bool {
		s.T().Log("querying add position cellar event...")
		ethClient, err := ethclient.Dial(fmt.Sprintf("http://%s", s.ethResource.GetHostPort("8545/tcp")))
		if err != nil {
			return false
		}

		// For non-anonymous events, the first log topic is a keccak256 hash of the
		// event signature.
		eventSignature := []byte("AddPositionToCatalogue(uint32)")
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

		vault_abi, err := CellarV22MetaData.GetAbi()
		s.Require().NoError(err)

		if len(logs) > 0 {
			s.T().Logf("found %d logs!", len(logs))
			for _, log := range logs {
				if len(log.Data) > 0 {
					var event CellarV22AddPositionToCatalogue
					err := vault_abi.UnpackIntoInterface(&event, "AddPositionToCatalogue", log.Data)
					s.Require().NoError(err, "failed to unpack AddPositionToCatalogue event from log data")
					s.Require().Equal(uint32(1), event.Position)

					return true
				}
			}
		}

		return false
	}, 3*time.Minute, 10*time.Second, "add position cellar event never seen")
}

func (s *IntegrationTestSuite) TestScheduledAxelarCorkProposal() {
	s.checkCellarExists(vaultCellar)

	orch := s.chain.orchestrators[0]
	orchClientCtx, err := s.chain.clientContext("tcp://localhost:26657", orch.keyring, "orch", orch.address())
	s.Require().NoError(err)
	currentHeight, err := s.GetLatestBlockHeight(orchClientCtx)
	s.Require().NoError(err)
	protoJson := `
	{
		"call": {
			"CellarV1": {
				"function": {
					"TrustPosition" : {
						"position": {
							"Erc20Address": "0x0000000000000000000000000000000000000000"
						}
					}
				}
			}
		}
	}
	`

	targetBlockHeight := currentHeight + 90
	chainID := uint64(10)
	deadline := uint64(time.Now().Add(10 * time.Minute).Unix())
	proposal := axelarcorktypes.NewAxelarScheduledCorkProposal(
		"axelarscheduled cork proposal test",
		"description",
		uint64(targetBlockHeight),
		chainID,
		vaultCellar.String(),
		protoJson,
		deadline,
	)

	proposalMsg, err := govtypesv1beta1.NewMsgSubmitProposal(
		proposal,
		sdk.Coins{
			{
				Denom:  testDenom,
				Amount: math.NewInt(1000000),
			},
		},
		orch.address(),
	)
	s.Require().NoError(err, "Unable to create governance proposal")

	s.T().Log("Submit proposal")
	submitProposalResponse, err := s.chain.sendMsgs(*orchClientCtx, proposalMsg)
	s.Require().NoError(err)
	s.Require().Zero(submitProposalResponse.Code, "raw log: %s", submitProposalResponse.RawLog)

	s.T().Log("Check proposal was submitted correctly")
	govQueryClient := govtypesv1beta1.NewQueryClient(orchClientCtx)
	var proposalID uint64
	s.Require().Eventually(func() bool {
		proposalsQueryResponse, err := govQueryClient.Proposals(context.Background(), &govtypesv1beta1.QueryProposalsRequest{})
		if err != nil {
			s.T().Logf("error querying proposals: %e", err)
			return false
		}

		if len(proposalsQueryResponse.Proposals) == 0 {
			return false
		}

		for _, p := range proposalsQueryResponse.Proposals {
			if p.Content.TypeUrl == "/axelarcork.v1.AxelarScheduledCorkProposal" {
				s.Require().Equal(govtypesv1beta1.StatusVotingPeriod, p.Status, "proposal not in voting period")
				proposalID = p.ProposalId
				return true
			}
		}

		return false
	}, time.Second*30, time.Second*5, "proposal submission was never found")

	s.T().Log("Vote for proposal")
	// wait so the client for val0 will be aware of the latest tx sequence
	time.Sleep(time.Second * 10)
	for _, val := range s.chain.validators {
		kr, err := val.keyring()
		s.Require().NoError(err)
		localClientCtx, err := s.chain.clientContext("tcp://localhost:26657", &kr, "val", val.address())
		s.Require().NoError(err)

		voteMsg := govtypesv1beta1.NewMsgVote(val.address(), proposalID, govtypesv1beta1.OptionYes)
		voteResponse, err := s.chain.sendMsgs(*localClientCtx, voteMsg)
		s.Require().NoError(err)
		s.Require().Zero(voteResponse.Code, "Vote error: %s", voteResponse.RawLog)
	}

	s.T().Log("Waiting for proposal to be approved..")
	s.Require().Eventually(func() bool {
		proposalQueryResponse, _ := govQueryClient.Proposal(context.Background(), &govtypesv1beta1.QueryProposalRequest{ProposalId: proposalID})
		return govtypesv1beta1.StatusPassed == proposalQueryResponse.Proposal.Status
	}, time.Second*30, time.Second*5, "proposal was never accepted")
	s.T().Log("Proposal approved!")

	s.T().Log("Waiting for scheduled cork to be created by steward")
	axelarcorkQueryClient := axelarcorktypes.NewQueryClient(orchClientCtx)
	s.Require().Eventually(func() bool {
		proposalQueryResponse, _ := axelarcorkQueryClient.QueryScheduledCorks(context.Background(), &axelarcorktypes.QueryScheduledCorksRequest{ChainId: chainID})
		return len(proposalQueryResponse.Corks) == 4
	}, time.Second*120, time.Second*2, "corks never scheduled")

	s.T().Log("wait for scheduled height")
	s.Require().Eventuallyf(func() bool {
		currentHeight, err := s.GetLatestBlockHeight(orchClientCtx)
		if err != nil {
			s.T().Logf("error quering latest height (probably transient): %s", err)
			return false
		}
		if currentHeight >= targetBlockHeight {
			return true
		} else {
			res, err := axelarcorkQueryClient.QueryScheduledCorks(context.Background(), &axelarcorktypes.QueryScheduledCorksRequest{})
			if err != nil {
				return false
			}

			s.T().Logf("call: %s, height: %d, chain ID: %d, address: %s", hex.EncodeToString(res.Corks[0].Cork.EncodedContractCall), res.Corks[0].BlockHeight, res.Corks[0].Cork.ChainId, res.Corks[0].Cork.TargetContractAddress)
			// verify that the scheduled corks have not yet been consumed
			s.Require().Len(res.Corks, len(s.chain.validators))
		}

		return false
	}, 3*time.Minute, 10*time.Second, "never reached scheduled height")

	s.T().Log("waiting to see approved cork result")
	s.Require().Eventually(func() bool {
		axelarcorkResultQueryResponse, _ := axelarcorkQueryClient.QueryCorkResults(context.Background(), &axelarcorktypes.QueryCorkResultsRequest{ChainId: chainID})
		if len(axelarcorkResultQueryResponse.CorkResults) > 0 {
			if !axelarcorkResultQueryResponse.CorkResults[0].Approved {
				s.Fail("cork result not approved")
			}

			return true
		}

		return false
	}, time.Second*30, time.Second*5, "cork result not approved")
}
