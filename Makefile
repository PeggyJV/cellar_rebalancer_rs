.DEFAULT_GOAL := e2e_cork_test

SOMMELIER_VERSION := "v8.0.3"
VALIDATOR_IMAGE := "ghcr.io/peggyjv/sommelier-sommelier:$(SOMMELIER_VERSION)"
ORCHESTRATOR_IMAGE := "ghcr.io/peggyjv/gravity-bridge-orchestrator:main"

go_protos:
	@scripts/build_go_protos.sh

api_docs:
	@scripts/build_api_docs.sh

e2e_build_images: e2e_clean_slate
	@docker pull $(VALIDATOR_IMAGE)
	@docker tag $(VALIDATOR_IMAGE) sommelier:prebuilt
	@docker pull $(ORCHESTRATOR_IMAGE)
	@docker tag $(ORCHESTRATOR_IMAGE) orchestrator:prebuilt
	@docker build -t steward:prebuilt -f Dockerfile .
	@docker build -t ethereum:prebuilt -f integration_tests/ethereum/Dockerfile integration_tests/ethereum/

e2e_clean_slate:
	@scripts/clean_slate.sh

e2e_cork_test: e2e_aave_v2_stablecoin_test e2e_cellar_v1_test e2e_cellar_v2_test e2e_cellar_v2_2_test e2e_scheduled_cork_proposal_test e2e_scheduled_axelar_cork_proposal_test e2e_scheduled_cork_proposal_multicall_test e2e_boring_vault_manager_test

# Because of the way `make` works, using the e2e_clean_slate as as a prerequisite for
# the individual tests doesn't work when `e2e_cork_test` runs the test targets in series,
# so we explicitly call the cleanup script in each test target.
e2e_aave_v2_stablecoin_test:
	@scripts/clean_slate.sh
	@E2E_SKIP_CLEANUP=true integration_tests/integration_tests.test -test.failfast -test.v -test.run IntegrationTestSuite -testify.m TestAaveV2Stablecoin || make -s fail

e2e_cellar_v1_test:
	@scripts/clean_slate.sh
	@E2E_SKIP_CLEANUP=true integration_tests/integration_tests.test -test.failfast -test.v -test.run IntegrationTestSuite -testify.m TestCellarV1 || make -s fail

e2e_cellar_v2_test:
	@scripts/clean_slate.sh
	@E2E_SKIP_CLEANUP=true integration_tests/integration_tests.test -test.failfast -test.v -test.run IntegrationTestSuite -testify.m TestCellarV2 || make -s fail

e2e_cellar_v2_2_test:
	@scripts/clean_slate.sh
	@E2E_SKIP_CLEANUP=true integration_tests/integration_tests.test -test.failfast -test.v -test.run IntegrationTestSuite -testify.m TestCellarV2_2 || make -s fail

e2e_scheduled_cork_proposal_test:
	@scripts/clean_slate.sh
	@E2E_SKIP_CLEANUP=true integration_tests/integration_tests.test -test.failfast -test.v -test.run IntegrationTestSuite -testify.m TestScheduledCorkProposal || make -s fail

e2e_scheduled_axelar_cork_proposal_test:
	@scripts/clean_slate.sh
	@E2E_SKIP_CLEANUP=true integration_tests/integration_tests.test -test.failfast -test.v -test.run IntegrationTestSuite -testify.m TestScheduledAxelarCorkProposal || make -s fail

e2e_scheduled_cork_proposal_multicall_test:
	@scripts/clean_slate.sh
	@E2E_SKIP_CLEANUP=true integration_tests/integration_tests.test -test.failfast -test.v -test.run IntegrationTestSuite -testify.m TestScheduledCorkMulticallProposal || make -s fail

e2e_boring_vault_manager_test:
	@scripts/clean_slate.sh
	@E2E_SKIP_CLEANUP=true integration_tests/integration_tests.test -test.failfast -test.v -test.run IntegrationTestSuite -testify.m TestBoringVaultManager || make -s fail

fail:
	@echo 'test failed; dumping container logs into ./testdata for review'
	@mkdir -p ./testdata
	@docker logs ethereum > testdata/ethereum.log 2>&1 || true
	@docker logs sommelier0 > testdata/sommelier0.log 2>&1 || true
	@docker logs sommelier1 > testdata/sommelier1.log 2>&1 || true
	@docker logs sommelier2 > testdata/sommelier2.log 2>&1 || true
	@docker logs sommelier3 > testdata/sommelier3.log 2>&1 || true
	@docker logs orchestrator0 > testdata/orchestrator0.log 2>&1 || true
	@docker logs orchestrator1 > testdata/orchestrator1.log 2>&1 || true
	@docker logs orchestrator2 > testdata/orchestrator2.log 2>&1 || true
	@docker logs orchestrator3 > testdata/orchestrator3.log 2>&1 || true
	@docker logs steward0 > testdata/steward0.log 2>&1 || true
	@docker logs steward1 > testdata/steward1.log 2>&1 || true
	@docker logs steward2 > testdata/steward2.log 2>&1 || true
	@docker logs steward3 > testdata/steward3.log 2>&1 || true
	@false

