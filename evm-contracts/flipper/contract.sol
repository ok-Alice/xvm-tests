// compiler version must be greater than or equal to 0.8.13 and less than 0.9.0
//SPDX-License-Identifier: MIT
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

    address ink_address;

    constructor (address _ink_address) {
        ink_address = _ink_address;
    }

 
    function flip(
    ) public
    returns (bool)
    {
        bytes4 selector = 0x633aa551;
        bytes memory contract_address = abi.encodePacked(ink_address);
        bytes memory buffer = bytes.concat(
            selector
        );

        XVM_PRECOMPILE.xvm_call("\x1f\x00", contract_address, buffer);
        return true;
    }


    // mapped id Number(0) -> bool



    function encode_uint128(uint128 value) private pure returns (bytes memory) {
        return abi.encodePacked(value);
    }

}

