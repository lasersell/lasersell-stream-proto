# lasersell-stream-proto

`lasersell-stream-proto` contains shared JSON protocol types for LaserSell stream clients and servers.

It provides strongly typed `serde` models for stream messages, including:
- `ClientMessage` commands sent by clients
- `ServerMessage` events sent by the server
- Strategy, limits, and market-context payloads

## Quick Example

```rust
use lasersell_stream_proto::{ClientMessage, StrategyConfigMsg};

let msg = ClientMessage::Configure {
    wallet_pubkeys: vec!["YourWalletPubkey".to_string()],
    strategy: StrategyConfigMsg {
        target_profit_pct: 5.0,
        stop_loss_pct: 1.5,
    },
};

let json = serde_json::to_string(&msg)?;
let decoded = ClientMessage::from_text(&json)?;
assert_eq!(msg, decoded);
# Ok::<(), serde_json::Error>(())
```
