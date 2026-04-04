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

# **Vine**&#x2001;🌿

> **Electron's IPC is untyped. You send a string event name and hope the handler matches. Refactoring a message field breaks things silently in production. There is no schema validation at the wire.**

_"Change a message field and every consumer breaks loudly at compile time, not silently at runtime."_

[![License: CC0-1.0](https://img.shields.io/badge/License-CC0_1.0-lightgrey.svg)](https://github.com/CodeEditorLand/Vine/tree/Current/LICENSE)
[<img src="https://editor.land/Image/Rust.svg" width="14" alt="Rust" />](https://www.rust-lang.org/)&#x2001;[![Status](https://img.shields.io/badge/Status-Under%20Development-yellow.svg)](https://github.com/CodeEditorLand/Vine)

Every inter-process service interface starts as a `.proto` file. The generated Rust and TypeScript stubs are the only way Land processes communicate. gRPC over a Unix domain socket runs at native memory-copy speed: microseconds for any message under 64KB. Changing a message field breaks every consumer at compile time.

---

## What It Does&#x2001;🔐

- **Typed at the wire.** Every message is a `.proto` contract compiled to Rust and TypeScript stubs.
- **Compile-time safety.** Change a field and every consumer breaks at the compiler, not in production.
- **Microsecond latency.** gRPC over Unix domain socket runs at memory-copy speed.
- **Versioned protocol.** The `.proto` file is the single source of truth for all IPC.

---

## In the Ecosystem&#x2001;🌿 + 🏞️

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

---

## Development&#x2001;🛠️

Vine is a component of the Land workspace. Follow the
[Land Repository](https://github.com/CodeEditorLand/Land) instructions to
build and run.

---

## License&#x2001;⚖️

CC0 1.0 Universal. Public domain. No restrictions.
[LICENSE](https://github.com/CodeEditorLand/Vine/tree/Current/LICENSE)

---

## See Also

- [Vine Documentation](https://editor.land/Doc/vine)
- [Architecture Overview](https://editor.land/Doc/architecture)
- [Why gRPC](https://editor.land/Doc/why-grpc)
- [Mountain](https://github.com/CodeEditorLand/Mountain)
- [Cocoon](https://github.com/CodeEditorLand/Cocoon)


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
