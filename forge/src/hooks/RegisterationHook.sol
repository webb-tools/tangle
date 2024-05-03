// SPDX-License-Identifier: GPL-3.0-only
pragma solidity >=0.8.3;

/// @dev Created by the service blueprint designer (gadget developer)
contract RegistrationHook {
    /// @dev Only allow the contract to call itself
    modifier onlySelf() {
      require(msg.sender == address(this), "RegistrationHook: Only self");
      _;
    }

    /// @dev A Hook that gets called by the runtime when an Operator tries to register on this blueprint.
    /// @param registrationInputs The inputs that the Operator provided during the registration.
    ///
    /// Unless this function reverts, the Operator will be registered on this blueprint.
    /// @custom:hook
    function onRegister(bytes calldata registrationInputs) public payable onlySelf {}
}
