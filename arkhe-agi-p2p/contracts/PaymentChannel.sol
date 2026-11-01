// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

/**
 * @title PaymentChannel
 * @notice Canais de streaming e micropagamento para liquidação de inferências (Hyperspace)
 */
contract PaymentChannel {
    struct Channel {
        address sender;
        address recipient;
        uint256 deposit;
        uint256 settled;
        bool isOpen;
    }

    mapping(bytes32 => Channel) public channels;

    event ChannelOpened(bytes32 indexed channelId, address indexed sender, address indexed recipient, uint256 deposit);
    event ChannelClosed(bytes32 indexed channelId, uint256 finalSettledAmount);

    function openChannel(bytes32 channelId, address recipient) external payable {
        require(channels[channelId].sender == address(0), "Channel exists");
        require(msg.value > 0, "Zero deposit");
        channels[channelId] = Channel(msg.sender, recipient, msg.value, 0, true);
        emit ChannelOpened(channelId, msg.sender, recipient, msg.value);
    }

    function closeChannel(bytes32 channelId, uint256 finalAmount) external {
        Channel storage ch = channels[channelId];
        require(ch.isOpen, "Channel closed");
        require(finalAmount <= ch.deposit, "Exceeds deposit");
        ch.isOpen = false;
        ch.settled = finalAmount;

        uint256 refund = ch.deposit - finalAmount;
        if (finalAmount > 0) {
            payable(ch.recipient).transfer(finalAmount);
        }
        if (refund > 0) {
            payable(ch.sender).transfer(refund);
        }
        emit ChannelClosed(channelId, finalAmount);
    }
}
