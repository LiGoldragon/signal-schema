//! Typed binary public contract for TypeSchema first and the accepted document roots.
use rkyv::{Archive,Deserialize,Serialize};
use signal_sema_storage::{ContentHash,DeclarationRoot,DocumentKind,FixtureScope,SlotIdentifier,SlotSummary};
#[derive(Archive,Serialize,Deserialize,Clone,Debug,PartialEq,Eq)]
pub enum Request {
 IngestTypeSchema { scope:FixtureScope, slot:SlotIdentifier, legacy_text:String },
 StoreDocumentRoot { scope:FixtureScope, slot:SlotIdentifier, root:DeclarationRoot },
 List { scope:FixtureScope, kind:Option<DocumentKind> },
 Fetch { hash:ContentHash },
 Subscribe { scope:FixtureScope, kind:Option<DocumentKind> },
}
#[derive(Archive,Serialize,Deserialize,Clone,Debug,PartialEq,Eq)]
pub enum Reply { Stored(SlotSummary), Listed(Vec<SlotSummary>), Fetched(Option<SlotSummary>), Subscribed, Rejected(Rejection) }
#[derive(Archive,Serialize,Deserialize,Clone,Copy,Debug,PartialEq,Eq)]
pub enum Rejection { InvalidTypeSchema, RootKindMismatch, StorageFailed }
pub fn encode_request(value:&Request)->Result<Vec<u8>,String>{rkyv::to_bytes::<rkyv::rancor::Error>(value).map(|b|b.to_vec()).map_err(|e|e.to_string())}
