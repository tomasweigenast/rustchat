import asyncio

async def client(host, port, local_port):
    print("connecting to ",host,":",port," with port: ", local_port)

    local_address = ('127.0.0.1', local_port)
    reader, writer = await asyncio.open_connection(host, port)
    print("port: ", local_port, " connected.")
    # Send data to the server
    writer.write(b'Hello, server!')
    await writer.drain()

    print("data sent, waiting response")

    # # Receive data from the server
    data = await reader.read(-1)
    print(f'Received: [{data.decode()}] (plain bytes: [{data}])')

    # writer.close()
    await writer.wait_closed()

async def main():
    num_clients = 1  # Adjust the number of concurrent clients as needed
    server_host = '127.0.0.1'  # Replace with your server's host
    server_port = 7878  # Replace with your server's port

    # Assign unique local ports to each client
    local_ports = range(9000, 9001) #range(9000, 9010)

    await client(server_host, server_port, 9999)
    # tasks = [client(server_host, server_port, local_port) for local_port in local_ports]
    # await asyncio.gather(*tasks)

if __name__ == '__main__':
    asyncio.run(main())