# Sloppy Chain P2P

## Features

### Peer Discovery
Node should be able to discover peers given a set of existing peers. Node will:
1. Ask existing peers for their peers using a _Depth First Search (DFS)_ until `PEER_LIMIT` is maxed out or no other peers can be found.
2. Maintain a `peer_list` where inactive peers are removed and cycled
3. 



## Resource(s)
- [Unresponsive BTC Nodes](http://www.petecorey.com/blog/2018/07/09/ping-pong-and-unresponsive-bitcoin-nodes/)
- 