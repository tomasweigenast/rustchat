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

  stdin.transform(utf8.decoder).listen((input) {
    final message = input.trim();
    if (message.isNotEmpty) {
      final packet = createPacket(message);
      print("Going to send: $packet");
      socket.add(packet);
    }
  });
}

void _onMessage(Uint8List buffer) {
  print("Message received: $buffer");
}

const kMaxPacketSize = 576;
Uint8List createPacket(String content) {
  final payload = utf8.encode(content);
  final payloadSize = 2 + payload.lengthInBytes;

  final builder = BytesBuilder(copy: false);

  // payload size
  builder.add(Uint8List(4)..buffer.asByteData().setUint32(0, payloadSize, Endian.big));

  // packet id
  builder.addByte(1);

  // control bits
  builder.addByte(0);

  // payload
  builder.add(payload);

  if (builder.length > kMaxPacketSize) {
    throw "Message too big.";
  }

  return builder.toBytes();
}
