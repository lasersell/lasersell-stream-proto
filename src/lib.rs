//! Shared JSON protocol models for LaserSell stream clients and servers.
//!
//! The types in this crate define the wire format exchanged over the stream API
//! and are designed for round-trip JSON serialization with `serde`.

use serde::{Deserialize, Deserializer, Serialize};

/// Supported market types for an opened position.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MarketTypeMsg {
    /// pump.fun market.
    PumpFun,
    /// PumpSwap market.
    PumpSwap,
    /// Meteora Dynamic Bonding Curve market.
    MeteoraDbc,
    /// Meteora DAMM v2 market.
    MeteoraDammV2,
    /// Raydium Launchpad market.
    RaydiumLaunchpad,
    /// Raydium CPMM market.
    RaydiumCpmm,
}

/// Context payload for `MarketTypeMsg::PumpFun`.
///
/// This is currently empty and acts as an explicit marker.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PumpFunContextMsg {}

/// Context payload for `MarketTypeMsg::PumpSwap`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PumpSwapContextMsg {
    /// PumpSwap pool account.
    pub pool: String,
    /// Optional PumpSwap global config account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_config: Option<String>,
}

/// Context payload for `MarketTypeMsg::MeteoraDbc`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MeteoraDbcContextMsg {
    /// Meteora DBC pool account.
    pub pool: String,
    /// Meteora DBC config account.
    pub config: String,
    /// Quote mint used by the pool.
    pub quote_mint: String,
}

/// Context payload for `MarketTypeMsg::MeteoraDammV2`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MeteoraDammV2ContextMsg {
    /// Meteora DAMM v2 pool account.
    pub pool: String,
}

/// Context payload for `MarketTypeMsg::RaydiumLaunchpad`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RaydiumLaunchpadContextMsg {
    /// Raydium Launchpad pool account.
    pub pool: String,
    /// Raydium Launchpad config account.
    pub config: String,
    /// Raydium Launchpad platform account.
    pub platform: String,
    /// Quote mint used by the pool.
    pub quote_mint: String,
    /// User's quote token account associated with the position.
    pub user_quote_account: String,
}

/// Context payload for `MarketTypeMsg::RaydiumCpmm`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RaydiumCpmmContextMsg {
    /// Raydium CPMM pool account.
    pub pool: String,
    /// Raydium CPMM config account.
    pub config: String,
    /// Quote mint used by the pool.
    pub quote_mint: String,
    /// User's quote token account associated with the position.
    pub user_quote_account: String,
}

/// Market-specific context carried with position events.
///
/// Exactly one optional context field should match `market_type`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MarketContextMsg {
    /// Market discriminator for the active context field.
    pub market_type: MarketTypeMsg,
    /// Context for `pump_fun` markets.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pumpfun: Option<PumpFunContextMsg>,
    /// Context for `pump_swap` markets.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pumpswap: Option<PumpSwapContextMsg>,
    /// Context for `meteora_dbc` markets.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meteora_dbc: Option<MeteoraDbcContextMsg>,
    /// Context for `meteora_damm_v2` markets.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meteora_damm_v2: Option<MeteoraDammV2ContextMsg>,
    /// Context for `raydium_launchpad` markets.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raydium_launchpad: Option<RaydiumLaunchpadContextMsg>,
    /// Context for `raydium_cpmm` markets.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raydium_cpmm: Option<RaydiumCpmmContextMsg>,
}

/// Client-side strategy thresholds used for automated exits.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StrategyConfigMsg {
    /// Target take-profit percentage.
    pub target_profit_pct: f64,
    /// Stop-loss percentage.
    pub stop_loss_pct: f64,
    /// Max seconds to wait for an outbound transaction deadline.
    pub deadline_timeout_sec: u64,
}

/// Server-enforced per-session and per-key limits.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LimitsMsg {
    /// Max concurrent positions tracked at high priority.
    pub hi_capacity: u32,
    /// PnL push cadence in milliseconds.
    pub pnl_flush_ms: u64,
    /// Max positions allowed in one session.
    pub max_positions_per_session: u32,
    /// Max wallets accepted in one session.
    #[serde(default)]
    pub max_wallets_per_session: u32,
    /// Max tracked positions per wallet.
    #[serde(default)]
    pub max_positions_per_wallet: u32,
    /// Max simultaneous sessions per API key.
    #[serde(default)]
    pub max_sessions_per_api_key: u32,
}

/// Commands sent from client to server.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ClientMessage {
    /// Keepalive ping from client.
    Ping {
        /// Client timestamp in Unix milliseconds.
        client_time_ms: u64,
    },
    /// Initial session configuration for wallets and strategy.
    Configure {
        /// Wallet pubkeys to monitor. Accepts legacy `wallet_pubkey` alias.
        #[serde(
            alias = "wallet_pubkey",
            deserialize_with = "deserialize_wallet_pubkeys"
        )]
        wallet_pubkeys: Vec<String>,
        /// Strategy thresholds for the session.
        strategy: StrategyConfigMsg,
    },
    /// Update strategy thresholds for an active session.
    UpdateStrategy {
        /// New strategy configuration.
        strategy: StrategyConfigMsg,
    },
    /// Request that a tracked position be closed.
    ClosePosition {
        /// Optional internal position identifier.
        #[serde(skip_serializing_if = "Option::is_none")]
        position_id: Option<u64>,
        /// Optional token account key for lookup when ID is unknown.
        #[serde(skip_serializing_if = "Option::is_none")]
        token_account: Option<String>,
    },
    /// Request an immediate exit signal and unsigned transaction.
    ///
    /// Also deserializes from the legacy `sell_now` message type.
    #[serde(rename = "request_exit_signal", alias = "sell_now")]
    RequestExitSignal {
        /// Optional internal position identifier.
        #[serde(skip_serializing_if = "Option::is_none")]
        position_id: Option<u64>,
        /// Optional token account key for lookup when ID is unknown.
        #[serde(skip_serializing_if = "Option::is_none")]
        token_account: Option<String>,
        /// Optional slippage tolerance, in basis points.
        #[serde(skip_serializing_if = "Option::is_none")]
        slippage_bps: Option<u16>,
    },
}

/// Events and responses sent from server to client.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ServerMessage {
    /// Successful handshake response with limits.
    HelloOk {
        /// Assigned session identifier.
        session_id: u64,
        /// Server timestamp in Unix milliseconds.
        server_time_ms: u64,
        /// Effective limits for the session/API key.
        limits: LimitsMsg,
    },
    /// Keepalive pong from server.
    Pong {
        /// Server timestamp in Unix milliseconds.
        server_time_ms: u64,
    },
    /// Error response for invalid requests or runtime failures.
    Error {
        /// Stable machine-readable error code.
        code: String,
        /// Human-readable error message.
        message: String,
    },
    /// Incremental PnL update for a position.
    PnlUpdate {
        /// Internal position identifier.
        position_id: u64,
        /// Profit/loss in quote units.
        profit_units: i64,
        /// Estimated proceeds in quote units.
        proceeds_units: u64,
        /// Server timestamp in Unix milliseconds.
        server_time_ms: u64,
    },
    /// Balance update for a tracked wallet/mint.
    BalanceUpdate {
        /// Wallet pubkey the balance belongs to.
        wallet_pubkey: String,
        /// Token mint pubkey.
        mint: String,
        /// Optional token account pubkey.
        #[serde(skip_serializing_if = "Option::is_none")]
        token_account: Option<String>,
        /// Optional token program pubkey.
        #[serde(skip_serializing_if = "Option::is_none")]
        token_program: Option<String>,
        /// Token amount in native units.
        tokens: u64,
        /// Slot the balance snapshot came from.
        slot: u64,
    },
    /// Notification that a new position has been opened.
    PositionOpened {
        /// Internal position identifier.
        position_id: u64,
        /// Wallet pubkey associated with the position.
        wallet_pubkey: String,
        /// Token mint pubkey.
        mint: String,
        /// Token account pubkey.
        token_account: String,
        /// Optional token program pubkey.
        #[serde(skip_serializing_if = "Option::is_none")]
        token_program: Option<String>,
        /// Position token amount in native units.
        tokens: u64,
        /// Entry cost in quote units.
        entry_quote_units: u64,
        /// Optional market metadata for this position.
        #[serde(skip_serializing_if = "Option::is_none")]
        market_context: Option<MarketContextMsg>,
        /// Slot when the position opened.
        slot: u64,
    },
    /// Notification that a position has been closed.
    PositionClosed {
        /// Internal position identifier.
        position_id: u64,
        /// Wallet pubkey associated with the position.
        wallet_pubkey: String,
        /// Token mint pubkey.
        mint: String,
        /// Optional token account pubkey.
        #[serde(skip_serializing_if = "Option::is_none")]
        token_account: Option<String>,
        /// Reason string for the close event.
        reason: String,
        /// Slot when the position closed.
        slot: u64,
    },
    /// Exit signal payload that includes an unsigned transaction.
    ExitSignalWithTx {
        /// Session identifier for correlation.
        session_id: u64,
        /// Internal position identifier.
        position_id: u64,
        /// Wallet pubkey associated with the position.
        wallet_pubkey: String,
        /// Token mint pubkey.
        mint: String,
        /// Optional token account pubkey.
        #[serde(skip_serializing_if = "Option::is_none")]
        token_account: Option<String>,
        /// Optional token program pubkey.
        #[serde(skip_serializing_if = "Option::is_none")]
        token_program: Option<String>,
        /// Position token amount in native units.
        position_tokens: u64,
        /// Profit/loss in quote units.
        profit_units: i64,
        /// Trigger reason for the exit.
        reason: String,
        /// Trigger timestamp in Unix milliseconds.
        triggered_at_ms: u64,
        /// Optional market metadata for this position.
        #[serde(skip_serializing_if = "Option::is_none")]
        market_context: Option<MarketContextMsg>,
        /// Base64-encoded unsigned transaction payload.
        unsigned_tx_b64: String,
    },
}

impl ClientMessage {
    /// Parses a JSON string into a [`ClientMessage`].
    pub fn from_text(text: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(text)
    }
}

/// Deserializes either a single wallet pubkey string or an array.
fn deserialize_wallet_pubkeys<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum WalletPubkeysField {
        One(String),
        Many(Vec<String>),
    }

    match WalletPubkeysField::deserialize(deserializer)? {
        WalletPubkeysField::One(pubkey) => Ok(vec![pubkey]),
        WalletPubkeysField::Many(pubkeys) => Ok(pubkeys),
    }
}

impl ServerMessage {
    /// Serializes this message into its JSON wire representation.
    pub fn to_text(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn round_trip<T>(value: T)
    where
        T: Serialize + for<'de> Deserialize<'de> + PartialEq + std::fmt::Debug,
    {
        let json = serde_json::to_string(&value).expect("serialize");
        let decoded: T = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(value, decoded);
    }

    fn base_context() -> MarketContextMsg {
        MarketContextMsg {
            market_type: MarketTypeMsg::PumpFun,
            pumpfun: None,
            pumpswap: None,
            meteora_dbc: None,
            meteora_damm_v2: None,
            raydium_launchpad: None,
            raydium_cpmm: None,
        }
    }

    #[test]
    fn market_context_round_trip_pumpfun() {
        let mut ctx = base_context();
        ctx.market_type = MarketTypeMsg::PumpFun;
        ctx.pumpfun = Some(PumpFunContextMsg {});
        round_trip(ctx);
    }

    #[test]
    fn market_context_round_trip_pumpswap() {
        let mut ctx = base_context();
        ctx.market_type = MarketTypeMsg::PumpSwap;
        ctx.pumpswap = Some(PumpSwapContextMsg {
            pool: "11111111111111111111111111111111".to_string(),
            global_config: Some("22222222222222222222222222222222".to_string()),
        });
        round_trip(ctx);
    }

    #[test]
    fn market_context_round_trip_meteora_dbc() {
        let mut ctx = base_context();
        ctx.market_type = MarketTypeMsg::MeteoraDbc;
        ctx.meteora_dbc = Some(MeteoraDbcContextMsg {
            pool: "11111111111111111111111111111111".to_string(),
            config: "22222222222222222222222222222222".to_string(),
            quote_mint: "33333333333333333333333333333333".to_string(),
        });
        round_trip(ctx);
    }

    #[test]
    fn market_context_round_trip_meteora_damm_v2() {
        let mut ctx = base_context();
        ctx.market_type = MarketTypeMsg::MeteoraDammV2;
        ctx.meteora_damm_v2 = Some(MeteoraDammV2ContextMsg {
            pool: "11111111111111111111111111111111".to_string(),
        });
        round_trip(ctx);
    }

    #[test]
    fn market_context_round_trip_raydium_launchpad() {
        let mut ctx = base_context();
        ctx.market_type = MarketTypeMsg::RaydiumLaunchpad;
        ctx.raydium_launchpad = Some(RaydiumLaunchpadContextMsg {
            pool: "11111111111111111111111111111111".to_string(),
            config: "22222222222222222222222222222222".to_string(),
            platform: "33333333333333333333333333333333".to_string(),
            quote_mint: "44444444444444444444444444444444".to_string(),
            user_quote_account: "55555555555555555555555555555555".to_string(),
        });
        round_trip(ctx);
    }

    #[test]
    fn market_context_round_trip_raydium_cpmm() {
        let mut ctx = base_context();
        ctx.market_type = MarketTypeMsg::RaydiumCpmm;
        ctx.raydium_cpmm = Some(RaydiumCpmmContextMsg {
            pool: "11111111111111111111111111111111".to_string(),
            config: "22222222222222222222222222222222".to_string(),
            quote_mint: "33333333333333333333333333333333".to_string(),
            user_quote_account: "44444444444444444444444444444444".to_string(),
        });
        round_trip(ctx);
    }

    #[test]
    fn client_configure_round_trip() {
        let msg = ClientMessage::Configure {
            wallet_pubkeys: vec![
                "11111111111111111111111111111111".to_string(),
                "22222222222222222222222222222222".to_string(),
            ],
            strategy: StrategyConfigMsg {
                target_profit_pct: 5.0,
                stop_loss_pct: 1.5,
                deadline_timeout_sec: 45,
            },
        };

        round_trip(msg);
    }

    #[test]
    fn client_configure_deserializes_legacy_wallet_pubkey_string() {
        let raw = r#"{
            "type":"configure",
            "wallet_pubkey":"11111111111111111111111111111111",
            "strategy":{
                "target_profit_pct":5.0,
                "stop_loss_pct":1.5,
                "deadline_timeout_sec":45
            }
        }"#;

        let msg: ClientMessage = serde_json::from_str(raw).expect("deserialize");
        assert_eq!(
            msg,
            ClientMessage::Configure {
                wallet_pubkeys: vec!["11111111111111111111111111111111".to_string()],
                strategy: StrategyConfigMsg {
                    target_profit_pct: 5.0,
                    stop_loss_pct: 1.5,
                    deadline_timeout_sec: 45,
                },
            }
        );

        let encoded = serde_json::to_value(&msg).expect("serialize");
        assert_eq!(
            encoded.get("wallet_pubkeys"),
            Some(&serde_json::json!(["11111111111111111111111111111111"]))
        );
        assert_eq!(encoded.get("wallet_pubkey"), None);
    }

    #[test]
    fn request_exit_signal_deserializes_sell_now_alias() {
        let raw = r#"{
            "type":"sell_now",
            "position_id":123,
            "slippage_bps":42
        }"#;

        let msg: ClientMessage = serde_json::from_str(raw).expect("deserialize");
        assert_eq!(
            msg,
            ClientMessage::RequestExitSignal {
                position_id: Some(123),
                token_account: None,
                slippage_bps: Some(42),
            }
        );

        let encoded = serde_json::to_value(&msg).expect("serialize");
        assert_eq!(
            encoded.get("type"),
            Some(&serde_json::Value::String(
                "request_exit_signal".to_string()
            ))
        );
    }

    #[test]
    fn server_hello_ok_round_trip() {
        let msg = ServerMessage::HelloOk {
            session_id: 42,
            server_time_ms: 1700000000000,
            limits: LimitsMsg {
                hi_capacity: 256,
                pnl_flush_ms: 100,
                max_positions_per_session: 256,
                max_wallets_per_session: 8,
                max_positions_per_wallet: 64,
                max_sessions_per_api_key: 1,
            },
        };

        round_trip(msg);
    }

    #[test]
    fn exit_signal_with_tx_round_trip() {
        let ctx = MarketContextMsg {
            market_type: MarketTypeMsg::RaydiumCpmm,
            pumpfun: None,
            pumpswap: None,
            meteora_dbc: None,
            meteora_damm_v2: None,
            raydium_launchpad: None,
            raydium_cpmm: Some(RaydiumCpmmContextMsg {
                pool: "11111111111111111111111111111111".to_string(),
                config: "22222222222222222222222222222222".to_string(),
                quote_mint: "33333333333333333333333333333333".to_string(),
                user_quote_account: "44444444444444444444444444444444".to_string(),
            }),
        };
        let msg = ServerMessage::ExitSignalWithTx {
            session_id: 7,
            position_id: 8,
            wallet_pubkey: "55555555555555555555555555555555".to_string(),
            mint: "11111111111111111111111111111111".to_string(),
            token_account: Some("22222222222222222222222222222222".to_string()),
            token_program: None,
            position_tokens: 10,
            profit_units: 5,
            reason: "tp".to_string(),
            triggered_at_ms: 123,
            market_context: Some(ctx),
            unsigned_tx_b64: "dGVzdA==".to_string(),
        };
        round_trip(msg);
    }

    #[test]
    fn pnl_update_round_trip() {
        let msg = ServerMessage::PnlUpdate {
            position_id: 5,
            profit_units: 12,
            proceeds_units: 34,
            server_time_ms: 999,
        };
        round_trip(msg);
    }
}
