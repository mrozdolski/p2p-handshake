## P2P Handshake Node Project

This project demonstrates a P2P handshake mechanism between two nodes. The handshake is initiated by one node and acknowledged by the other, following a predefined sequence of messages to establish a connection.

## Platform Compatibility

Both the target node and the handshake code have been tested and compile on Linux.

## Code Independence

The solution does not depend on the code of the target node.

The submitted code does not reuse entire preexisting handshake implementations such as libp2p_noise/XX.

## Setup

1. Clone the repository 

`https://github.com/mrozdolski/p2p-handshake.git`

2. Navigate to the project directory:

`cd p2p-handshake`

3. Build the project:

`cargo build`

## Running the Project

You'll need to run two instances of the project simultaneously (representing two nodes) to observe the handshake process.

1. Start the first node:

`cargo run`

2. In a separate terminal, start the second node:

`cargo run`

## Verifying the Handshake

Once both nodes are running, they will automatically attempt to establish a handshake. To verify that the handshake has concluded successfully, look for the following messages in the terminal outputs:

**Initiating node (the one that initiates the handshake):**
- `Node handshake initiated and completed 🤝`

**Receiving node (the one that receives the handshake):**
- `Node handshake successful 🤝`

Both nodes will exchange a series of messages that make up the handshake protocol. Successful completion of the protocol is indicated by the messages mentioned above.

## Troubleshooting

- Ensure that no other processes are using ports 5000 and 5001, as these are required for the nodes to communicate.
- If you notice any "Idle waiting for a peer" messages, it means one of the nodes is waiting for the other to become available for the handshake. Ensure both nodes are running simultaneously.