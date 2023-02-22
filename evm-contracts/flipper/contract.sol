// compiler version must be greater than or equal to 0.8.13 and less than 0.9.0
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

interface XVM {
    function xvm_call(
        bytes calldata context,
        bytes calldata to,
        bytes calldata input
    ) external;
}

contract flipper {
    XVM constant XVM_PRECOMPILE = XVM(0x0000000000000000000000000000000000005005);


    constructor () {
    }

 
    function flip(
        bytes memory contract_address
    ) public
    returns (bool)
    {
        bytes4 selector = 0x633aa551;
        bytes memory encoded_address = abi.encodePacked(contract_address);
        bytes memory buffer = bytes.concat(
            selector
        );

        XVM_PRECOMPILE.xvm_call("\x1f\x00\x00\x00\x00", encoded_address, buffer);
        return true;
    }
 
    function set(
        bytes memory ink_address,
        bool  value
    ) public
    returns (bool)
    {
        bytes4 selector = 0xe8c45eb6;
        bytes memory contract_address = abi.encodePacked(ink_address);
        bytes memory buffer = bytes.concat(
            selector,
            abi.encodePacked(value)
        );

        XVM_PRECOMPILE.xvm_call("\x1f\x00\x00\x00\x00", contract_address, buffer);
        return true;
    }


    function encode_uint128(uint128 value) private pure returns (bytes memory) {
        return abi.encodePacked(value);
    }

}

