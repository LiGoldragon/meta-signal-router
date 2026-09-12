use meta_signal_router::{
    AdjudicationDenial, ByteViewable, ChannelDuration, ChannelEndpoint, ChannelExtension,
    ChannelGrant, ChannelMessageKind, ChannelOrderRejectionReason, ChannelRevocation,
    ComponentKind, ConnectionClass, NetworkEndpoint, OperationKind, OtherPersonaEngine, Query,
    RejectedChannelOrder, Response, Restorable, Signal, Signalizable, UnimplementedReason,
    UnimplementedRequest,
};

fn grant() -> ChannelGrant {
    ChannelGrant {
        grant_source: ChannelEndpoint::External(ConnectionClass::Network(NetworkEndpoint {
            host_name: String::from("hexis"),
            network_port: 7421,
        })),
        grant_target: ChannelEndpoint::Internal(ComponentKind::Harness),
        channel_message_kinds: vec![
            ChannelMessageKind::MessageDelivery,
            ChannelMessageKind::TranscriptEvent,
        ],
        channel_duration: ChannelDuration::TimeBound(1_757_600_000_000_000_000),
    }
}

fn queries() -> Vec<Query> {
    vec![
        Query::Extend(ChannelExtension {
            channel_identifier: String::from("channel-aab"),
            channel_duration: ChannelDuration::Permanent,
        }),
        Query::Deny(AdjudicationDenial {
            adjudication_request_identifier: String::from("adjudication-7"),
            text_body: String::from("the owner declined this channel"),
        }),
        Query::SetMirrorEnabled(true),
        Query::Revoke(ChannelRevocation {
            channel_identifier: String::from("channel-aab"),
            text_body: String::from("the grant expired"),
        }),
        Query::Grant(grant()),
    ]
}

fn responses() -> Vec<Response> {
    vec![
        Response::ChannelOrderRejected(RejectedChannelOrder {
            operation_kind: OperationKind::Deny,
            channel_order_rejection_reason: ChannelOrderRejectionReason::MetaAuthorityRequired,
        }),
        Response::ChannelGranted(String::from("channel-aab")),
        Response::AdjudicationDenied(String::from("adjudication-7")),
        Response::MirrorEnabledSet(false),
        Response::ChannelRevoked(String::from("channel-aab")),
        Response::ChannelExtended(String::from("channel-aab")),
        Response::RequestUnimplemented(UnimplementedRequest {
            operation_kind: OperationKind::Grant,
            unimplemented_reason: UnimplementedReason::PolicyStoreUnavailable,
        }),
    ]
}

/// Every endpoint class the authority can name, so no arm of the connection
/// vocabulary rides only in the canonical file.
fn endpoint_classes() -> Vec<Query> {
    [
        ConnectionClass::System(String::from("systemd")),
        ConnectionClass::NonOwnerUser(1001),
        ConnectionClass::OtherPersona(OtherPersonaEngine {
            engine_identifier: String::from("engine-2"),
            host_name: String::from("goldragon"),
        }),
        ConnectionClass::Owner,
    ]
    .into_iter()
    .map(|connection_class| {
        Query::Grant(ChannelGrant {
            grant_source: ChannelEndpoint::External(connection_class),
            grant_target: ChannelEndpoint::Internal(ComponentKind::Router),
            channel_message_kinds: Vec::new(),
            channel_duration: ChannelDuration::OneShot,
        })
    })
    .collect()
}

#[test]
fn orders_round_trip_through_received_bytes() {
    for query in queries().into_iter().chain(endpoint_classes()) {
        let received =
            Signal::<Query>::from(query.signalize().expect("order archives").bytes().to_vec());
        assert_eq!(received.restore().expect("order restores"), query);
    }
}

#[test]
fn answers_round_trip_through_received_bytes() {
    for response in responses() {
        let received = Signal::<Response>::from(
            response
                .signalize()
                .expect("answer archives")
                .bytes()
                .to_vec(),
        );
        assert_eq!(received.restore().expect("answer restores"), response);
    }
}

#[test]
fn malformed_peer_bytes_are_rejected() {
    assert!(Signal::<Query>::from(vec![0xff, 0, 1]).restore().is_err());
}

/// `Owner`, `OneShot`, `Permanent` and every `ChannelMessageKind` are bare
/// tags. Ethos Zero gives a variant whose head spells a declared type that
/// type as its payload, so a tag that collided with a type name would silently
/// start carrying one. Carrying them over the wire is the witness that they
/// stay bare.
#[test]
fn bare_heads_cross_the_wire_as_bare_heads() {
    let query = Query::Grant(ChannelGrant {
        grant_source: ChannelEndpoint::External(ConnectionClass::Owner),
        grant_target: ChannelEndpoint::Internal(ComponentKind::Mirror),
        channel_message_kinds: vec![ChannelMessageKind::InboxQuery],
        channel_duration: ChannelDuration::OneShot,
    });
    let received =
        Signal::<Query>::from(query.signalize().expect("order archives").bytes().to_vec());
    assert_eq!(received.restore().expect("order restores"), query);
}

#[cfg(feature = "datom")]
mod datom {
    use super::*;
    use datom_codec::{Actualizing, Budget, Datomizable, Potential};
    use protos::{Protosizable, ReaderBudget, Textualizable};

    /// The ceiling this contract applies to peer-supplied Datom text: one
    /// mebibyte of extent and 256 levels of descent. The wire type constrains
    /// neither, so the reader must.
    const PEER_TEXT_EXTENT: usize = 1024 * 1024;
    const PEER_TEXT_DEPTH: i64 = 256;

    fn budget() -> Budget {
        Budget {
            remaining: i64::try_from(PEER_TEXT_EXTENT).expect("the extent fits an i64"),
            reader: ReaderBudget {
                remaining: PEER_TEXT_EXTENT,
            },
            depth: 0,
            maximum_depth: PEER_TEXT_DEPTH,
        }
    }

    fn textualize<T: Datomizable<Output = datom_codec::Datom>>(value: T) -> String {
        value.datomize(vec![]).protosize().textualize()
    }

    #[test]
    fn orders_round_trip_as_datom_text() {
        for query in queries().into_iter().chain(endpoint_classes()) {
            let text = textualize(query.clone());
            let restored = Potential::<Query>::from(text)
                .actualize(&mut budget())
                .expect("order actualizes");
            assert_eq!(restored, query);
        }
    }

    #[test]
    fn answers_round_trip_as_datom_text() {
        for response in responses() {
            let text = textualize(response.clone());
            let restored = Potential::<Response>::from(text)
                .actualize(&mut budget())
                .expect("answer actualizes");
            assert_eq!(restored, response);
        }
    }

    #[test]
    fn every_canonical_datom_line_actualizes_into_a_contract_head() {
        let canonical = include_str!("../examples/canonical.datom");
        let mut lines = 0;
        for line in canonical.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with(';') {
                continue;
            }
            lines += 1;
            let actualized = Potential::<Query>::from(line.to_owned())
                .actualize(&mut budget())
                .is_ok()
                || Potential::<Response>::from(line.to_owned())
                    .actualize(&mut budget())
                    .is_ok();
            assert!(actualized, "canonical line is no contract head: {line}");
        }
        assert_eq!(
            lines, 16,
            "canonical file should carry sixteen contract heads"
        );
    }

    #[test]
    fn peer_text_beyond_the_extent_is_refused() {
        let mut exhausted = Budget {
            remaining: 4,
            reader: ReaderBudget { remaining: 4 },
            depth: 0,
            maximum_depth: PEER_TEXT_DEPTH,
        };
        let text = textualize(Query::Grant(grant()));
        assert!(
            Potential::<Query>::from(text)
                .actualize(&mut exhausted)
                .is_err()
        );
    }
}
