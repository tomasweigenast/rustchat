import 'dart:convert';
import 'dart:io';
import 'dart:typed_data';

Future<void> main() async {
  final destinationAddr = InternetAddress("127.0.0.1");
  final destinationPort = 7878;
  final randomPort = 2556;
  final socket = await RawDatagramSocket.bind(InternetAddress.anyIPv4, randomPort);
  socket.handleError((err, stack) {
    print("Error on socket: $err");
    print(stack);
  });
  socket.where((event) => event == RawSocketEvent.read).listen(_onMessage);

  while (true) {
    final message = readline("Message:");
    if (message == null) continue;

    final packet = createPacket(message);
    print("Going to send: $packet");
    await socket.send(packet, destinationAddr, destinationPort);
  }
}

void _onMessage(RawSocketEvent event) {}

String? readline(String msg) {
  stdout.write("$msg ");
  return stdin.readLineSync();
}

const kMaxPacketSize = 576;
Uint8List createPacket(String content) {
  final payload = utf8.encode(content);
  final payloadSize = payload.lengthInBytes;

  final builder = BytesBuilder(copy: false);
  // packet id
  builder.addByte(1);

  // payload size
  builder.add(Uint8List(2)..buffer.asByteData().setUint16(0, payloadSize, Endian.big));

  // control bits
  builder.addByte(0);

  // sequence number
  builder.add(Uint8List(4)..buffer.asByteData().setUint32(0, DateTime.now().millisecondsSinceEpoch ~/ 1000, Endian.big));

  // mac number
  builder.add(Uint8List(4)..buffer.asByteData().setUint32(0, 1, Endian.big));

  // payload
  builder.add(payload);

  if (builder.length > kMaxPacketSize) {
    throw "Message too big.";
  }

  return builder.toBytes();
}
