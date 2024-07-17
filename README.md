# rustchat

A chat server made in Rust that communicates using an underlying Onion network.

## Connection with peers

The server uses UDP for transmitting data, as Onion is by nature an insecure network. The server doesn't store
messages, because they are peer-to-peer encrypted, but works as a forwarder between clients.

### Packet format

The maximum size for a packet is 876 bytes, excluding the UDP header. Longer packets will be dropped and the
incoming IP may be blocked. The header has 12 bytes, so 564 bytes are available for the payload.

The packet format is as follows:

```plaintext
+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
|               1               |               2               |               3               |               4               | *OCTECTS*
+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
| 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 |10 |11 |12 |13 |14 |15 |16 |17 |18 |19 |20 |21 |22 |23 |24 |25 |26 |27 |28 |29 |30 |31 | *BITS*
+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
|         Packet Type Id        |                           Payload Size                        | 0 | 0 | 0 | 0 | 0 |FRG|ENC|ACK|
+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
|                                                          Seq/Ack Number                                                       |
+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
|            Packet Id          |                                                  RESERVED                                     |
+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
|                                                               MAC                                                             |
+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
|                                                              Payload                                                          |
+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
|                                                                ...                                                            |
+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
```

**Fields Legend**

- **Packet Type Id**: Contains the id of the packet type that is sent.
- **Payload Size**: Indicates the size of the payload field.
- **Control Bits**:
  - **ACK**: Indicates if the packet carries an ACK.
  - **ENC**: Indicates if the packet payload is encrypted.
  - **FRG**: Indicates if the packet is fragmented.
- **Seq/Ack Number**: The sequence number of the packet or the sequence number acked, if the ACK bit is set.
- **MAC**: Carries the [MAC](https://en.wikipedia.org/wiki/Message_authentication_code) of the packet.
- **Payload**: The actual payload of the packet. 

### Data Types 
| Data Type | Byte Length        | Encoding Format                              | Description                                                       |
|-----------|--------------------|----------------------------------------------|-------------------------------------------------------------------|
| string    | UTF-8 bytes length | String length as varint + UTF-8 encoded data | The length of the string is encoded as a varint, followed by the UTF-8 encoded string data. |
| boolean   | 1 byte             | **0x01** for true, **0x00** for false                | Booleans are represented by a single byte: **0x01** for true and **0x00** for false. |
| int8      | 1 byte             | Raw bytes                                    | 8-bit signed integer.                                             |
| int16     | 2 bytes            | Raw bytes                                    | 16-bit signed integer, big-endian format.                      |
| int32     | 4 bytes            | Raw bytes                                    | 32-bit signed integer, big-endian format.                      |
| int64     | 8 bytes            | Raw bytes                                    | 64-bit signed integer, big-endian format.                      |
| uint8     | 1 byte             | Raw bytes                                    | 8-bit unsigned integer.                                           |
| uint16    | 2 bytes            | Raw bytes                                    | 16-bit unsigned integer, big-endian format.                    |
| uint32    | 4 bytes            | Raw bytes                                    | 32-bit unsigned integer, big-endian format.                    |
| uint64    | 8 bytes            | Raw bytes                                    | 64-bit unsigned integer, big-endian format.                    |
| varint    | up to 4 bytes            | Raw bytes                                    | A varying length, unsigned integer that is up to 4 bytes, big-endian format.                    |
| float32   | 4 bytes            | Raw bytes                                    | 32-bit floating point number (IEEE 754 standard), big-endian format. |
| float64   | 8 bytes            | Raw bytes                                    | 64-bit floating point number (IEEE 754 standard), big-endian format. |


### Encryption
Packets are encrypted two times:

- First encryption is used to send the data to the bridge server. It is encrypted with the public key of the server.
- Second encryption is used when packets are transmitting messages. The second encryption is done peer-to-peer: encrypted on the source and decrypted on the destination. 

#### First Encryption
The first packet encryption covers the whole packet without the packet id that is processed by the server first. Althought it is visible for anyone who is seeing the channel, the server can detect malicious changes by inspecting the MAC code of the packet, which is calculated on the client using a shared secret.

#### Second Encryption
The second layer of encryption only covers a few packet types, which send sensitive information, such as chat messages, photos or videos. They are encrypted using peer-to-peer. The bit `ENC` indicates if the packet contains this second layer of encryption
or not. **Not finished yet.**