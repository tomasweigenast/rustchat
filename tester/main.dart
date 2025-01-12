import 'dart:convert';
import 'dart:io';
import 'dart:math';
import 'dart:typed_data';

Future<void> main() async {
  final destinationAddr = InternetAddress("127.0.0.1");
  final destinationPort = 7878;
  final sourcePort = 1024 + Random().nextInt(9999);
  final socket = await Socket.connect(
    destinationAddr,
    destinationPort,
    sourcePort: sourcePort,
    timeout: const Duration(seconds: 3),
  );

  socket.listen(_onMessage);

  // final packet = Packet.message(message: "hi");
  // print("Send packet: $packet");
  // socket.add(packet.encode());
  stdin.transform(utf8.decoder).listen((input) {
    final message = input.trim();
    if (message.isNotEmpty) {
      try {
        final packet = Packet.message(message: message);
        print("Send packet: $packet");
        socket.add(packet.encode());
      } catch (err) {
        print(err);
      }
    }
  });

  await socket.done;
}

void _onMessage(Uint8List buffer) {
  print("Message received: $buffer");
}

const kMaxPacketSize = 1024 * 1024 * 12;

class Packet {
  final PacketType type;
  final Uint8List payload;

  const Packet({required this.type, required this.payload});

  factory Packet.message({required String message}) {
    return Packet(type: PacketType.message, payload: utf8.encode(message));
  }

  Uint8List encode() {
    final payloadSize = 1 + payload.lengthInBytes;
    final buffer = Uint8List(5 + payload.lengthInBytes);

    final byteData = ByteData.sublistView(buffer);
    byteData.setUint32(0, payloadSize, Endian.big);

    buffer[4] = type.index;

    buffer.setRange(5, buffer.lengthInBytes, payload);

    if (buffer.lengthInBytes > kMaxPacketSize) {
      throw "Message too big.";
    }

    return buffer;
  }

  @override
  String toString() => "Packet(type: $type, payload: ${payload.sublist(0, min(20, payload.length - 1))})";
}

enum PacketType { unknown, signIn, signOut, message }
