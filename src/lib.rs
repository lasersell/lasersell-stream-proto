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

/// A slippage band describing the max sellable tokens at a given slippage.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SlippageBandMsg {
    /// Slippage threshold in basis points.
    pub slippage_bps: u16,
    /// Maximum tokens sellable within this slippage band.
    pub max_tokens: u64,
    /// Coverage as a percentage of the full position (0..100).
    pub coverage_pct: f64,
}

/// A single take-profit level in a chained sell strategy.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TakeProfitLevelMsg {
    /// PnL percentage threshold to trigger this level.
    pub profit_pct: f64,
    /// Percentage of remaining position to sell when triggered.
    pub sell_pct: f64,
    /// New trailing stop percentage after this level fires (0 = no change).
    #[serde(default)]
    pub trailing_stop_pct: f64,
}

/// Client-side strategy thresholds used for automated exits.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct StrategyConfigMsg {
    /// Target take-profit percentage (legacy single-level; ignored when `take_profit_levels` is non-empty).
    pub target_profit_pct: f64,
    /// Stop-loss percentage.
    pub stop_loss_pct: f64,
    /// Trailing stop percentage (locks in profits as price rises).
    #[serde(default)]
    pub trailing_stop_pct: f64,
    /// Automatically sell when a token graduates to a new DEX.
    #[serde(default)]
    pub sell_on_graduation: bool,
    /// Multi-level take-profit chain. When non-empty, overrides `target_profit_pct`.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub take_profit_levels: Vec<TakeProfitLevelMsg>,
    /// Scale down chained sells when pool liquidity is thin and redistribute overflow.
    #[serde(default)]
    pub liquidity_guard: bool,
    /// Move stop loss to breakeven once profit reaches this percentage (0 = disabled).
    #[serde(default)]
    pub breakeven_trail_pct: f64,
}

/// Auto-buy configuration for a watched wallet.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AutoBuyConfigMsg {
    /// Which of the user's own wallets to execute the buy on.
    pub wallet_pubkey: String,
    /// Amount to spend in SOL lamports when the watched wallet buys in a SOL market.
    pub amount_quote_units: u64,
    /// Amount to spend in USD1 base units when the watched wallet buys in a USD1 market.
    /// When `None`, USD1 markets are skipped for auto-buy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount_usd1_units: Option<u64>,
}

/// A single watched wallet entry with optional auto-buy mirror config.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WatchWalletEntryMsg {
    /// Solana pubkey of the external wallet to watch.
    pub pubkey: String,
    /// Optional auto-buy config. When set, the stream triggers a buy on
    /// the user's own wallet whenever the watched wallet opens a position.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_buy: Option<AutoBuyConfigMsg>,
    /// When true, the stream also mirrors the watched wallet's sells by
    /// triggering a full exit on the user's corresponding mirror position.
    #[serde(default)]
    pub mirror_sell: bool,
}

/// Mirror trading hardening configuration sent during Configure.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct MirrorConfigMsg {
    /// Max concurrent mirror positions per watched wallet (0 = unlimited).
    #[serde(default)]
    pub max_positions_per_wallet: u32,
    /// Cooldown between mirror buys in seconds (0 = no cooldown).
    #[serde(default)]
    pub cooldown_sec: u64,
    /// Skip tokens created/deployed by the watched wallet.
    #[serde(default)]
    pub skip_creator_tokens: bool,
    /// Max total SOL deployed across all active mirror positions (0.0 = unlimited).
    #[serde(default)]
    pub max_active_sol: f64,
    /// Slippage tolerance for mirror buys in basis points (0 = use default).
    #[serde(default)]
    pub buy_slippage_bps: u16,
    /// Minimum pool liquidity in SOL to allow a mirror buy (None = disabled).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_liquidity_sol: Option<f64>,
    /// Maximum price drift % from watched wallet's entry before skipping (None = disabled).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_entry_drift_pct: Option<f64>,
    /// Auto-disable watched wallet after N consecutive losing mirror trades (None = disabled).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_consecutive_losses: Option<u32>,
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
    /// Max external wallets that can be watched per session (copy trading).
    #[serde(default)]
    pub max_watch_wallets_per_session: u32,
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
        /// How the client will submit the signed transaction (e.g. "helius_sender", "rpc", "astralane").
        #[serde(default)]
        send_mode: Option<String>,
        /// Priority fee tip in lamports (required for some send modes).
        #[serde(default)]
        tip_lamports: Option<u64>,
        /// External wallets to watch for copy trading (tier 1+).
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        watch_wallets: Vec<WatchWalletEntryMsg>,
        /// Mirror trading hardening configuration.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        mirror_config: Option<MirrorConfigMsg>,
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
    /// Replace the set of monitored wallets for an active session.
    UpdateWallets {
        /// Full replacement list of wallet pubkeys.
        #[serde(deserialize_with = "deserialize_wallet_pubkeys")]
        wallet_pubkeys: Vec<String>,
    },
    /// Replace the set of watched external wallets for copy trading.
    UpdateWatchWallets {
        /// Full replacement list of watch wallet entries.
        watch_wallets: Vec<WatchWalletEntryMsg>,
    },
    /// Override the strategy for a single active position.
    ///
    /// The provided strategy fully replaces the session-level strategy for this
    /// position only. Other positions are unaffected.
    UpdatePositionStrategy {
        /// The position to override.
        position_id: u64,
        /// Full strategy replacement for this position.
        strategy: StrategyConfigMsg,
    },
    /// Report the outcome of a mirror buy transaction back to the stream.
    ///
    /// Sent by the desktop app after signing and submitting (or failing to submit)
    /// a `MirrorBuySignal` transaction. Allows the stream to immediately clear
    /// pending state (dedup locks, pending tags) instead of waiting for the
    /// periodic cleanup sweep.
    MirrorBuyResult {
        /// Token mint pubkey that the mirror buy targeted.
        mint: String,
        /// True if the transaction was successfully submitted to the network.
        /// False if signing or submission failed.
        success: bool,
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
        /// Current token price in quote units.
        #[serde(skip_serializing_if = "Option::is_none")]
        token_price_quote: Option<u64>,
        /// Current market cap in quote units.
        #[serde(skip_serializing_if = "Option::is_none")]
        market_cap_quote: Option<u64>,
        /// True when this position belongs to a watched (copy-traded) wallet.
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        watched: bool,
    },
    /// Liquidity snapshot for a position with slippage bands.
    LiquiditySnapshot {
        /// Internal position identifier.
        position_id: u64,
        /// Slippage bands describing sellable amounts at each threshold.
        bands: Vec<SlippageBandMsg>,
        /// Liquidity trend: "growing", "stable", or "draining".
        liquidity_trend: String,
        /// Server timestamp in Unix milliseconds.
        server_time_ms: u64,
        /// True when this position belongs to a watched (copy-traded) wallet.
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        watched: bool,
    },
    /// A trade (swap) observed on the pool for a tracked position.
    TradeTick {
        /// Internal position identifier.
        position_id: u64,
        /// Unix timestamp in milliseconds when the trade was observed.
        time_ms: u64,
        /// Trade direction: "buy" or "sell".
        side: String,
        /// Token amount traded in native units.
        token_amount: u64,
        /// Quote (SOL) amount traded in lamports.
        quote_amount: u64,
        /// Price per token in lamports.
        price_quote: u64,
        /// Wallet that initiated the swap.
        #[serde(skip_serializing_if = "Option::is_none")]
        maker: Option<String>,
        /// Transaction signature (base58-encoded).
        #[serde(skip_serializing_if = "Option::is_none")]
        tx_signature: Option<String>,
        /// True when this position belongs to a watched (copy-traded) wallet.
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        watched: bool,
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
        /// Human-readable token name (e.g. "Lamppoli").
        #[serde(skip_serializing_if = "Option::is_none")]
        token_name: Option<String>,
        /// Token ticker symbol (e.g. "LAMP").
        #[serde(skip_serializing_if = "Option::is_none")]
        token_symbol: Option<String>,
        /// Token decimal places for display.
        #[serde(skip_serializing_if = "Option::is_none")]
        token_decimals: Option<u8>,
        /// Token price in quote units at time of open.
        #[serde(skip_serializing_if = "Option::is_none")]
        token_price_quote: Option<u64>,
        /// Market cap in quote units at time of open.
        #[serde(skip_serializing_if = "Option::is_none")]
        market_cap_quote: Option<u64>,
        /// Pool liquidity in quote units at time of open.
        #[serde(skip_serializing_if = "Option::is_none")]
        pool_liquidity_quote: Option<u64>,
        /// Server timestamp when the position opened, in Unix milliseconds.
        #[serde(skip_serializing_if = "Option::is_none")]
        opened_at_ms: Option<u64>,
        /// Slot when the position opened.
        slot: u64,
        /// True when this position belongs to a watched (copy-traded) wallet.
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        watched: bool,
        /// Identifier of the mirror source that triggered this position (e.g. watched wallet pubkey).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        mirror_source: Option<String>,
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
        /// Identifier of the mirror source that triggered this position (e.g. watched wallet pubkey).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        mirror_source: Option<String>,
        /// Slot when the position closed.
        slot: u64,
        /// True when this position belongs to a watched (copy-traded) wallet.
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        watched: bool,
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
        /// Tokens being sold (present for partial/chained sells).
        #[serde(skip_serializing_if = "Option::is_none")]
        sell_tokens: Option<u64>,
        /// Which chained take-profit level fired (0-indexed).
        #[serde(skip_serializing_if = "Option::is_none")]
        level_index: Option<u32>,
        /// Identifier of the mirror source that triggered this position (e.g. watched wallet pubkey).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        mirror_source: Option<String>,
        /// True when this position belongs to a watched (copy-traded) wallet.
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        watched: bool,
    },
    /// Unsigned buy transaction triggered by a watched wallet's purchase.
    ///
    /// Sent when a watched wallet opens a position and the mirror buy passes
    /// all hardening checks. The client should sign and submit the transaction,
    /// then send a `MirrorBuyResult` message back to report the outcome.
    MirrorBuySignal {
        /// Session identifier for correlation.
        session_id: u64,
        /// Watched wallet that triggered this mirror buy.
        watched_wallet: String,
        /// Token mint pubkey being bought.
        mint: String,
        /// User's own wallet that will execute the buy.
        user_wallet: String,
        /// Amount to spend in quote units (lamports for SOL, base units for USD1).
        amount_quote_units: u64,
        /// Quote asset: "SOL" or "USD1".
        input: String,
        /// Base64-encoded unsigned buy transaction.
        unsigned_tx_b64: String,
        /// Slippage tolerance in basis points.
        slippage_bps: u16,
        /// Transaction send mode (e.g. "helius_sender", "rpc", "astralane").
        #[serde(skip_serializing_if = "Option::is_none")]
        send_mode: Option<String>,
        /// Priority fee tip in lamports.
        #[serde(skip_serializing_if = "Option::is_none")]
        tip_lamports: Option<u64>,
        /// Optional market metadata.
        #[serde(skip_serializing_if = "Option::is_none")]
        market_context: Option<MarketContextMsg>,
    },
    /// Notification that a mirror buy could not be executed.
    MirrorBuyFailed {
        /// Watched wallet that triggered the failed buy.
        watched_wallet: String,
        /// Token mint pubkey that was targeted.
        mint: String,
        /// Human-readable reason for the failure.
        reason: String,
    },
    /// Notification that a watched wallet has been auto-disabled.
    MirrorWalletAutoDisabled {
        /// Watched wallet pubkey that was disabled.
        watched_wallet: String,
        /// Reason for auto-disable (e.g. "consecutive_losses").
        reason: String,
        /// Number of consecutive losses that triggered the disable.
        loss_count: u32,
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
                trailing_stop_pct: 0.0,
                sell_on_graduation: false,
                take_profit_levels: vec![],
                liquidity_guard: false,
                breakeven_trail_pct: 0.0,
            },
            send_mode: None,
            tip_lamports: None,
            watch_wallets: vec![],
            mirror_config: None,
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
                "stop_loss_pct":1.5
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
                    trailing_stop_pct: 0.0,
                    sell_on_graduation: false,
                    take_profit_levels: vec![],
                    liquidity_guard: false,
                    breakeven_trail_pct: 0.0,
                },
                send_mode: None,
                tip_lamports: None,
                watch_wallets: vec![],
                mirror_config: None,
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
                max_watch_wallets_per_session: 10,
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
            sell_tokens: None,
            level_index: None,
            mirror_source: None,
            watched: false,
        };
        round_trip(msg);
    }

    #[test]
    fn strategy_with_take_profit_levels_round_trip() {
        let msg = ClientMessage::Configure {
            wallet_pubkeys: vec!["11111111111111111111111111111111".to_string()],
            strategy: StrategyConfigMsg {
                target_profit_pct: 0.0,
                stop_loss_pct: 5.0,
                trailing_stop_pct: 10.0,
                sell_on_graduation: false,
                take_profit_levels: vec![
                    TakeProfitLevelMsg {
                        profit_pct: 50.0,
                        sell_pct: 30.0,
                        trailing_stop_pct: 5.0,
                    },
                    TakeProfitLevelMsg {
                        profit_pct: 100.0,
                        sell_pct: 30.0,
                        trailing_stop_pct: 3.0,
                    },
                ],
                liquidity_guard: true,
                breakeven_trail_pct: 0.0,
            },
            send_mode: None,
            tip_lamports: None,
            watch_wallets: vec![],
            mirror_config: None,
        };
        round_trip(msg);
    }

    #[test]
    fn strategy_without_levels_backward_compatible() {
        let raw = r#"{
            "type":"configure",
            "wallet_pubkeys":["11111111111111111111111111111111"],
            "strategy":{
                "target_profit_pct":25.0,
                "stop_loss_pct":5.0
            }
        }"#;
        let msg: ClientMessage = serde_json::from_str(raw).expect("deserialize");
        if let ClientMessage::Configure { strategy, .. } = msg {
            assert!(strategy.take_profit_levels.is_empty());
            assert_eq!(strategy.target_profit_pct, 25.0);
        } else {
            panic!("expected Configure");
        }
    }

    #[test]
    fn exit_signal_with_partial_sell_round_trip() {
        let msg = ServerMessage::ExitSignalWithTx {
            session_id: 1,
            position_id: 2,
            wallet_pubkey: "11111111111111111111111111111111".to_string(),
            mint: "22222222222222222222222222222222".to_string(),
            token_account: None,
            token_program: None,
            position_tokens: 1000,
            profit_units: 50,
            reason: "chained_tp".to_string(),
            triggered_at_ms: 999,
            market_context: None,
            unsigned_tx_b64: "dGVzdA==".to_string(),
            sell_tokens: Some(300),
            level_index: Some(0),
            mirror_source: None,
            watched: false,
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
            token_price_quote: None,
            market_cap_quote: None,
            watched: false,
        };
        round_trip(msg);
    }

    #[test]
    fn trade_tick_round_trip() {
        let msg = ServerMessage::TradeTick {
            position_id: 7,
            time_ms: 1700000000123,
            side: "buy".to_string(),
            token_amount: 50_000_000,
            quote_amount: 1_200_000_000,
            price_quote: 24_000,
            maker: Some("11111111111111111111111111111111".to_string()),
            tx_signature: Some("22222222222222222222222222222222".to_string()),
            watched: false,
        };
        round_trip(msg);
    }

    #[test]
    fn liquidity_snapshot_round_trip() {
        let msg = ServerMessage::LiquiditySnapshot {
            position_id: 5,
            bands: vec![
                SlippageBandMsg {
                    slippage_bps: 100,
                    max_tokens: 5000,
                    coverage_pct: 50.0,
                },
                SlippageBandMsg {
                    slippage_bps: 500,
                    max_tokens: 10000,
                    coverage_pct: 100.0,
                },
            ],
            liquidity_trend: "growing".to_string(),
            server_time_ms: 999,
            watched: false,
        };
        round_trip(msg);
    }
}
