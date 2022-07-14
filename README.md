# Asynchronous Modbus TCP Server in Rust with Tokio and rmodbus

This code uses Tokio's TcpListener and the rmodbus library to make an async TCP
modbus server in Rust. It can handle concurrent connections using Tokio's green
threads.

## Usage

By default, this listens for modbus client connections on port `5502`.

```
$ cargo build
$ cargo run
Listening on 0.0.0.0:5502
Client connected: 127.0.0.1:38758
16-bit registers from address 0 to 9: [0, 0, 200, 214, 249, 169, 0, 0, 0, 0]
Responding to client 127.0.0.1:38758 with: [c0, 48, 0, 0, 0, 6, 1, 10, 0, 2, 0, 4]
16-bit registers from address 0 to 9: [0, 0, 200, 214, 249, 169, 0, 0, 0, 0]
Responding to client 127.0.0.1:38758 with: [44, 25, 0, 0, 0, b, 1, 3, 8, 0, c8, 0, d6, 0, f9, 0, a9]
16-bit registers from address 0 to 9: [0, 0, 200, 214, 249, 169, 0, 0, 0, 0]
Responding to client 127.0.0.1:38758 with: [8e, 37, 0, 0, 0, b, 1, 4, 8, 0, 0, 0, 0, 0, 0, 0, 0]
Client connected: 127.0.0.1:38770
16-bit registers from address 0 to 9: [0, 0, 233, 80, 160, 89, 0, 0, 0, 0]
Responding to client 127.0.0.1:38770 with: [22, ed, 0, 0, 0, 6, 1, 10, 0, 2, 0, 4]
16-bit registers from address 0 to 9: [0, 0, 233, 80, 160, 89, 0, 0, 0, 0]
Responding to client 127.0.0.1:38770 with: [a8, d9, 0, 0, 0, b, 1, 3, 8, 0, e9, 0, 50, 0, a0, 0, 59]
16-bit registers from address 0 to 9: [0, 0, 233, 80, 160, 89, 0, 0, 0, 0]
```

## Development

Run `cargo fmt` to format the code and `cargo clippy` to lint the code.
