<table>
<tr>
<td align="left" valign="middle">
<h3 align="left">Vine</h3>
</td>
<td align="left" valign="middle">
<h3 align="left">🌿</h3>
</td>
<td align="left" valign="middle">
<h3 align="left">+</h3>
</td>
<td align="left" valign="middle">
<h3 align="left">
<a href="https://Editor.Land" target="_blank">
<picture>
<source media="(prefers-color-scheme: dark)" srcset="https://PlayForm.Cloud/Dark/Image/GitHub/Land.svg">
<source media="(prefers-color-scheme: light)" srcset="https://PlayForm.Cloud/Image/GitHub/Land.svg">
<img width="28" alt="Land Logo" src="https://PlayForm.Cloud/Image/GitHub/Land.svg">
</picture>
</a>
</h3>
</td>
<td align="left" valign="middle">
<h3 align="left">
<a href="https://Editor.Land" target="_blank">
Land
</a>
</h3>
</td>
<td align="left" valign="middle">
<h3 align="left">🏞️</h3>
</td>
</tr>
</table>

---

# **Vine** 🌿

The gRPC Protocol Layer for Land 🏞️

[![License: CC0-1.0](https://img.shields.io/badge/License-CC0_1.0-lightgrey.svg)](https://github.com/CodeEditorLand/Vine/tree/Current/LICENSE)
[![Status](https://img.shields.io/badge/Status-Under%20Development-yellow.svg)](https://github.com/CodeEditorLand/Vine)

**Vine** is the gRPC protocol definition and communication specification for the
**Land Code Editor** ecosystem. It defines the strongly-typed IPC layer used for
communication between `Mountain` (Rust backend), `Cocoon` (Node.js extension
host), and the planned `Grove` (Rust/WASM extension host).

**Vine** is engineered to:

1. **Define Protocol Contracts:** Provide `.proto` files that specify the gRPC
   service definitions for all inter-component communication.
2. **Enable Strong Typing:** Ensure type-safe communication through Protocol
   Buffers and generated Rust/TypeScript code.
3. **Support Multiple Transports:** Design for transport agnosticism with
   support for TCP, IPC, and WASM host functions.
4. **Implement Health Monitoring:** Provide heartbeat and connection state
   management for reliable communication.

---

## Key Features 🔐

- **Protocol Buffer Definitions:** `.proto` files specifying gRPC service
  definitions for all inter-component communication.
- **Strong Typing:** Type-safe communication through Protocol Buffers with
  generated Rust and TypeScript code.
- **Transport Agnosticism:** Designed for multiple transport backends including
  TCP, IPC, and WASM host functions.
- **Health Monitoring:** Built-in heartbeat and connection state management for
  reliable communication.
- **Spine Protocol:** Extension host coordination using action/response pattern
  for command execution.

---

## Core Architecture Principles 🏗️

| Principle                 | Description                                                                         | Key Components Involved                   |
| :------------------------ | :---------------------------------------------------------------------------------- | :---------------------------------------- |
| **Contract-First**        | Define all service interfaces in `.proto` files before implementation.              | `Proto/*.proto`, protocol buffer compiler |
| **Type Safety**           | Generate strongly-typed code from protocol definitions for compile-time guarantees. | Generated Rust/TypeScript code            |
| **Transport Agnosticism** | Design protocol layer independent of specific transport implementation.             | `Transport` trait, strategy pattern       |
| **Health Awareness**      | Built-in connection monitoring and heartbeat for reliability.                       | Health check messages, timeout handling   |

---

## `Vine` in the Land Ecosystem 🌿 + 🏞️

| Component          | Role & Key Responsibilities                                  |
| :----------------- | :----------------------------------------------------------- |
| **gRPC Server**    | Hosted by `Mountain` for extension host communication.       |
| **gRPC Client**    | Used by `Cocoon` and `Grove` to communicate with `Mountain`. |
| **Spine Protocol** | Extension host coordination layer for command execution.     |

---

## Getting Started 🚀

### Current Status 📊

`Vine` is currently a placeholder for the gRPC protocol definitions. The actual
protocol implementation resides in:

- **`Mountain`:** gRPC server implementation in `Vine/` directory
- **`Cocoon`:** gRPC client implementation in `Services/MountainGRPCClient.ts`

### Future Usage 🚀

When fully implemented, `Vine` will be used as:

```toml
[dependencies]
Vine = { git = "https://github.com/CodeEditorLand/Vine.git", branch = "Current" }
```

**Key Dependencies:**

- `tonic` — Rust gRPC framework
- `prost` — Protocol Buffers implementation
- `@grpc/grpc-js` — Node.js gRPC client (for Cocoon)

---

## Status 📊

⚠️ **Under Development** - This component is currently a placeholder. The
protocol definitions and implementation are in progress.

## Overview 📖

Vine defines the gRPC-based communication protocol used between components in
the Land architecture:

- **Mountain** ↔ **Cocoon** communication
- **Mountain** ↔ **Grove** communication
- **Spine** protocol for extension host coordination

## Planned Features

- **gRPC Service Definitions:** Protocol buffer (`.proto`) files for all
  inter-component communication
- **Spine Protocol:** Extension host connection protocol (action/response
  pattern)
- **Health Monitoring:** Heartbeat and connection state management
- **Message Types:** Structured message formats for commands, events, and
  responses
- **Transport Agnostic:** Support for multiple transport layers (TCP, IPC, WASM)

## Protocol Structure (Planned)

```
Element/Vine/
├── Proto/
│   ├── Vine.proto # Core protocol definitions
│   ├── Spine.proto # Extension host protocol
│   └── Grove.proto # Grove-specific extensions
├── Source/
│   ├── lib.rs # Protocol library
│   ├── Message/ # Message type definitions
│   ├── Service/ # gRPC service implementations
│   └── Client/ # Protocol clients
└── Documentation/
  └── Protocol.md # Protocol specification
```

---

## System Architecture Diagram 🏗️

This diagram illustrates `Vine`'s planned role as the gRPC protocol layer in the
Land ecosystem.

```mermaid
graph LR
classDef vine fill:#f9f,stroke:#333,stroke-width:2px;
classDef mountain fill:#9cf,stroke:#333,stroke-width:2px;
classDef cocoon fill:#ccf,stroke:#333,stroke-width:2px;
classDef grove fill:#cfc,stroke:#333,stroke-width:1px;

subgraph "Mountain ⛰️ (Rust Backend)"
VineServer["Vine gRPC Server"]:::mountain
end

subgraph "Vine 🌿 (Protocol Layer)"
VineProto["Vine.proto"]:::vine
SpineProto["Spine.proto"]:::vine
GroveProto["Grove.proto"]:::vine
end

subgraph "Clients"
CocoonClient["Cocoon gRPC Client"]:::cocoon
GroveClient["Grove gRPC Client"]:::grove
end

VineServer --> VineProto
VineProto <--> CocoonClient
VineProto <--> GroveClient
```

## Related Components 🔗

- [**Mountain**](https://github.com/CodeEditorLand/Land/tree/Current/Documentation/Architecture/components/Mountain.md) -
  Core VS Code implementation (protocol client)
- [**Cocoon**](https://github.com/CodeEditorLand/Cocoon/tree/Current/) - Node.js
  Extension Host (protocol server)
- [**Grove**](../Grove/) - Rust/WASM Extension Host (protocol server)

---

## Deep Dive & Component Breakdown 🔬

**Vine** is currently a placeholder for the gRPC protocol definitions. When
implemented, the protocol structure will include:

- **[`Proto/`](Proto/)** - Protocol buffer definitions
    - [`Vine.proto`](Proto/Vine.proto) - Core Mountain↔Cocoon communication
    - [`Spine.proto`](Proto/Spine.proto) - Extension host coordination protocol
    - [`Grove.proto`](Proto/Grove.proto) - Grove-specific extensions
- **[`Source/`](https://github.com/CodeEditorLand/Vine/tree/Current/Source/)** -
  Rust implementation
- [`Message/`](https://github.com/CodeEditorLand/Vine/tree/Current/Source/Message/) -
  Message type definitions
- [`Service/`](https://github.com/CodeEditorLand/Vine/tree/Current/Source/Service/) -
  gRPC service implementations
- [`Client/`](https://github.com/CodeEditorLand/Vine/tree/Current/Source/Client/) -
  Protocol clients

For the current protocol specification, refer to the
[Spine Contract](https://github.com/CodeEditorLand/Land/tree/Current/Documentation/Architecture/integration/SpineContract.md)
documentation.

---

## Development Status 📊

| Feature           | Status                              |
| ----------------- | ----------------------------------- |
| Proto Definitions | ⏳ Planned                          |
| gRPC Services     | ⏳ Planned                          |
| Spine Protocol    | 📝 Specified (see SpineContract.md) |
| Health Monitoring | ⏳ Planned                          |
| Message Types     | ⏳ Planned                          |

## References 📚

- [Spine Contract](https://github.com/CodeEditorLand/Land/tree/Current/Documentation/Architecture/integration/SpineContract.md) -
  Extension host communication contract
- [Communication Flows](https://github.com/CodeEditorLand/Land/tree/Current/Documentation/Architecture/integration/CommunicationFlows.md) -
  Component communication patterns

## License ⚖️

This project is licensed under Creative Commons CC0.

See the LICENSE file for details.

---

## Changelog 📜

Stay updated with our progress! See
[`CHANGELOG.md`](https://github.com/CodeEditorLand/Vine/tree/Current/) for a
history of changes specific to **Vine**.

---

## Funding & Acknowledgements 🙏🏻

Code Editor Land is funded through the NGI0 Commons Fund, established by NLnet
with financial support from the European Commission's Next Generation Internet
programme, under grant agreement No. 101135429.

The project is operated by PlayForm, based in Sofia, Bulgaria.

PlayForm acts as the open-source steward for Code Editor Land under the NGI0
Commons Fund grant.

<table>
	<thead>
		<tr>
			<th align="left"><strong>Land</strong></th>
			<th align="left"><strong>PlayForm</strong></th>
			<th align="left"><strong>NLnet</strong></th>
			<th align="left"><strong>NGI0 Commons Fund</strong></th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td align="left" valign="middle">
				<a href="https://Editor.Land">
					<img width="60" src="https://raw.githubusercontent.com/CodeEditorLand/Asset/refs/heads/Current/Logo/Land.svg" alt="Land">
				</a>
			</td>
			<td align="left" valign="middle">
				<a href="https://PlayForm.Cloud">
					<img width="76" src="https://raw.githubusercontent.com/PlayForm/Asset/refs/heads/Current/Logo/PlayForm.svg" alt="PlayForm">
				</a>
			</td>
			<td align="left" valign="middle">
				<a href="https://NLnet.NL">
					<img width="240" src="https://NLnet.NL/logo/banner.svg" alt="NLnet">
				</a>
			</td>
			<td align="left" valign="middle">
				<a href="https://NLnet.NL/commonsfund">
					<img width="240" src="https://NLnet.NL/image/logos/NGI0CommonsFund_tag_black_mono.svg" alt="NGI0 Commons Fund">
				</a>
			</td>
		</tr>
	</tbody>
</table>

---

**Project Maintainers**: Source Open
([Source/Open@Editor.Land](mailto:Source/Open@Editor.Land)) |
[GitHub Repository](https://github.com/CodeEditorLand/Vine) |
[Report an Issue](https://github.com/CodeEditorLand/Vine/issues) |
[Security Policy](https://github.com/CodeEditorLand/Vine/security/policy)
