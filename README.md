# rustchat

A chat server made in Rust that communicates using an underlying Onion network.

## Connection with peers

The server uses UDP for transmitting data, as Onion is by nature an insecure network. The server doesn't store
messages, because they are peer-to-peer encrypted, but works as a forwarder between clients.

### Packet format

The maximum size for a packet is 512 bytes, excluding the UDP header. Longer packets will be dropped and the
incoming IP may be blocked.

The packet format is as follows:

```plaintext
+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
| 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 |10 |11 |12 |13 |14 |15 |16 |17 |18 |19 |20 |21 |22 |23 |
+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
|            Packet Id          |                           Payload Size                        |
+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
|                          Seq/Ack Number                       |           Control Bits        | 
+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
|                                           Payload                                             |
+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
|                                             ...                                               |
+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
```
