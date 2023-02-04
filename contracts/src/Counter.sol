// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

contract Counter {
    uint public number = 0;
    event Log(uint updatedVal);
    
    function setNumber(uint newNumber) public {
        number = newNumber;
    }

    function increment() public {
        number++;
        emit Log(number);
    }
}
