// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.16;

import {Address, Owned} from "./interfaces.sol";

contract ManagerWithMerkleVerification is Owned {
    using Address for address;

    // ========================================= STATE =========================================

    /**
     * @notice A merkle tree root that restricts what data can be passed to the BoringVault.
     * @dev Maps a strategist address to their specific merkle root.
     * @dev Each leaf is composed of the keccak256 hash of abi.encodePacked {decodersAndSanitizer, target, valueIsNonZero, selector, argumentAddress_0, ...., argumentAddress_N}
     *      Where:
     *             - decodersAndSanitizer is the addres to call to extract packed address arguments from the calldata
     *             - target is the address to make the call to
     *             - valueIsNonZero is a bool indicating whether or not the value is non-zero
     *             - selector is the function selector on target
     *             - argumentAddress is each allowed address argument in that call
     */
    mapping(address => bytes32) public manageRoot;

    /**
     * @notice Used to pause calls to `manageVaultWithMerkleVerification`.
     */
    bool public isPaused;

    //============================== ERRORS ===============================

    error ManagerWithMerkleVerification__InvalidManageProofLength();
    error ManagerWithMerkleVerification__InvalidTargetDataLength();
    error ManagerWithMerkleVerification__InvalidValuesLength();
    error ManagerWithMerkleVerification__InvalidDecodersAndSanitizersLength();
    error ManagerWithMerkleVerification__FlashLoanNotExecuted();
    error ManagerWithMerkleVerification__FlashLoanNotInProgress();
    error ManagerWithMerkleVerification__BadFlashLoanIntentHash();
    error ManagerWithMerkleVerification__FailedToVerifyManageProof(address target, bytes targetData, uint256 value);
    error ManagerWithMerkleVerification__Paused();
    error ManagerWithMerkleVerification__OnlyCallableByBoringVault();
    error ManagerWithMerkleVerification__OnlyCallableByBalancerVault();
    error ManagerWithMerkleVerification__TotalSupplyMustRemainConstantDuringPlatform();

    //============================== EVENTS ===============================

    event ManageRootUpdated(address indexed strategist, bytes32 oldRoot, bytes32 newRoot);
    event BoringVaultManaged(uint256 callsMade);
    event Paused();
    event Unpaused();

    //============================== IMMUTABLES ===============================

    /**
     * @notice The BoringVault this contract can manage.
     */
    address public immutable vault;

    constructor(address _owner, address _vault, address _balancerVault) Owned(msg.sender) {
        vault = payable(_vault);
    }

    // ========================================= ADMIN FUNCTIONS =========================================

    /**
     * @notice Sets the manageRoot.
     * @dev Callable by OWNER_ROLE.
     */
    function setManageRoot(address strategist, bytes32 _manageRoot) external {
        bytes32 oldRoot = manageRoot[strategist];
        manageRoot[strategist] = _manageRoot;
        emit ManageRootUpdated(strategist, oldRoot, _manageRoot);
    }

    /**
     * @notice Pause this contract, which prevents future calls to `manageVaultWithMerkleVerification`.
     * @dev Callable by MULTISIG_ROLE.
     */
    function pause() external {
        isPaused = true;
        emit Paused();
    }

    /**
     * @notice Unpause this contract, which allows future calls to `manageVaultWithMerkleVerification`.
     * @dev Callable by MULTISIG_ROLE.
     */
    function unpause() external {
        isPaused = false;
        emit Unpaused();
    }
}
