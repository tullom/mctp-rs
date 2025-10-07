# mctp-rs

A `no_std` Rust implementation of the Management Component Transport Protocol (MCTP) as defined in the [DMTF DSP0236 specification](https://www.dmtf.org/sites/default/files/standards/documents/DSP0236_1.3.3.pdf).

## Documentation

Latest API docs: [https://dymk.github.io/mctp-rs/](https://dymk.github.io/mctp-rs/) (redirects to crate docs).

## Overview

MCTP is a communication protocol designed for platform management subsystems in computer systems. It facilitates communication between management controllers (like BMCs) and managed devices across various bus types. This library provides:

- **Protocol Implementation**: Complete MCTP transport layer with packet assembly/disassembly
- **Medium Abstraction**: Support for different physical transport layers (SMBus/eSPI included)
- **No-std Compatible**: Suitable for embedded and resource-constrained environments

## Features

- `espi` - Enables eSPI device support via the `espi-device` crate

## Usage

### Basic Message Handling

```rust
use mctp_rs::*;

// Create a packet context with a medium and assembly buffer
let mut assembly_buffer = [0u8; 1024];
let medium = /* your medium implementation */;
let mut context = MctpPacketContext::new(medium, &mut assembly_buffer);

// Receive and parse incoming packets
match context.receive_packet(raw_packet_data) {
    Ok(Some(message)) => {
        // Complete message received
        match message.header_and_body {
            MctpMessageHeaderAndBody::Control { header, body } => {
                // Handle MCTP control message
                println!("Received control command: {:?}", header.command_code);
            }
            MctpMessageHeaderAndBody::VendorDefinedPci { header, body } => {
                // Handle vendor-defined PCI message
            }
            MctpMessageHeaderAndBody::VendorDefinedIana { header, body } => {
                // Handle vendor-defined IANA message
            }
        }
    }
    Ok(None) => {
        // Partial message, waiting for more packets
    }
    Err(e) => {
        // Handle protocol or medium error
        eprintln!("Error receiving packet: {:?}", e);
    }
}
```

### Sending Messages

```rust
// Create a reply context (usually from a received message)
let reply_context = MctpReplyContext {
    destination_endpoint_id: EndpointId::Id(0x20),
    source_endpoint_id: EndpointId::Id(0x21),
    packet_sequence_number: MctpSequenceNumber::new(0),
    message_tag: MctpMessageTag::try_from(1).unwrap(),
    medium_context: (), // Medium-specific context
};

// Serialize a message into packets
let message_data = b"Hello MCTP!";
let mut packet_state = context.serialize_packet(reply_context, message_data)?;

// Send each packet
while let Some(packet_result) = packet_state.next() {
    let packet = packet_result?;
    // Send packet via your transport medium
    send_packet_via_transport(packet);
}
```

### Control Commands

```rust
use mctp_rs::control_command::*;

// Create a GetEndpointId request
let request = GetEndpointIdRequest;
let mut buffer = [0u8; 64];
let serialized = request.serialize(&mut buffer)?;

// Parse a GetEndpointId response
let response = GetEndpointIdResponse::deserialize(response_data)?;
println!("Endpoint ID: {:?}", response.endpoint_id);
println!("Endpoint Type: {:?}", response.endpoint_type);
```

### Implementing Custom Mediums

```rust
use mctp_rs::medium::*;

#[derive(Debug, Clone, Copy)]
struct MyMedium {
    mtu: usize,
}

#[derive(Debug, Clone, Copy)]
struct MyMediumFrame {
    packet_size: usize,
}

impl MctpMedium for MyMedium {
    type Frame = MyMediumFrame;
    type Error = &'static str;
    type ReplyContext = ();

    fn max_message_body_size(&self) -> usize {
        self.mtu
    }

    fn deserialize<'buf>(
        &self,
        packet: &'buf [u8],
    ) -> MctpPacketResult<(Self::Frame, &'buf [u8]), Self> {
        // Parse medium-specific headers and return MCTP payload
        Ok((MyMediumFrame { packet_size: packet.len() }, packet))
    }

    fn serialize<'buf, E, F>(
        &self,
        _reply_context: Self::ReplyContext,
        buffer: &'buf mut [u8],
        message_writer: F,
    ) -> MctpPacketResult<&'buf [u8], Self>
    where
        F: for<'a> FnOnce(&'a mut [u8]) -> MctpPacketResult<usize, Self>,
    {
        // Write medium-specific headers, call message_writer for MCTP data
        let message_len = message_writer(buffer)?;
        Ok(&buffer[..message_len])
    }
}

impl MctpMediumFrame<MyMedium> for MyMediumFrame {
    fn packet_size(&self) -> usize {
        self.packet_size
    }

    fn reply_context(&self) -> <MyMedium as MctpMedium>::ReplyContext {
        ()
    }
}
```

## Architecture

The library is structured around:

- **`MctpPacketContext`**: Main entry point for handling MCTP packets
- **`MctpMedium`**: Trait for implementing transport-specific packet handling
- **`MctpMessage`**: Represents a complete MCTP message with reply context
- **Control Commands**: Type-safe implementation of MCTP control protocol


## License

MIT License - see [LICENSE.md](LICENSE.md) for details.

## Contributing

1. Ensure `cargo check` and `cargo test` pass
2. Test with all feature combinations using `cargo hack --feature-powerset check`
3. Maintain `no_std` compatibility
4. Follow the existing code patterns for protocol message handling
