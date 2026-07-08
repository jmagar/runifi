# OpenWiki Documentation Plan for rustifi

## Repository Analysis

**rustifi** is a Rust-based MCP (Model Context Protocol) server that bridges AI clients (Claude, Cursor, etc.) to Ubiquiti UniFi network controllers. It exposes both official and internal UniFi REST APIs through a unified MCP tool interface.

### Key Characteristics
- **Language**: Rust (1.86+)
- **Purpose**: UniFi network controller REST API bridge for MCP clients and CLI
- **APIs Supported**: Official Network Integration API + Internal V1/V2 site APIs
- **Transport**: HTTP MCP server + stdio MCP transport
- **Architecture**: Strict layering - UnifiClient → UnifiService → tools.rs/cli.rs thin shims

## Documentation Strategy

### Initial Page Set (focused, minimal)

Since this is a small-to-medium repository with clear domain boundaries, the documentation should be:

1. **openwiki/quickstart.md** - Single comprehensive entry point covering:
   - What rustifi is and what it does
   - Quickstart for CLI use
   - Quickstart for MCP server (HTTP + stdio)
   - Links to detailed external docs in /docs directory
   - Brief architecture overview
   - Key source references

### Why This Structure Works

- **Single repository focus**: rustifi has one clear domain (UniFi API bridge)
- **Existing comprehensive docs**: The /docs directory already contains extensive documentation (CLI.md, CONFIG.md, SETUP.md, OAUTH.md, etc.)
- **No need to duplicate**: OpenWiki should summarize and link, not replace existing docs
- **Small repository**: ~30-40 source files, single binary, clear architecture

## openwiki/quickstart.md Content Plan

### Section 1: Repository Overview
- What rustifi is (UniFi MCP server)
- What it does (bridges AI clients to UniFi controllers)
- Key APIs (official Network Integration + internal site APIs)
- Binary name: `runifi`

### Section 2: Quick Start
- Prerequisites (Rust 1.86+, UniFi controller, API key)
- Environment setup (.env.example)
- CLI commands (health, clients, devices, etc.)
- MCP server startup (HTTP and stdio modes)

### Section 3: Architecture Overview
- Strict layering: UnifiClient → UnifiService → thin shims
- API families: official, internal, hybrid
- Action dispatch system

### Section 4: Key Concepts
- UniFi controller types (UDM vs legacy)
- TLS verification requirements
- Auth modes (bearer, OAuth, loopback)
- Action families (official_*, unifi_*, hybrid)

### Section 5: Development Workflow
- How to add a new action
- Test strategy (unit tests + live smoke tests)
- Cargo workspace structure (main + xtask)

### Section 6: Further Reading (links)
- /docs/SETUP.md - Full setup guide
- /docs/CLI.md - Complete CLI reference
- /docs/CONFIG.md - All configuration options
- /docs/mcp/ - MCP server internals
- /AGENTS.md - Essential commands for developers

## Evidence Sources

### Primary Sources
- /README.md - User-facing overview, quickstart, CLI usage, MCP actions, environment variables
- /Cargo.toml - Package metadata, dependencies, binary name
- /src/lib.rs - Module structure
- /src/main.rs - Entry point, CLI modes
- /src/app.rs - Business service layer
- /src/actions.rs - Action dispatcher
- /src/unifi.rs - HTTP client
- /src/config.rs - Configuration loading
- /src/mcp/tools.rs - MCP tool dispatch
- /AGENTS.md - Development instructions
- /CHANGELOG.md - Version history (0.2.0 current)

### Secondary Sources
- /docs/ directory - Comprehensive documentation
- /src/mcp/ - MCP server implementation
- /tests/ - Test structure

## Remaining Questions

None identified. The repository structure is clear and well-documented.

## Next Steps

1. Create openwiki/quickstart.md with the structure above
2. Update /AGENTS.md to add OpenWiki reference section
3. Delete this _plan.md file
