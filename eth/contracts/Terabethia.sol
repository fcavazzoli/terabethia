pragma solidity ^0.8.0;

import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import "./IStarknetCore.sol";
import "./ITerabethiaCore.sol";

contract Terabethia is Initializable, ITerabethiaCore {
    // The StarkNet core contract.
    IStarknetCore starknetCore;

    bytes32 constant STORAGE_POSITION =
        keccak256("terabethia.storage.position");

    // Terabethia Contract on Starknet
    uint256 constant STARKNET_CONTRACT =
        0x011215026475fe87b55d6638ee97b0113427d667f4a1d8a6cc8d0b573ea702af;

    struct SimpleStorage {
        mapping(bytes32 => uint256) messages;
        uint256 nonce;
    }

    event LogMessageToL2(
        address indexed from_address,
        uint256 indexed to_address,
        uint256 indexed nonce,
        uint256[] payload
    );

    function initialize(IStarknetCore starknetCore_) public initializer {
        starknetCore = starknetCore_;
    }

    function simpleStorage() internal pure returns (SimpleStorage storage ds) {
        bytes32 position = STORAGE_POSITION;

        assembly {
            ds.slot := position
        }
    }

    function consumeMessage(uint256 from_address, uint256[] calldata data)
        external
        returns (bytes32)
    {
        bytes32 msgHash = keccak256(
            abi.encodePacked(
                from_address,
                uint256(uint160(msg.sender)),
                data.length,
                data
            )
        );

        uint256[] memory payload = new uint256[](1);
        payload[0] = uint256(msgHash);

        return starknetCore.consumeMessageFromL2(STARKNET_CONTRACT, payload);
    }

    function sendMessage(uint256 to_address, uint256[] calldata payload)
        external
        returns (bytes32)
    {
        simpleStorage().nonce += 1;

        bytes32 msgHash = keccak256(
            abi.encodePacked(
                uint256(uint160(msg.sender)),
                to_address,
                simpleStorage().nonce,
                payload.length,
                payload
            )
        );

        simpleStorage().messages[msgHash] += 1;

        // we only emit event, so we can auto-trigger message consumption on the IC
        emit LogMessageToL2(
            msg.sender,
            to_address,
            simpleStorage().nonce,
            payload
        );

        return msgHash;
    }
}
