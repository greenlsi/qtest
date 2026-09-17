# WebServer Example

This example demonstrates how to deploy a web server for testing STM32F4 peripherals using QTest.

## Features

- **HTTP API**: RESTful API for peripheral access and control
- **WebSocket Server**: Real-time IRQ notifications
- **QEMU Integration**: Connects to QEMU via QTest protocol

## Building

To build the example with the required features:

```bash
cargo build --example webserver --features qtest-web
```

## Running

Run the example with default settings:

```bash
cargo run --example webserver --features qtest-web
```

Or customize the configuration using environment variables or command-line arguments:

```bash
# Using environment variables
LOG_LEVEL=info API_URL=0.0.0.0:8080 WS_URL=0.0.0.0:8081 QTEST_URL=localhost:3000 \
  cargo run --example webserver --features qtest-web

# Using command-line arguments
cargo run --example webserver --features qtest-web -- \
  --log-level info \
  --api-url 0.0.0.0:8080 \
  --ws-url 0.0.0.0:8081 \
  --qtest-url localhost:3000
```

## Configuration Options

- `--log-level`: Logging verbosity (trace, debug, info, warn, error). Default: `debug`
- `--api-url`: HTTP API server address. Default: `127.0.0.1:8080`
- `--ws-url`: WebSocket server address. Default: `127.0.0.1:8081`
- `--qtest-url`: QTest socket address. Default: `localhost:3000`

## API Endpoints

### General Endpoints

- `GET /status` - Check if QEMU session is connected
- `POST /clock_step?ns=<nanoseconds>` - Step the clock
- `POST /clock_set?ns=<nanoseconds>` - Set the clock

### STM32F4-Specific Endpoints

- `GET /stm32f4/gpio_info/{gpio_id}?parts=<pin_numbers>` - Get GPIO peripheral information
  - Example: `/stm32f4/gpio_info/gpioa?parts=0&parts=1&parts=5`
  
- `GET /stm32f4/timer_info/{timer_id}?parts=<channel_numbers>` - Get Timer peripheral information
  - Example: `/stm32f4/timer_info/tim2?parts=0&parts=1`

### Memory Operations

- `GET /readb?addr=<address>` - Read byte
- `GET /readw?addr=<address>` - Read word (16-bit)
- `GET /readl?addr=<address>` - Read long (32-bit)
- `GET /readq?addr=<address>` - Read quad (64-bit)
- `POST /writeb?addr=<address>&val=<value>` - Write byte
- `POST /writew?addr=<address>&val=<value>` - Write word
- `POST /writel?addr=<address>&val=<value>` - Write long
- `POST /writeq?addr=<address>&val=<value>` - Write quad

## WebSocket

Connect to the WebSocket server to receive real-time IRQ notifications:

```javascript
const ws = new WebSocket('ws://127.0.0.1:8081');
ws.onmessage = (event) => {
  const irq = JSON.parse(event.data);
  console.log('IRQ received:', irq);
};
```

## Usage with QEMU

Start QEMU with QTest enabled:

```bash
qemu-system-arm -M netduinoplus2 \
  -qtest unix:/tmp/qtest.sock,server,wait=off \
  -qtest-log /dev/null \
  -nographic \
  -kernel your_firmware.elf
```

Then connect the web server to QEMU (if using the default socket path, update `--qtest-url` accordingly).

## Example Requests

```bash
# Check connection status
curl http://127.0.0.1:8080/status

# Get GPIO A information for pins 0, 1, and 5
curl "http://127.0.0.1:8080/stm32f4/gpio_info/gpioa?parts=0&parts=1&parts=5"

# Get Timer 2 information for channels 0 and 1
curl "http://127.0.0.1:8080/stm32f4/timer_info/tim2?parts=0&parts=1"

# Read from address 0x40020000 (32-bit)
curl -X POST "http://127.0.0.1:8080/readl?addr=0x40020000"

# Write to address 0x40020000 (32-bit value 0x12345678)
curl -X POST "http://127.0.0.1:8080/writel?addr=0x40020000&val=0x12345678"

# Step the clock by 1000 nanoseconds
curl -X POST "http://127.0.0.1:8080/clock_step?ns=1000"
```
