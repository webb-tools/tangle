pragma solidity ^0.8.0;

/**
 * @title EcdsaSecp256r1 interface to verify isgnature.
 */
interface IEcdsaSecp256r1 {
    /**
     * @dev Verify signed message .
     * @return A boolean confirming whether the public key is signer for the message. 
     */
    function verify(
        bytes32 public_key,
        bytes calldata signature,
        bytes calldata message
    ) external view returns (bool);
}