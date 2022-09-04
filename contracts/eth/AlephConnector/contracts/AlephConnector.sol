// SPDX-License-Identifier: MIT

pragma solidity >=0.8.16;

contract AlephConnector {
    event AlephMintETH(bytes32 indexed receiver, uint256 amount);

    function lock(bytes32 receiver) external payable {
        emit AlephMintETH(receiver, msg.value);
    }

    // TODO: implement reverse communication
}
