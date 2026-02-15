use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MarketTypeMsg {
    PumpFun,
    PumpSwap,
    MeteoraDbc,
    MeteoraDammV2,
    RaydiumLaunchpad,
    RaydiumCpmm,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PumpFunContextMsg {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PumpSwapContextMsg {
    pub pool: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_config: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MeteoraDbcContextMsg {
    pub pool: String,
    pub config: String,
    pub quote_mint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MeteoraDammV2ContextMsg {
    pub pool: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RaydiumLaunchpadContextMsg {
    pub pool: String,
    pub config: String,
    pub platform: String,
    pub quote_mint: String,
    pub user_quote_account: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RaydiumCpmmContextMsg {
    pub pool: String,
    pub config: String,
    pub quote_mint: String,
    pub user_quote_account: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MarketContextMsg {
    pub market_type: MarketTypeMsg,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pumpfun: Option<PumpFunContextMsg>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pumpswap: Option<PumpSwapContextMsg>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meteora_dbc: Option<MeteoraDbcContextMsg>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meteora_damm_v2: Option<MeteoraDammV2ContextMsg>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raydium_launchpad: Option<RaydiumLaunchpadContextMsg>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raydium_cpmm: Option<RaydiumCpmmContextMsg>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StrategyConfigMsg {
    pub target_profit_pct: f64,
    pub stop_loss_pct: f64,
    pub deadline_timeout_sec: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LimitsMsg {
    pub hi_capacity: u32,
    pub pnl_flush_ms: u64,
    pub max_positions_per_session: u32,
    #[serde(default)]
    pub max_wallets_per_session: u32,
    #[serde(default)]
    pub max_positions_per_wallet: u32,
    #[serde(default)]
    pub max_sessions_per_api_key: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ClientMessage {
    Ping {
        client_time_ms: u64,
    },
    Configure {
        #[serde(
            alias = "wallet_pubkey",
            deserialize_with = "deserialize_wallet_pubkeys"
        )]
        wallet_pubkeys: Vec<String>,
        strategy: StrategyConfigMsg,
    },
    UpdateStrategy {
        strategy: StrategyConfigMsg,
    },
    ClosePosition {
        #[serde(skip_serializing_if = "Option::is_none")]
        position_id: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        token_account: Option<String>,
    },
    #[serde(rename = "request_exit_signal", alias = "sell_now")]
    RequestExitSignal {
        #[serde(skip_serializing_if = "Option::is_none")]
        position_id: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        token_account: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        slippage_bps: Option<u16>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ServerMessage {
    HelloOk {
        session_id: u64,
        server_time_ms: u64,
        limits: LimitsMsg,
    },
    Pong {
        server_time_ms: u64,
    },
    Error {
        code: String,
        message: String,
    },
    PnlUpdate {
        position_id: u64,
        profit_units: i64,
        proceeds_units: u64,
        server_time_ms: u64,
    },
    BalanceUpdate {
        wallet_pubkey: String,
        mint: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        token_account: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        token_program: Option<String>,
        tokens: u64,
        slot: u64,
    },
    PositionOpened {
        position_id: u64,
        wallet_pubkey: String,
        mint: String,
        token_account: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        token_program: Option<String>,
        tokens: u64,
        entry_quote_units: u64,
        #[serde(skip_serializing_if = "Option::is_none")]
        market_context: Option<MarketContextMsg>,
        slot: u64,
    },
    PositionClosed {
        position_id: u64,
        wallet_pubkey: String,
        mint: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        token_account: Option<String>,
        reason: String,
        slot: u64,
    },
    ExitSignalWithTx {
        session_id: u64,
        position_id: u64,
        wallet_pubkey: String,
        mint: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        token_account: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        token_program: Option<String>,
        position_tokens: u64,
        profit_units: i64,
        reason: String,
        triggered_at_ms: u64,
        #[serde(skip_serializing_if = "Option::is_none")]
        market_context: Option<MarketContextMsg>,
        unsigned_tx_b64: String,
    },
}

impl ClientMessage {
    pub fn from_text(text: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(text)
    }
}

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
