# rustchat

A chat server made in Rust that communicates using an underlying Onion network.

## Connection with peers

The server uses UDP for transmitting data, as Onion is by nature an insecure network. The server doesn't store
messages, because they are peer-to-peer encrypted, but works as a forwarder between clients.

### Packet format

The maximum size for a packet is 512 bytes, excluding the UDP header. Longer packets will be dropped and the
incoming IP may be blocked.

The packet format is as follows:
+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
| 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 |10 |11 |12 |13 |14 |15 |16 |17 |
+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
| Header (1 byte) | Sender ID (16 bytes) |
+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
| Timestamp (8 bytes) |
+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
| Message ID (8 bytes) |
+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
| Message Content (variable) |
+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
