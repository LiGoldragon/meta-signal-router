use meta_signal_router::*;

fn requests() -> Vec<(z2VVKk, &'static str)> {
    vec![
        (
            z2VVKk::z2VLcZ(z2VLSg {
                field_0: signal_router::schema::lib::z2VUhk::new("channel-aab".to_owned()),
                field_1: z2VWgk::z2VXGt(signal_router::schema::lib::z2VQGK::new(7)),
            }),
            "Extend",
        ),
        (
            z2VVKk::z2VKx6(z2VWSR {
                field_0: z2VSet::new("fixture".to_owned()),
                field_1: z2VXGr::new("fixture".to_owned()),
            }),
            "Deny",
        ),
        (z2VVKk::z2VYZY(z2VZs4::new(true)), "SetMirrorEnabled"),
        (
            z2VVKk::z2VTdF(z2VRn4 {
                field_0: signal_router::schema::lib::z2VUhk::new("channel-aab".to_owned()),
                field_1: z2VXGr::new("fixture".to_owned()),
            }),
            "Revoke",
        ),
        (
            z2VVKk::z2VQkd(z2VXEz {
                field_0: z2Vbxp::z2Va3A(z2VVrv::z2VRvM(signal_standard::schema::lib::z2VaVE {
                    field_0: signal_standard::schema::lib::z2VLyh::new("host-aab".to_owned()),
                    field_1: signal_standard::schema::lib::z2VQaE::new(7),
                })),
                field_1: z2Vbxp::z2Va3A(z2VVrv::z2VRvM(signal_standard::schema::lib::z2VaVE {
                    field_0: signal_standard::schema::lib::z2VLyh::new("host-aab".to_owned()),
                    field_1: signal_standard::schema::lib::z2VQaE::new(7),
                })),
                field_2: Vec::new(),
                field_3: z2VWgk::z2VXGt(signal_router::schema::lib::z2VQGK::new(7)),
            }),
            "Grant",
        ),
    ]
}

fn replies() -> Vec<(z2VZMR, &'static str)> {
    vec![
        (
            z2VZMR::z2VPNU(z2VbCv {
                field_0: z2VMzP::z2VVom,
                field_1: z2VL5G::z2VeAc,
            }),
            "ChannelOrderRejected",
        ),
        (
            z2VZMR::z2VRJ5(z2VUEJ::new(signal_router::schema::lib::z2VUhk::new(
                "channel-aab".to_owned(),
            ))),
            "ChannelGranted",
        ),
        (
            z2VZMR::z2VSbY(z2VMyn::new(z2VSet::new("fixture".to_owned()))),
            "AdjudicationDenied",
        ),
        (z2VZMR::z2VWPR(z2VZs4::new(true)), "MirrorEnabledSet"),
        (
            z2VZMR::z2VXtU(z2VShs::new(signal_router::schema::lib::z2VUhk::new(
                "channel-aab".to_owned(),
            ))),
            "ChannelRevoked",
        ),
        (
            z2VZMR::z2VZzt(z2VPBk::new(signal_router::schema::lib::z2VUhk::new(
                "channel-aab".to_owned(),
            ))),
            "ChannelExtended",
        ),
        (
            z2VZMR::z2VLEb(z2VSuA {
                field_0: z2VMzP::z2VVom,
                field_1: z2VXAD::z2VYnp,
            }),
            "RequestUnimplemented",
        ),
    ]
}

#[test]
fn every_request_round_trips_through_the_bound_frame() {
    for (request, _head) in requests() {
        let expected = request.clone();
        let exchange = signal_frame::ExchangeIdentifier::new(
            signal_frame::SessionEpoch::new(41),
            signal_frame::ExchangeLane::Connector,
            signal_frame::LaneSequence::first(),
        );
        let encoded = request
            .encode_request_frame(exchange)
            .expect("request frame encodes");
        let (decoded_exchange, decoded) =
            ContractMarker::decode_single_request(&encoded).expect("request frame decodes");
        assert_eq!(decoded_exchange, exchange);
        assert_eq!(decoded, expected);
    }
}

#[test]
fn every_reply_has_bound_frame_and_rkyv_behavior() {
    for (reply, _head) in replies() {
        let expected = reply.clone();
        let exchange = signal_frame::ExchangeIdentifier::new(
            signal_frame::SessionEpoch::new(43),
            signal_frame::ExchangeLane::Connector,
            signal_frame::LaneSequence::first(),
        );
        let encoded = reply
            .clone()
            .encode_reply_frame(exchange)
            .expect("reply frame encodes");
        ContractMarker::decode_frame(&encoded).expect("reply frame decodes");

        let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&reply).expect("reply archives");
        let recovered =
            rkyv::from_bytes::<z2VZMR, rkyv::rancor::Error>(&archive).expect("reply recovers");
        assert_eq!(recovered, expected);
    }
}

#[cfg(feature = "dotos-text")]
#[test]
fn every_root_round_trips_through_dotos_with_visible_heads() {
    use dotos::{DotosEncode, DotosSource};

    for (request, head) in requests() {
        let text = request.to_dotos();
        assert!(text.starts_with(&format!("{head}.")), "{text}");
        let recovered = DotosSource::new(&text)
            .parse::<z2VVKk>()
            .expect("request Dotos decodes");
        assert_eq!(recovered, request);
    }
    for (reply, head) in replies() {
        let text = reply.to_dotos();
        assert!(text.starts_with(&format!("{head}.")), "{text}");
        let recovered = DotosSource::new(&text)
            .parse::<z2VZMR>()
            .expect("reply Dotos decodes");
        assert_eq!(recovered, reply);
    }
}
