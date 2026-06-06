//! Witness tests for the meta-signal-router wire contract.
//!
//! Proves the schema-emitted policy operations and replies round-trip
//! through the rkyv archive and through NOTA text, that each variant maps
//! to its contract-local short header, and that meta-order names never
//! appear in the routed-message-kind vocabulary.
//!
//! The current refreshed schema-rust-next base emits, for a pure
//! `wire_contract()` target, the wire types + NOTA codec + route
//! witnesses + the `short_header` constant module + signal-frame helpers.
//! The daemon wraps those contract frames in the triad-runtime
//! length-prefixed envelope.

use meta_signal_router::{
    AdjudicationDenial, ChannelDuration, ChannelEndpoint, ChannelExtension, ChannelGrant,
    ChannelMessageKind, ChannelOrderRejectionReason, ChannelRevocation, ComponentName,
    ConnectionClass, DeniedAdjudication, ExtendedChannel, GrantedChannel, Input, OperationKind,
    Output, RejectedChannelOrder, RevokedChannel, UnimplementedReason, UnimplementedRequest,
    short_header,
};

fn grant() -> ChannelGrant {
    ChannelGrant {
        source: ChannelEndpoint::External(ConnectionClass::Owner),
        destination: ChannelEndpoint::Internal(ComponentName::Router),
        kinds: vec![
            ChannelMessageKind::MessageSubmission,
            ChannelMessageKind::InboxQuery,
        ],
        duration: ChannelDuration::Permanent,
    }
}

fn inputs() -> Vec<Input> {
    vec![
        Input::Grant(grant()),
        Input::Extend(ChannelExtension {
            channel: "channel-aab".to_owned(),
            duration: ChannelDuration::TimeBound(1_730_000_000_000_000_000),
        }),
        Input::Revoke(ChannelRevocation {
            channel: "channel-aab".to_owned(),
            reason: "operator closed the path".to_owned(),
        }),
        Input::Deny(AdjudicationDenial {
            request: "adjudication-aab".to_owned(),
            reason: "destination unavailable".to_owned(),
        }),
    ]
}

fn outputs() -> Vec<Output> {
    vec![
        Output::ChannelGranted(GrantedChannel("channel-aab".to_owned())),
        Output::ChannelExtended(ExtendedChannel("channel-aab".to_owned())),
        Output::ChannelRevoked(RevokedChannel("channel-aab".to_owned())),
        Output::AdjudicationDenied(DeniedAdjudication("adjudication-aab".to_owned())),
        Output::ChannelOrderRejected(RejectedChannelOrder {
            operation: OperationKind::Grant,
            reason: ChannelOrderRejectionReason::PolicyRefused,
        }),
        Output::RequestUnimplemented(UnimplementedRequest {
            operation: OperationKind::Grant,
            reason: UnimplementedReason::NotBuiltYet,
        }),
    ]
}

#[test]
fn input_operations_round_trip_through_rkyv() {
    for input in inputs() {
        let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&input).expect("rkyv encode input");
        let recovered =
            rkyv::from_bytes::<Input, rkyv::rancor::Error>(&bytes).expect("rkyv decode input");
        assert_eq!(recovered, input);
    }
}

#[test]
fn output_replies_round_trip_through_rkyv() {
    for output in outputs() {
        let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&output).expect("rkyv encode output");
        let recovered =
            rkyv::from_bytes::<Output, rkyv::rancor::Error>(&bytes).expect("rkyv decode output");
        assert_eq!(recovered, output);
    }
}

#[test]
fn input_operations_round_trip_through_signal_frame() {
    for input in inputs() {
        let frame = input
            .encode_signal_frame()
            .expect("encode input signal frame");
        let (route, recovered) =
            Input::decode_signal_frame(&frame).expect("decode input signal frame");
        assert_eq!(route, input.route());
        assert_eq!(recovered, input);
    }
}

#[test]
fn output_replies_round_trip_through_signal_frame() {
    for output in outputs() {
        let frame = output
            .encode_signal_frame()
            .expect("encode output signal frame");
        let (route, recovered) =
            Output::decode_signal_frame(&frame).expect("decode output signal frame");
        assert_eq!(route, output.route());
        assert_eq!(recovered, output);
    }
}

#[test]
fn short_headers_are_contract_local_and_distinct() {
    let headers = [
        short_header::INPUT_GRANT,
        short_header::INPUT_EXTEND,
        short_header::INPUT_REVOKE,
        short_header::INPUT_DENY,
        short_header::OUTPUT_CHANNEL_GRANTED,
        short_header::OUTPUT_CHANNEL_EXTENDED,
        short_header::OUTPUT_CHANNEL_REVOKED,
        short_header::OUTPUT_ADJUDICATION_DENIED,
        short_header::OUTPUT_CHANNEL_ORDER_REJECTED,
        short_header::OUTPUT_REQUEST_UNIMPLEMENTED,
    ];
    for (outer_index, outer) in headers.iter().enumerate() {
        for (inner_index, inner) in headers.iter().enumerate() {
            if outer_index != inner_index {
                assert_ne!(outer, inner, "short headers must be distinct");
            }
        }
    }
}

#[cfg(feature = "nota-text")]
#[test]
fn input_operations_round_trip_through_nota_text() {
    for input in inputs() {
        let text = input.to_nota();
        let recovered: Input = text.parse().expect("parse input nota");
        assert_eq!(recovered, input);
        assert!(!text.contains("Mutate"));
        assert!(!text.contains("Retract"));
        assert!(!text.contains("Assert"));
    }
}

#[cfg(feature = "nota-text")]
#[test]
fn output_replies_round_trip_through_nota_text() {
    for output in outputs() {
        let text = output.to_nota();
        let recovered: Output = text.parse().expect("parse output nota");
        assert_eq!(recovered, output);
    }
}

#[cfg(feature = "nota-text")]
#[test]
fn grant_operation_encodes_as_contract_local_nota_head() {
    let text = Input::Grant(grant()).to_nota();
    assert!(
        text.starts_with("(Grant"),
        "expected Grant head, got {text}"
    );
    let recovered: Input = text.parse().expect("parse grant nota");
    assert_eq!(recovered, Input::Grant(grant()));
}

#[cfg(feature = "nota-text")]
#[test]
fn meta_order_names_are_not_channel_message_kinds() {
    let kinds = [
        ChannelMessageKind::MessageIngressSubmission,
        ChannelMessageKind::MessageSubmission,
        ChannelMessageKind::InboxQuery,
        ChannelMessageKind::FocusObservation,
        ChannelMessageKind::PromptBufferObservation,
        ChannelMessageKind::MessageDelivery,
        ChannelMessageKind::TerminalInput,
        ChannelMessageKind::TerminalCapture,
        ChannelMessageKind::TerminalResize,
        ChannelMessageKind::TranscriptEvent,
        ChannelMessageKind::AdjudicationRequest,
        ChannelMessageKind::DeliveryNotification,
    ];
    for kind in kinds {
        let text = kind.to_nota();
        assert_ne!(text, "ChannelGrant");
        assert_ne!(text, "ChannelExtend");
        assert_ne!(text, "ChannelRevoke");
        assert_ne!(text, "AdjudicationDeny");
        assert_ne!(text, "AdjudicationDenial");
        assert_ne!(text, "Grant");
        assert_ne!(text, "Extend");
        assert_ne!(text, "Revoke");
        assert_ne!(text, "Deny");
    }
}
