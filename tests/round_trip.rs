use signal_schema::{Error, Rejection, Reply, Request, encode_reply, encode_request};
use signal_sema_storage::{FixtureScope, SlotIdentifier};

#[test]
fn type_schema_ingest_round_trips_through_typed_codec_boundary() {
    let value = Request::IngestTypeSchema {
        scope: FixtureScope(1),
        slot: SlotIdentifier(1),
        legacy_text: "{}".into(),
    };
    let bytes = match encode_request(&value) {
        Ok(bytes) => bytes,
        Err(Error::Encoding(source)) => panic!("typed archive error: {source}"),
    };
    assert_eq!(
        rkyv::from_bytes::<Request, rkyv::rancor::Error>(&bytes).unwrap(),
        value
    );

    let reply: Result<Vec<u8>, Error> = encode_reply(&Reply::Rejected(Rejection::InvalidRoot));
    assert!(reply.is_ok());
}
