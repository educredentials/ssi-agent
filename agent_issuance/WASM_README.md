# Agent Issuance - WebAssembly Compilation Guide

This guide provides comprehensive instructions for compiling the `agent_issuance` crate to WebAssembly for browser-based credential issuance.

## Table of Contents

- [Overview](#overview)
- [Prerequisites](#prerequisites)
- [Quick Start](#quick-start)
- [Architecture](#architecture)
- [Limitations & Challenges](#limitations--challenges)
- [Build Instructions](#build-instructions)
- [Usage Examples](#usage-examples)
- [API Reference](#api-reference)
- [Troubleshooting](#troubleshooting)
- [Production Considerations](#production-considerations)

## Overview

The WASM implementation enables credential issuance directly in web browsers, providing:

- **Browser-native execution**: No server-side dependencies required
- **OpenID4VCI support**: Standards-compliant credential issuance
- **In-memory storage**: Fast, ephemeral credential management
- **JavaScript interop**: Easy integration with web applications

### Key Features

- Create credential offers
- Issue verifiable credentials
- Verify credential requests (basic)
- In-memory event sourcing

### Known Limitations

- **No persistent storage**: All data is kept in memory (lost on page reload)
- **Simplified cryptography**: No Stronghold integration (requires Web Crypto API implementation)
- **No credential signing**: Proof-of-concept signatures only
- **Single-threaded**: Browser JavaScript execution model
- **Limited proof verification**: Basic validation only

## Prerequisites

### Required Tools

1. **Rust toolchain** (1.76.0 or later)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **wasm32-unknown-unknown target**
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

3. **wasm-bindgen-cli** (must match wasm-bindgen dependency version)
   ```bash
   cargo install wasm-bindgen-cli --version 0.2.92
   ```

### Optional Tools

4. **wasm-opt** (for optimization)
   ```bash
   cargo install wasm-opt
   ```

5. **Local web server** (for testing)
   ```bash
   # Python 3
   python3 -m http.server 8000

   # Node.js
   npx http-server -p 8000
   ```

## Quick Start

### 1. Build the WASM Module

```bash
cd /home/daniel/Documents/GitHub/ssi-agent/impierce-ssi-agent/agent_issuance
./build-wasm.sh
```

This will:
- Compile the crate for `wasm32-unknown-unknown`
- Generate JavaScript bindings with wasm-bindgen
- Create TypeScript definitions
- Optimize the WASM binary (if wasm-opt is installed)
- Output files to the `pkg/` directory

### 2. Test the Demo

```bash
# Start a local web server
python3 -m http.server 8000

# Open in browser
open http://localhost:8000/www/index.html
```

### 3. Integrate into Your Application

```javascript
import init, { WasmIssuanceAgent, WasmIssuanceConfig } from './pkg/agent_issuance.js';

// Initialize the WASM module
await init();

// Create configuration
const config = new WasmIssuanceConfig(
    'https://issuer.example.com',
    'My Issuer'
);

// Create agent
const agent = new WasmIssuanceAgent(config);

// Create a credential offer
const offer = await agent.create_credential_offer(
    ['credential-config-001'],
    ['urn:ietf:params:oauth:grant-type:pre-authorized_code']
);

console.log('Credential offer:', JSON.parse(offer));
```

## Architecture

### Component Overview

```
┌─────────────────────────────────────────────┐
│           Browser Environment                │
│                                              │
│  ┌────────────────────────────────────┐    │
│  │   JavaScript Application            │    │
│  └───────────┬────────────────────────┘    │
│              │                               │
│              ▼                               │
│  ┌────────────────────────────────────┐    │
│  │   WASM Bindings (wasm-bindgen)     │    │
│  └───────────┬────────────────────────┘    │
│              │                               │
│              ▼                               │
│  ┌────────────────────────────────────┐    │
│  │   WasmIssuanceAgent                │    │
│  │   - Credential Offers               │    │
│  │   - Credential Issuance             │    │
│  │   - Request Verification            │    │
│  └───────────┬────────────────────────┘    │
│              │                               │
│              ▼                               │
│  ┌────────────────────────────────────┐    │
│  │   WasmEventStore (In-Memory)       │    │
│  └────────────────────────────────────┘    │
│                                              │
└─────────────────────────────────────────────┘
```

### Key Differences from Native Implementation

| Feature | Native | WASM |
|---------|--------|------|
| Storage | PostgreSQL/MongoDB | In-memory |
| Async Runtime | Tokio | wasm-bindgen-futures |
| HTTP Client | reqwest (native TLS) | reqwest (wasm) or fetch API |
| Cryptography | Stronghold + iota-sdk | Web Crypto API |
| Configuration | File system | JavaScript objects |
| Threading | Multi-threaded | Single-threaded |

## Limitations & Challenges

### Critical Blockers

#### 1. CQRS-ES Framework

**Problem**: The `cqrs-es` crate depends on database backends (postgres-es, mongo-es) that are not WASM-compatible.

**Current Workaround**: Custom in-memory event store implementation in `/home/daniel/Documents/GitHub/ssi-agent/impierce-ssi-agent/agent_issuance/src/wasm.rs`

**Future Solution**:
- Create a `cqrs-es-wasm` adapter
- Implement IndexedDB backend for browser persistence
- Use LocalStorage for simple key-value persistence

#### 2. Agent Secret Manager

**Problem**: The `agent_secret_manager` dependency uses:
- `iota_stronghold`: Native secure storage (not WASM-compatible)
- `iota-sdk`: Native blockchain SDK
- Native cryptography libraries

**Current Workaround**: Simplified key management without secure storage

**Future Solution**:
- Implement Web Crypto API for key generation and signing
- Use browser's native credential storage
- Create WASM-compatible DID resolver

#### 3. File System Access

**Problem**: Configuration system in `agent_shared` reads from file system (line 206 in `/home/daniel/Documents/GitHub/ssi-agent/impierce-ssi-agent/agent_issuance/src/state.rs`)

```rust
let file = std::fs::read(file.as_path()).expect("Failed to read credential configuration file");
```

**Current Workaround**: Configuration passed via JavaScript objects

**Future Solution**:
- Use browser File API for user-selected files
- Embed configuration in WASM binary at compile time
- Load configuration from HTTP endpoints

#### 4. HTTP Client Usage

**Problem**: Direct `reqwest::Client` usage in offer aggregate (line 176-190 in `/home/daniel/Documents/GitHub/ssi-agent/impierce-ssi-agent/agent_issuance/src/offer/aggregate.rs`)

```rust
let client = reqwest::Client::new();
client.get(target).send().await
```

**Solution**:
- Use `reqwest` with `wasm` feature enabled
- Or use browser's native `fetch` API via `web-sys`

Add to `Cargo.toml`:
```toml
[target.'cfg(target_arch = "wasm32")'.dependencies]
reqwest = { version = "0.12", features = ["json"] }
```

### Dependency Compatibility

#### Compatible Dependencies

✅ **Already WASM-compatible**:
- `serde` / `serde_json`
- `chrono` (with wasm feature)
- `url`
- `base64`
- `jsonwebtoken`
- `thiserror`
- `anyhow`
- `async-trait`
- `uuid`

#### Incompatible Dependencies

❌ **Not WASM-compatible**:
- `cqrs-es` (database backends)
- `iota_stronghold`
- `iota-sdk` / `iota-sdk-legacy`
- `tokio` (use with wasm features or wasm-bindgen-futures)

#### Conditionally Compatible

⚠️ **Require special configuration**:
- `reqwest`: Needs `wasm` feature
- `rand`: Needs `getrandom` with `js` feature
- `tokio`: Limited WASM support

## Build Instructions

### Development Build

```bash
./build-wasm.sh debug
```

Produces:
- Unoptimized WASM binary (~2-5 MB)
- Debug symbols included
- Faster build times

### Production Build

```bash
./build-wasm.sh release
```

Produces:
- Optimized WASM binary (~500KB - 1MB)
- No debug symbols
- Smaller file size
- wasm-opt optimizations applied

### Clean Build

```bash
./build-wasm.sh release clean
```

### Manual Build

```bash
# Build the WASM target
cargo build --target wasm32-unknown-unknown --release --features wasm

# Generate bindings
wasm-bindgen target/wasm32-unknown-unknown/release/agent_issuance.wasm \
    --out-dir pkg \
    --target web \
    --typescript

# Optimize (optional)
wasm-opt -Oz -o pkg/agent_issuance_bg_opt.wasm pkg/agent_issuance_bg.wasm
mv pkg/agent_issuance_bg_opt.wasm pkg/agent_issuance_bg.wasm
```

### Build Outputs

After building, the `pkg/` directory contains:

```
pkg/
├── agent_issuance_bg.wasm       # WebAssembly binary
├── agent_issuance.js            # JavaScript bindings
├── agent_issuance.d.ts          # TypeScript definitions
└── package.json                 # NPM package metadata (optional)
```

## Usage Examples

### Basic Credential Issuance

```javascript
import init, { WasmIssuanceAgent, WasmIssuanceConfig } from './pkg/agent_issuance.js';

async function issueCredential() {
    // Initialize WASM
    await init();

    // Create agent
    const config = new WasmIssuanceConfig(
        'https://issuer.example.com',
        'Example Issuer'
    );
    const agent = new WasmIssuanceAgent(config);

    // Prepare credential data
    const credentialData = {
        id: 'did:example:holder123',
        first_name: 'Alice',
        last_name: 'Smith',
        email: 'alice@example.com'
    };

    // Issue credential
    const result = await agent.issue_credential(
        JSON.stringify(credentialData),
        'credential-config-001'
    );

    const credential = JSON.parse(result);
    console.log('Issued credential:', credential);
}
```

### Creating and Retrieving Offers

```javascript
// Create an offer
const offerJson = await agent.create_credential_offer(
    ['config-001', 'config-002'],
    ['urn:ietf:params:oauth:grant-type:pre-authorized_code']
);

const offer = JSON.parse(offerJson);
const offerId = offer.offer_id;

// Retrieve the offer later
const retrievedOffer = await agent.get_credential_offer(offerId);
console.log('Retrieved offer:', JSON.parse(retrievedOffer));
```

### Error Handling

```javascript
try {
    const result = await agent.issue_credential(invalidData, 'config-001');
} catch (error) {
    if (error instanceof Error) {
        console.error('Error issuing credential:', error.message);
    } else {
        console.error('Unknown error:', error);
    }
}
```

## API Reference

### WasmIssuanceConfig

Configuration object for the issuance agent.

```typescript
class WasmIssuanceConfig {
    constructor(issuer_url: string, issuer_name: string);

    get issuer_url(): string;
    set issuer_url(url: string);

    get issuer_name(): string;
    set issuer_name(name: string);
}
```

### WasmIssuanceAgent

Main agent class for credential operations.

```typescript
class WasmIssuanceAgent {
    constructor(config: WasmIssuanceConfig);

    // Get agent configuration
    get_config(): WasmIssuanceConfig;

    // Create a credential offer
    create_credential_offer(
        credential_configuration_ids: string[],
        grant_types: string[]
    ): Promise<string>;

    // Get credential offer by ID
    get_credential_offer(offer_id: string): string;

    // Verify a credential request
    verify_credential_request(credential_request: string): Promise<string>;

    // Issue a credential
    issue_credential(
        credential_data: string,
        credential_configuration_id: string
    ): Promise<string>;

    // Get all offers
    get_all_offers(): string;

    // Clear storage
    clear_storage(): void;
}
```

### Utility Functions

```typescript
// Initialize WASM module
function init(): Promise<void>;

// Log to browser console
function log(message: string): void;

// Get module version
function version(): string;
```

## Troubleshooting

### WASM Module Fails to Load

**Symptom**: "Failed to fetch" or CORS errors

**Solution**:
1. Ensure you're serving the files from a web server (not `file://`)
2. Check MIME types are correct (`.wasm` should be `application/wasm`)
3. Verify the path to `agent_issuance_bg.wasm` is correct

```javascript
// Check network tab in browser DevTools
// Look for 404 or CORS errors
```

### Build Fails with "linker `rust-lld` not found"

**Solution**:
```bash
# Reinstall the wasm32 target
rustup target remove wasm32-unknown-unknown
rustup target add wasm32-unknown-unknown
```

### "getrandom" Error

**Symptom**: Runtime error about `getrandom`

**Solution**: Ensure `getrandom` is configured with the `js` feature:

```toml
[target.'cfg(target_arch = "wasm32")'.dependencies]
getrandom = { version = "0.2", features = ["js"] }
```

### Memory Issues

**Symptom**: Out of memory errors in browser

**Solution**:
1. Use the optimized release build
2. Clear storage periodically with `agent.clear_storage()`
3. Consider implementing data pagination

### TypeScript Compilation Errors

**Solution**: Ensure you're using the generated `.d.ts` file:

```typescript
import { WasmIssuanceAgent } from './pkg/agent_issuance';
```

## Production Considerations

### Security

1. **Key Management**
   - ⚠️ Current implementation lacks secure key storage
   - Implement Web Crypto API for key operations
   - Never expose private keys to JavaScript

2. **Data Validation**
   - Validate all inputs on both JS and Rust sides
   - Sanitize JSON data before processing
   - Implement rate limiting for operations

3. **HTTPS Required**
   - Web Crypto API requires secure context
   - Always use HTTPS in production
   - Consider using Content Security Policy

### Performance

1. **Lazy Loading**
   ```javascript
   // Load WASM only when needed
   const loadWasm = async () => {
       const module = await import('./pkg/agent_issuance.js');
       await module.default();
       return module;
   };
   ```

2. **Caching**
   - Enable browser caching for `.wasm` files
   - Use service workers for offline support
   - Consider compressing WASM with gzip/brotli

3. **Memory Management**
   - Monitor memory usage with `performance.memory`
   - Clear storage when no longer needed
   - Implement data retention policies

### Storage Options

1. **LocalStorage** (simple, limited to 5-10MB)
   ```javascript
   // Save offer to LocalStorage
   localStorage.setItem('offers', JSON.stringify(offers));
   ```

2. **IndexedDB** (complex, larger capacity)
   ```javascript
   // Use IndexedDB for persistent storage
   const db = await openDatabase();
   await db.put('offers', offer);
   ```

3. **Session Storage** (temporary, cleared on tab close)
   ```javascript
   sessionStorage.setItem('currentOffer', JSON.stringify(offer));
   ```

### Browser Compatibility

Minimum browser versions:
- Chrome/Edge: 87+
- Firefox: 89+
- Safari: 15+
- Opera: 73+

Check compatibility:
```javascript
if (typeof WebAssembly === 'object') {
    // WASM is supported
} else {
    // Fallback to server-side implementation
}
```

### Monitoring

Add telemetry:
```javascript
// Track WASM load time
const loadStart = performance.now();
await init();
const loadTime = performance.now() - loadStart;
console.log(`WASM loaded in ${loadTime}ms`);

// Track operation performance
const opStart = performance.now();
await agent.issue_credential(data, config);
const opTime = performance.now() - opStart;
console.log(`Credential issued in ${opTime}ms`);
```

## Next Steps

### Recommended Improvements

1. **Implement Web Crypto API Integration**
   - Add proper key generation
   - Implement credential signing
   - Add proof verification

2. **Add IndexedDB Backend**
   - Persistent storage in browser
   - Larger storage capacity
   - Transaction support

3. **Implement Full CQRS-ES**
   - WASM-compatible event store
   - Proper aggregate replay
   - Event versioning

4. **Add Unit Tests**
   - `wasm-bindgen-test` integration
   - Browser-based testing
   - CI/CD pipeline

5. **Optimize Bundle Size**
   - Feature flags for unused code
   - Dynamic imports
   - Tree shaking

### Resources

- [wasm-bindgen Book](https://rustwasm.github.io/wasm-bindgen/)
- [Rust and WebAssembly](https://rustwasm.github.io/book/)
- [OpenID4VCI Specification](https://openid.net/specs/openid-4-verifiable-credential-issuance-1_0.html)
- [W3C Verifiable Credentials](https://www.w3.org/TR/vc-data-model/)
- [Web Crypto API](https://developer.mozilla.org/en-US/docs/Web/API/Web_Crypto_API)

## Contributing

When contributing WASM-related changes:

1. Test in multiple browsers
2. Document any new dependencies
3. Update this README with changes
4. Add TypeScript type definitions
5. Include usage examples

## License

Same as parent project - see main repository LICENSE file.
