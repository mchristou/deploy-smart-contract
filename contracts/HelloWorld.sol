// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.0;

contract HelloWorld {
    string private message;

    constructor(string memory _message) {
        message = _message;
    }

    function updateMessage(string memory _message) public {
        message = _message;
    }

    function getMessage() public view returns (string memory) {
        return message;
    }
}
