// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

/**
 * @title InferenceAttestation
 * @notice Registro on-chain de atestação de inferência (SUM Chain / EVM)
 */
contract InferenceAttestation {
    event InferenceAttested(
        bytes32 indexed modelCid,
        bytes32 indexed inferenceRoot,
        address indexed attestedBy
    );

    mapping(bytes32 => bool) public validInferenceRoots;

    function registerInference(bytes32 modelCid, bytes32 inferenceRoot) external {
        require(inferenceRoot != bytes32(0), "Invalid inference root");
        validInferenceRoots[inferenceRoot] = true;
        emit InferenceAttested(modelCid, inferenceRoot, msg.sender);
    }
}
