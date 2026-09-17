# QTest Web Examples

This directory contains examples demonstrating how to use the `qtest-web` crate.

## Basic Server

**File:** `basic_server.rs`

A basic web server example that provides HTTP API and WebSocket endpoints for interacting with QEMU via QTest protocol.

### Running

```bash
cargo run --example basic_server
```

### Configuration

The server can be configured using environment variables or command-line arguments:

```bash
# Using environment variables
LOG_LEVEL=info API_URL=0.0.0.0:8080 WS_URL=0.0.0.0:8081 QTEST_URL=localhost:3000 \
  cargo run --example basic_server

# Using command-line arguments
cargo run --example basic_server -- \
  --log-level info \
  --api-url 0.0.0.0:8080 \
  --ws-url 0.0.0.0:8081 \
  --qtest-url localhost:3000
```

### Options

- `--log-level`: Set logging verbosity (trace, debug, info, warn, error). Default: `debug`
- `--api-url`: HTTP API server address. Default: `127.0.0.1:8080`
- `--ws-url`: WebSocket server address. Default: `127.0.0.1:8081`
- `--qtest-url`: QTest socket address. Default: `localhost:3000`

### Features

- **HTTP API**: RESTful endpoints for clock control and memory operations
- **WebSocket Server**: Real-time IRQ notifications
- **QEMU Integration**: Connects to QEMU via QTest protocol

### Usage with Custom Platform

To use this server with a custom platform (e.g., STM32F4), see the `qtest-stm32f4` crate examples.
