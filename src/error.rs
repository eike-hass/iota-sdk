// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Error handling in iota-client crate.

/// Type alias of `Result` in iota-client
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
/// Error type of the iota client crate.
#[allow(clippy::large_enum_variant)]
pub enum Error {
    /// Error when building indexation messages
    #[error("Error when building indexation message: {0}")]
    IndexationError(String),
    /// Error when building transaction messages
    #[error("Error when building transaction message")]
    TransactionError,
    /// The wallet account doesn't have enough balance
    #[error("The wallet account doesn't have enough balance. It only has {0}, required is {1}")]
    NotEnoughBalance(u64, u64),
    /// The wallet account doesn't have enough balance
    #[error(
        "The wallet account has enough funds, but splitted on too many outputs: {0}, max. is 127, consolidate them"
    )]
    ConsolidationRequired(usize),
    /// Dust error, for example not enough balance on an address
    #[error("Dust error: {0}")]
    DustError(String),
    /// Missing required parameters
    #[error("Must provide required parameter: {0}")]
    MissingParameter(&'static str),
    /// Invalid parameters
    #[error("Parameter is invalid:{0}")]
    InvalidParameter(&'static str),
    /// No node available in the synced node pool
    #[error("No synced node available")]
    SyncedNodePoolEmpty,
    /// Error on Url type conversion
    #[error("Failed to parse node_pool_urls")]
    NodePoolUrlsError,
    /// Error on reaching quorum
    #[error("Failed to reach quorum {0} {1}")]
    QuorumThresholdError(usize, usize),
    /// Error on quorum because not enough nodes are available
    #[error("Not enough nodes for quorum {0} {1}")]
    QuorumPoolSizeError(usize, usize),
    /// Error on API request
    #[error("Node error: {0}")]
    NodeError(String),
    /// Error on RwLock read
    #[error("Failed to read node RwLock")]
    NodeReadError,
    /// Hex string convert error
    #[error("{0}")]
    FromHexError(#[from] hex::FromHexError),
    /// Message types error
    #[error("{0}")]
    MessageError(#[from] bee_message::Error),
    /// Bee rest api error
    #[error("{0}")]
    BeeRestApiError(#[from] bee_rest_api::types::error::Error),
    /// The message doensn't need to be promoted or reattached
    #[error("Message ID `{0}` doesn't need to be promoted or reattached")]
    NoNeedPromoteOrReattach(String),
    /// The message cannot be included into the Tangle
    #[error("Message ID `{0}` couldn't get included into the Tangle")]
    TangleInclusionError(String),
    /// Mqtt client error
    #[cfg(feature = "mqtt")]
    #[error("{0}")]
    MqttClientError(#[from] rumqttc::ClientError),
    /// Invalid MQTT topic.
    #[error("The MQTT topic {0} is invalid")]
    InvalidMqttTopic(String),
    /// MQTT connection not found (all nodes MQTT's are disabled)
    #[error("MQTT connection not found (all nodes have the MQTT plugin disabled)")]
    MqttConnectionNotFound,
    /// IO error
    #[error("{0}")]
    IoError(#[from] std::io::Error),
    /// JSON error
    #[error("{0}")]
    Json(#[from] serde_json::Error),
    /// PoW error
    #[error("{0}")]
    Pow(String),
    /// Address not found
    #[error("Address: {0} not found in range: {1}")]
    InputAddressNotFound(String, String),
    /// Crypto.rs error
    #[error("{0}")]
    CryptoError(#[from] crypto::Error),
    /// Crypto.rs mnemonic error
    #[error("{0}")]
    MnemonicError(String),
    /// Invalid amount of parents
    #[error("Invalid amount of parents: {0}, length must be in 1..=8")]
    InvalidParentsAmount(usize),
    /// ureq error
    #[cfg(feature = "sync")]
    #[error("{0}")]
    UreqError(#[from] ureq::Error),
    /// Error from RestAPI calls with unexpected status code response
    #[cfg(any(feature = "async", feature = "wasm"))]
    #[error("Response error with status code {0}: {1}")]
    ResponseError(u16, String),
    /// reqwest error
    #[cfg(any(feature = "async", feature = "wasm"))]
    #[error("{0}")]
    ReqwestError(#[from] reqwest::Error),
    /// URL error
    #[error("{0}")]
    UrlError(#[from] url::ParseError),
    /// URL validation error
    #[error("{0}")]
    UrlValidationError(String),
    /// URL auth error
    #[error("Can't set {0} to URL")]
    UrlAuthError(String),
    /// Blake2b256 Error
    #[error("{0}")]
    Blake2b256Error(&'static str),
    /// Output Error
    #[error("Output error: {0}")]
    OutputError(&'static str),
    #[cfg(not(feature = "wasm"))]
    /// Tokio task join error
    #[error("{0}")]
    TaskJoinError(#[from] tokio::task::JoinError),
    /// Invalid mnemonic error
    #[error("Invalid mnemonic {0}")]
    InvalidMnemonic(String),
    /// PoW error
    #[error("{0}")]
    PowError(#[from] bee_pow::providers::miner::Error),
    /// API error
    #[error("Invalid API name")]
    ApiError,
    /// Rw lock failed.
    #[error("Rw lock failed")]
    PoisonError,
    /// Missing unlock block error
    #[error("missing unlock block")]
    MissingUnlockBlock,
    /// Ledger transport error
    #[cfg(feature = "ledger")]
    #[error("ledger transport error")]
    LedgerMiscError,
    /// Dongle Locked
    #[cfg(feature = "ledger")]
    #[error("ledger locked")]
    LedgerDongleLocked,
    /// Denied by User
    #[cfg(feature = "ledger")]
    #[error("denied by user")]
    LedgerDeniedByUser,
    /// Ledger Device not found
    #[cfg(feature = "ledger")]
    #[error("ledger device not found")]
    LedgerDeviceNotFound,
    /// Ledger Essence Too Large
    #[cfg(feature = "ledger")]
    #[error("ledger essence too large")]
    LedgerEssenceTooLarge,
    /// Ledger transport error
    #[cfg(feature = "ledger")]
    #[error("ledger app compiled for testnet but used with mainnet or vice versa")]
    LedgerNetMismatch,
    /// Wrong ledger seed error
    #[cfg(feature = "ledger")]
    #[error("ledger mnemonic is mismatched")]
    LedgerMnemonicMismatch,
}

// map most errors to a single error but there are some errors that
// need special care.
// LedgerDongleLocked: Ask the user to unlock the dongle
// LedgerDeniedByUser: The user denied a signing
// LedgerDeviceNotFound: No usable Ledger device was found
// LedgerMiscError: Everything else.
// LedgerEssenceTooLarge: Essence with bip32 input indices need more space then the internal buffer is big
#[cfg(feature = "ledger")]
impl From<iota_ledger::api::errors::APIError> for Error {
    fn from(error: iota_ledger::api::errors::APIError) -> Self {
        log::info!("ledger error: {}", error);
        match error {
            iota_ledger::api::errors::APIError::SecurityStatusNotSatisfied => Error::LedgerDongleLocked,
            iota_ledger::api::errors::APIError::ConditionsOfUseNotSatisfied => Error::LedgerDeniedByUser,
            iota_ledger::api::errors::APIError::TransportError => Error::LedgerDeviceNotFound,
            iota_ledger::api::errors::APIError::EssenceTooLarge => Error::LedgerEssenceTooLarge,
            _ => Error::LedgerMiscError,
        }
    }
}
