//! Typed binary public contract for TypeSchema first and the accepted document roots.
mod error;

pub use error::Error;
use rkyv::{Archive, Deserialize, Serialize};
use signal_sema_storage::{
    ChangeEvent, ContentHash, DocumentKind, FixtureScope, NexusRuntimeRoot, SemaStorageRoot,
    SignalContractRoot, SlotIdentifier, SlotSummary, SubscriptionIdentifier,
};

#[derive(Archive, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum Request {
    IngestTypeSchema {
        scope: FixtureScope,
        slot: SlotIdentifier,
        legacy_text: String,
    },
    StoreSignalContract {
        scope: FixtureScope,
        slot: SlotIdentifier,
        root: SignalContractRoot,
    },
    StoreNexusRuntime {
        scope: FixtureScope,
        slot: SlotIdentifier,
        root: NexusRuntimeRoot,
    },
    StoreSemaStorage {
        scope: FixtureScope,
        slot: SlotIdentifier,
        root: SemaStorageRoot,
    },
    List {
        scope: FixtureScope,
        kind: Option<DocumentKind>,
    },
    Fetch {
        hash: ContentHash,
    },
    Subscribe {
        scope: FixtureScope,
        kind: Option<DocumentKind>,
    },
}
#[derive(Archive, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum Reply {
    Stored(SlotSummary),
    Listed(Vec<SlotSummary>),
    Fetched(Option<SlotSummary>),
    Subscribed {
        identifier: SubscriptionIdentifier,
        initial: Vec<SlotSummary>,
    },
    Event(ChangeEvent),
    Rejected(Rejection),
}
#[derive(Archive, Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rejection {
    InvalidTypeSchema,
    InvalidRoot,
    StorageFailed,
}
pub fn encode_request(value: &Request) -> Result<Vec<u8>, Error> {
    rkyv::to_bytes::<rkyv::rancor::Error>(value)
        .map(|bytes| bytes.to_vec())
        .map_err(Error::from)
}
pub fn encode_reply(value: &Reply) -> Result<Vec<u8>, Error> {
    rkyv::to_bytes::<rkyv::rancor::Error>(value)
        .map(|bytes| bytes.to_vec())
        .map_err(Error::from)
}
