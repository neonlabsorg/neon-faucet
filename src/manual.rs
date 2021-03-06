//! Faucet manual module.

const MANUAL_HEADER: &str = r##"
# Neon Faucet

The Neon Faucet is a service that distributes small amounts of tokens.
"##;

const MANUAL_API: &str = r##"
# HTTP API Endpoints

A client uses standard HTTP requests to send data to the server.
Several endpoints are supported.

|:-:|:-:|-
|**Endpoint**|**Method**|**Workload**|**Description**|
|:-|:-:|:-:|-
| request_ping | GET | text | Requests ping to check availability of the service
| request_version | GET | | Requests version of the service
| request_neon_in_galans | POST | JSON | Requests NEON tokens, amount in galans (fractions)
| request_neon | POST | JSON | Requests NEON tokens
| request_erc20_list | GET | | Requests list of available ERC20 tokens
| request_erc20 | POST | JSON | Requests ERC20 tokens
|-

Examples of JSON workload:
```
{ "wallet": "0x4570e07200b6332989Dc04fA2a671b839D26eF0E", "amount": 1 }
```
```
{ "wallet": "0x4570e07200b6332989Dc04fA2a671b839D26eF0E", "token_addr": "0x00000000000000000000000000000000CafeBabe", "amount": 10 }
```

Example of ping request with **curl** utility:
```
curl -i -X GET -d 'Hello' 'http://localhost:3333/request_ping'
```

Example of version request with **curl** utility:
```
curl -i -X GET 'http://localhost:3333/request_version'
```

Example of request of list of ERC20 with **curl** utility:
```
curl -i -X GET 'http://localhost:3333/request_erc20_list'
```

Example of NEON drop request with **curl** utility:
```
curl -i -X POST \
    -d '{"wallet": "0x4570e07200b6332989Dc04fA2a671b839D26eF0E", \
         "amount": 1}' \
    'http://localhost:3333/request_neon'
```

Example of ERC20 drop request with **curl** utility:
```
curl -i -X POST \
    -d '{"wallet": "0x4570e07200b6332989Dc04fA2a671b839D26eF0E", \
         "token_addr": "0x00000000000000000000000000000000CafeBabe", \
         "amount": 1}' \
    'http://localhost:3333/request_erc20'
```
"##;

const MANUAL_CONFIG: &str = r##"
# Configuration

The configuration file should be in TOML format.

|:-:|-
|**Option**|**Description**|
|:-|-
| **rpc**.bind | Local interface TCP address
| **rpc**.port | TCP port to listen
| **rpc**.allowed_origins | List of client URLs that can send requests
| **web3**.enable | Flag to on/off the entire **web3** section
| **web3**.rpc_url | Ethereum network endpoint
| **web3**.private_key | Ethereum private key to support operations
| **web3**.tokens | List of available ERC20 token addresses
| **web3**.max_amount | Largest amount of ERC20 tokens to distribute with a single request
| **solana**.enable | Flag to on/off the entire **solana** section
| **solana**.url | Solana network endpoint
| **solana**.commitment | Solana client commitment level
| **solana**.operator_keyfile | Solana keyfile to support operations
| **solana**.evm_loader | Address of the EVM Loader program
| **solana**.max_amount | Largest amount of NEONs to distribute with a single request
|-

Example of the configuration file contents:
```
[rpc]
bind = "0.0.0.0"
port = 3333
allowed_origins = ["http://localhost"]

[web3]
enable = true
rpc_url = "http://localhost:9090/solana"
private_key = "0x0000000000000000000000000000000000000000000000000000000000000Ace"
tokens = ["0x00000000000000000000000000000000CafeBabe",
          "0x00000000000000000000000000000000DeadBeef"]
max_amount = 1000

[solana]
enable = true
url = "http://localhost:8899"
commitment = "processed"
evm_loader = "EvmLoaderId11111111111111111111111111111111"
operator_keyfile = "operator_id.json"
max_amount = 10
```

The configuration file is optional and, if present, can be incomplete
(default values or environment variables will be used in such cases).
"##;

const MANUAL_ENV: &str = r##"
# Environment Variables

Environment variables, if present, override portions of the configuration.

|:-:|:-:|-
|**Name**|**Overrides**|**Value Example**|
|:-|:-|-
| FAUCET_RPC_BIND | **rpc**.bind | `0.0.0.0`
| FAUCET_RPC_PORT | **rpc**.port | `3333`
| FAUCET_RPC_ALLOWED_ORIGINS | **rpc**.allowed_origins | `["http://localhost"]`
| FAUCET_WEB3_ENABLE | **web3**.enable | `true`
| WEB3_RPC_URL | **web3**.rpc_url | `http://localhost:9090/solana`
| WEB3_PRIVATE_KEY | **web3**.private_key | `0x00A`
| NEON_ERC20_TOKENS | **web3**.tokens | `["0x00B", "0x00C"]`
| NEON_ERC20_MAX_AMOUNT | **web3**.max_amount | `1000`
| FAUCET_SOLANA_ENABLE | **solana**.enable | `true`
| SOLANA_URL | **solana**.url | `http://localhost:8899`
| SOLANA_COMMITMENT | **solana**.commitment | `processed`
| EVM_LOADER | **solana**.evm_loader | `EvmLoaderId11111111111111111111111111111111`
| NEON_OPERATOR_KEYFILE | **solana**.operator_keyfile | `operator_id.json`
| NEON_ETH_MAX_AMOUNT | **solana**.max_amount | `10`
| NEON_LOG | | `json`
| RUST_LOG | | `info`
|-
"##;

/// Dump manual in raw Markdown format.
pub fn dump(api: bool, config: bool, env: bool) {
    println!("{}", MANUAL_HEADER);

    let all = !api && !config && !env;
    if all {
        println!("{}", MANUAL_API);
        println!("{}", MANUAL_CONFIG);
        println!("{}", MANUAL_ENV);
    }

    if api {
        println!("{}", MANUAL_API);
    }
    if config {
        println!("{}", MANUAL_CONFIG);
    }
    if env {
        println!("{}", MANUAL_ENV);
    }
}

use minimad::Alignment;
use termimad::MadSkin;

/// Renders manuals, all or selected.
pub fn show(api: bool, config: bool, env: bool) {
    let mut skin = MadSkin::default();
    skin.headers[0].align = Alignment::Left;

    skin.print_text(MANUAL_HEADER);

    let all = !api && !config && !env;
    if all {
        skin.print_text(MANUAL_API);
        skin.print_text(MANUAL_CONFIG);
        skin.print_text(MANUAL_ENV);
    }

    if api {
        skin.print_text(MANUAL_API);
    }
    if config {
        skin.print_text(MANUAL_CONFIG);
    }
    if env {
        skin.print_text(MANUAL_ENV);
    }
}
