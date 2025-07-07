# mctp-rs

A `no_std` MCTP (Management Component Transport Protocol) implementation in Rust.

## ⚠️ Current Implementation: Static EID Allocation Only

This implementation currently supports **static EID allocation only**. This means:

- Endpoint IDs (EIDs) are pre-configured and known at initialization time
- No dynamic discovery or EID assignment is performed
- Suitable for embedded systems, test environments, and controlled deployments
- Provides a solid foundation for basic MCTP communication

## Features

- ✅ `no_std` compatible with `heapless` collections
- ✅ Async-first API design
- ✅ Transport-agnostic (I2C, PCIe, eSPI, etc.)
- ✅ Manual packet serialization for portability
- ✅ Type-safe endpoint IDs and message tags
- ✅ Message fragmentation and reassembly
- ✅ Basic MCTP Control protocol support
- ✅ Client and Host endpoint APIs
- ⚠️ Static EID allocation only (no dynamic discovery)

## Cargo Features

- `client` (default): Enable `ClientEndpoint` for managed devices
- `host` (default): Enable `Endpoint` as `HostEndpoint` for management controllers
- `bridge` (default): Enable `Bridge` for multi-transport routing

## Quick Start

### Client Endpoint (Managed Device)

```rust
use mctp_rs::{ClientEndpoint, MessageHandler, MessageView, EndpointId};

struct MyHandler;

impl MessageHandler for MyHandler {
    async fn handle_application_message(
        &mut self,
        message_type: u8,
        request: MessageView,
        response_buf: &mut [u8],
    ) -> Option<usize> {
        match message_type {
            0x01 => {
                // Handle application-specific message
                response_buf[0] = 0x01; // Response message type
                response_buf[1] = 0x00; // Success
                Some(2) // Response length
            }
            _ => None, // No response
        }
    }

    async fn on_eid_assigned(&mut self, new_eid: EndpointId) {
        // Handle EID assignment notification
    }
}

// Usage
let transport = MyTransport::new(); // Your transport implementation
let endpoint = Endpoint::new(transport, EndpointId::new(42));
let handler = MyHandler;
let mut client = ClientEndpoint::new(endpoint, handler);

// Run the client event loop
let mut rx_buffer = [0u8; 1024];
let mut tx_buffer = [0u8; 1024];
client.listen(&mut rx_buffer, &mut tx_buffer).await; // Never returns
```

### Host Endpoint (Management Controller)

```rust
use mctp_rs::{endpoint::Endpoint, EndpointId};

// Usage
let transport = MyTransport::new(); // Your transport implementation
let mut host = Endpoint::new(transport, EndpointId::new(1));

// Send a message to a managed device
let message = [0x01, 0x02, 0x03]; // Your message
match host.send_message(EndpointId::new(42), &message).await {
    Ok(()) => println!("Message sent successfully"),
    Err(e) => println!("Send failed: {:?}", e),
}

// Receive a message
let mut response_buf = [0u8; 1024];
match host.receive_message(&mut response_buf).await {
    Ok(response) => {
        println!("Received message from EID {}: {:?}",
                 response.source_eid.0, response.payload);
    }
    Err(e) => println!("Receive failed: {:?}", e),
}
```

### Transport Implementation

Implement the `Transport` trait for your physical medium:

```rust
use mctp_rs::Transport;

struct MyI2cTransport {
    // Your I2C implementation
}

impl Transport for MyI2cTransport {
    type Error = MyI2cError;
    type PhysicalAddress = u8; // I2C address
    const MTU: usize = 64; // I2C MTU

    async fn send_packet(&mut self, packet: &[u8]) -> Result<(), Self::Error> {
        // Send packet over I2C
    }

    async fn receive_packet(&mut self, buffer: &mut [u8]) -> Result<usize, Self::Error> {
        // Receive packet from I2C, return packet length
    }
}
```

## Architecture

The library is organized into several layers:

1. **Core Data Structures** (`messages.rs`): `PacketHeader`, `EndpointId`, etc.
2. **Transport Abstraction** (`transport.rs`): Async trait for different physical media
3. **Endpoint Logic** (`endpoint.rs`): Message assembly/disassembly and reassembly
4. **Client API** (`client.rs`): High-level API for managed devices
5. **Host API** (`host.rs`): High-level API for management controllers
6. **Bridge API** (`bridge.rs`): Multi-transport routing (future)

## Static EID Configuration

Since this implementation uses static EID allocation, you need to:

1. **Pre-assign EIDs**: Each device must have a unique EID (1-254)
2. **Configure the Host**: The management controller must know all device EIDs
3. **No Discovery**: Devices don't announce themselves or get assigned EIDs dynamically

### Example Network Configuration

```rust
// Management Controller (Bus Owner)
const HOST_EID: u8 = 1;

// Managed Devices (pre-configured)
const DEVICE_A_EID: u8 = 10;
const DEVICE_B_EID: u8 = 11;
const SENSOR_EID: u8 = 20;
```

## Limitations

- **No Hot-Plug Support**: Devices must be known at initialization
- **No Dynamic Discovery**: No automatic device detection
- **Manual EID Management**: EID assignments must be managed manually
- **No Bridge Routing**: Multi-transport bridging not yet implemented

## Future Roadmap

- [ ] Dynamic EID allocation and discovery
- [ ] Hot-plug device support
- [ ] Advanced routing and bridging
- [ ] Transport-specific crates (`mctp-i2c`, `mctp-pcie`)
- [ ] PLDM message type support
- [ ] Message integrity checking

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.
