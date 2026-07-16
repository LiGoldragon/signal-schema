use signal_schema::{Request, encode_request};
use signal_sema_storage::{FixtureScope, SlotIdentifier};
#[test]
fn type_schema_ingest_round_trips() {
    let value = Request::IngestTypeSchema {
        scope: FixtureScope(1),
        slot: SlotIdentifier(1),
        legacy_text: "{}".into(),
    };
    let bytes = encode_request(&value).unwrap();
    assert_eq!(
        rkyv::from_bytes::<Request, rkyv::rancor::Error>(&bytes).unwrap(),
        value
    )
}
