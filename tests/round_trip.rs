//! Round-trip witnesses for the schema-derived `signal-schema` contract.

use nota::{NotaDecode, NotaEncode, NotaSource};
use signal_frame::{
    ExchangeIdentifier, ExchangeLane, LaneSequence, NonEmpty, Reply, SessionEpoch, SubReply,
};
use signal_schema::{
    EmitRust, Frame, FrameBody, Input, LoadedModules, ModuleName, OperationKind, Output,
    PackageLoaded, PackageName, PackageVersion, Rejected, RejectionDetail, RejectionReason,
    RustEmitted, RustText, SchemaModuleSource, SchemaModuleSources, SchemaSlotIdentifier,
    SchemaText,
};

const MINIMAL_SCHEMA: &str = include_str!("fixtures/minimal.schema");

fn exchange() -> ExchangeIdentifier {
    ExchangeIdentifier::new(
        SessionEpoch::new(1),
        ExchangeLane::Connector,
        LaneSequence::first(),
    )
}

fn slot() -> SchemaSlotIdentifier {
    SchemaSlotIdentifier::new("live")
}

fn package_name() -> PackageName {
    PackageName::new("signal-schema")
}

fn package_version() -> PackageVersion {
    PackageVersion::new("0.1.0")
}

fn module_name() -> ModuleName {
    ModuleName::new("lib")
}

fn package_sources() -> SchemaModuleSources {
    SchemaModuleSources::new(vec![SchemaModuleSource {
        module_name: module_name(),
        schema_text: SchemaText::new(MINIMAL_SCHEMA),
    }])
}

fn load_package() -> signal_schema::LoadPackage {
    signal_schema::LoadPackage {
        schema_slot_identifier: slot(),
        package_name: package_name(),
        package_version: package_version(),
        schema_module_sources: package_sources(),
    }
}

fn round_trip_request(request: Input) -> Input {
    let expected = request.clone();
    let frame = request.into_frame(exchange());
    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");
    match decoded.into_body() {
        FrameBody::Request { request, .. } => {
            assert_eq!(request.payloads().head(), &expected);
            request.payloads().head().clone()
        }
        other => panic!("expected request operation, got {other:?}"),
    }
}

fn round_trip_reply(reply: Output) -> Output {
    let frame = Frame::new(FrameBody::Reply {
        exchange: exchange(),
        reply: Reply::committed(NonEmpty::single(SubReply::Ok(reply))),
    });
    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");
    match decoded.into_body() {
        FrameBody::Reply { reply, .. } => match reply {
            Reply::Accepted { per_operation, .. } => match per_operation.into_head() {
                SubReply::Ok(payload) => payload,
                other => panic!("expected accepted reply payload, got {other:?}"),
            },
            other => panic!("expected accepted reply, got {other:?}"),
        },
        other => panic!("expected reply operation, got {other:?}"),
    }
}

fn round_trip_nota<T>(value: T, expected: &str)
where
    T: NotaEncode + NotaDecode + PartialEq + std::fmt::Debug,
{
    let encoded = value.to_nota();
    assert_eq!(encoded, expected);
    let recovered = NotaSource::new(&encoded)
        .parse::<T>()
        .expect("decode nota text");
    assert_eq!(recovered, value);
}

#[test]
fn every_request_round_trips_through_streaming_frame() {
    let requests = [
        Input::LoadPackage(load_package()),
        Input::EmitRust(EmitRust {
            schema_slot_identifier: slot(),
            module_name: module_name(),
        }),
    ];

    for request in requests {
        assert_eq!(round_trip_request(request.clone()), request);
    }
}

#[test]
fn every_reply_round_trips_through_streaming_frame() {
    let replies = [
        Output::PackageLoaded(PackageLoaded {
            schema_slot_identifier: slot(),
            package_name: package_name(),
            package_version: package_version(),
            loaded_modules: LoadedModules::new(vec![module_name()]),
        }),
        Output::RustEmitted(RustEmitted {
            schema_slot_identifier: slot(),
            module_name: module_name(),
            rust_text: RustText::new("pub struct Topic(String);"),
        }),
        Output::Rejected(Rejected {
            operation_kind: OperationKind::EmitRust,
            rejection_reason: RejectionReason::PackageNotLoaded,
            rejection_detail: RejectionDetail::new("slot not loaded"),
        }),
    ];

    for reply in replies {
        assert_eq!(round_trip_reply(reply.clone()), reply);
    }
}

#[test]
fn nota_examples_round_trip() {
    round_trip_nota(
        Input::EmitRust(EmitRust {
            schema_slot_identifier: slot(),
            module_name: module_name(),
        }),
        "(EmitRust (live lib))",
    );
    round_trip_nota(
        Output::PackageLoaded(PackageLoaded {
            schema_slot_identifier: slot(),
            package_name: package_name(),
            package_version: package_version(),
            loaded_modules: LoadedModules::new(vec![module_name()]),
        }),
        "(PackageLoaded (live signal-schema 0.1.0 [lib]))",
    );
    round_trip_nota(
        Output::Rejected(Rejected {
            operation_kind: OperationKind::EmitRust,
            rejection_reason: RejectionReason::PackageNotLoaded,
            rejection_detail: RejectionDetail::new("slot not loaded"),
        }),
        "(Rejected (EmitRust PackageNotLoaded [slot not loaded]))",
    );
}
