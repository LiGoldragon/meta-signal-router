//! Explicit producer-owned bootstrap authority state for the owner Router Interface.
//!
//! Every identity and canonical-order value below is an already-minted opaque
//! seat. None is derived from source spelling, position, or content.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AuthoritySeat {
    pub spelling: &'static str,
    pub local: u16,
    pub canonical: u64,
}

impl AuthoritySeat {
    pub const fn new(spelling: &'static str, local: u16, canonical: u64) -> Self {
        Self {
            spelling,
            local,
            canonical,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DeclarationSeat {
    pub owner_local: Option<u16>,
    pub spelling: &'static str,
    pub local: u16,
    pub canonical: u64,
}

impl DeclarationSeat {
    pub const fn new(
        owner_local: Option<u16>,
        spelling: &'static str,
        local: u16,
        canonical: u64,
    ) -> Self {
        Self {
            owner_local,
            spelling,
            local,
            canonical,
        }
    }
}

pub const AUTHORITY_IDENTITY: [u8; 32] = [
    150, 232, 213, 234, 78, 230, 172, 139, 85, 22, 239, 9, 251, 71, 126, 67, 246, 113, 123, 21, 70,
    148, 23, 45, 129, 253, 116, 212, 89, 43, 60, 114,
];
pub const AUTHORITY_REVISION: u64 = 1;
pub const GRAMMAR_DOCUMENT_LOCAL: u16 = 26889;
pub const GRAMMAR_SYNTAX_LOCAL: u16 = 22149;

pub const INTERFACE_SEAT: AuthoritySeat =
    AuthoritySeat::new("Interface", 21467, 0xc4b5f64f7a0f0d5c);
pub const NEXUS_SEAT: AuthoritySeat = AuthoritySeat::new("Nexus", 52067, 0x229900805bf4b4ce);
pub const SEMA_SEAT: AuthoritySeat = AuthoritySeat::new("Sema", 53265, 0x9b246638ed5b1a97);
pub const INPUT_SEAT: AuthoritySeat = AuthoritySeat::new("Input", 18160, 0x7dc3be95010b167c);
pub const OUTPUT_SEAT: AuthoritySeat = AuthoritySeat::new("Output", 39875, 0xf2e6eb136915d14d);
pub const REFUSAL_SEAT: AuthoritySeat = AuthoritySeat::new("Refusal", 54253, 0x97b3951d0ae17fb6);
pub const STRING_SEAT: AuthoritySeat = AuthoritySeat::new("String", 39001, 0x32d429547e0877b5);
pub const INTEGER_SEAT: AuthoritySeat = AuthoritySeat::new("Integer", 39351, 0x3999e58c298fbbe4);
pub const BOOLEAN_SEAT: AuthoritySeat = AuthoritySeat::new("Boolean", 48013, 0x59fe2ad3ee29dd2f);
pub const UNIT_SEAT: AuthoritySeat = AuthoritySeat::new("Unit", 11060, 0xbf2d18ec99f17594);
pub const VECTOR_SEAT: AuthoritySeat = AuthoritySeat::new("Vector", 61762, 0xcc0d6825fae11adf);
pub const OPTION_SEAT: AuthoritySeat = AuthoritySeat::new("Option", 11046, 0x59e026a5a2c671d5);
pub const MAP_SEAT: AuthoritySeat = AuthoritySeat::new("Map", 41018, 0xa2a617d00ae9bf99);
pub const RESULT_SEAT: AuthoritySeat = AuthoritySeat::new("Result", 20688, 0x11cb8f64a6ae91ab);
pub const STREAM_SEAT: AuthoritySeat = AuthoritySeat::new("Stream", 38623, 0x0d1a802e66430767);
pub const STREAMIDENTITY_SEAT: AuthoritySeat =
    AuthoritySeat::new("StreamIdentity", 25167, 0x93997976d9658799);

pub const RUST_VOCABULARY_LOCALS: [u16; 10] = [
    14987, 65388, 22856, 35422, 21212, 55582, 62225, 12158, 10633, 37507,
];

pub const DECLARATION_SEATS: &[DeclarationSeat] = &[
    DeclarationSeat::new(
        None,
        "AdjudicationRequestIdentifier",
        23177,
        0x2d047b61c6cfc2c7,
    ),
    DeclarationSeat::new(None, "TextBody", 38719, 0x7b77cbea246e9a24),
    DeclarationSeat::new(None, "MirrorEnabled", 47431, 0xc9cd1309bd7bcd65),
    DeclarationSeat::new(None, "SystemPrincipal", 43415, 0x2dc46d03d5cd4e5a),
    DeclarationSeat::new(None, "OtherPersonaEngine", 49709, 0xdd651f3ab37d3001),
    DeclarationSeat::new(None, "ConnectionClass", 33967, 0xd2330edcf0fef071),
    DeclarationSeat::new(Some(33967), "Owner", 18015, 0xeee13dcabc58d1cf),
    DeclarationSeat::new(Some(33967), "NonOwnerUser", 23482, 0x86713e177c5f0d2b),
    DeclarationSeat::new(Some(33967), "System", 46073, 0x1d2b919dbe0074a7),
    DeclarationSeat::new(Some(33967), "OtherPersona", 28591, 0xc6b08d5492a8896d),
    DeclarationSeat::new(Some(33967), "Network", 20710, 0x039e81d9668a1a0d),
    DeclarationSeat::new(None, "ChannelEndpoint", 54493, 0x78fe38606cf5f2d6),
    DeclarationSeat::new(Some(54493), "Internal", 35824, 0x8eb4f290b702554a),
    DeclarationSeat::new(Some(54493), "External", 48017, 0x5ee1cdbbe4cdfb44),
    DeclarationSeat::new(None, "ChannelMessageKind", 29403, 0x9cf3b80179a7e027),
    DeclarationSeat::new(
        Some(29403),
        "MessageIngressSubmission",
        191,
        0x9ea0d2b2500714bd,
    ),
    DeclarationSeat::new(Some(29403), "MessageSubmission", 17203, 0x5530d5d0fe93b776),
    DeclarationSeat::new(Some(29403), "InboxQuery", 1288, 0xf65bbbdb4346ccb8),
    DeclarationSeat::new(Some(29403), "FocusObservation", 12979, 0xe3bc519fd255cef0),
    DeclarationSeat::new(
        Some(29403),
        "PromptBufferObservation",
        30205,
        0x50d3427ed92b3cf0,
    ),
    DeclarationSeat::new(Some(29403), "MessageDelivery", 14639, 0x339e289caf7f9662),
    DeclarationSeat::new(Some(29403), "TerminalInput", 28964, 0x8a0b72df31acb21e),
    DeclarationSeat::new(Some(29403), "TerminalCapture", 57054, 0xe708d915396af320),
    DeclarationSeat::new(Some(29403), "TerminalResize", 5117, 0x8659ebfd734b4779),
    DeclarationSeat::new(Some(29403), "TranscriptEvent", 44034, 0xf74ba7cac84d57b9),
    DeclarationSeat::new(
        Some(29403),
        "AdjudicationRequest",
        17029,
        0x43af7b976995eb16,
    ),
    DeclarationSeat::new(
        Some(29403),
        "DeliveryNotification",
        3089,
        0x1006ef3de31da42d,
    ),
    DeclarationSeat::new(None, "ChannelDuration", 36741, 0xa33e240c08fc2e80),
    DeclarationSeat::new(Some(36741), "OneShot", 52102, 0xa39a2f8f7d1d2bc9),
    DeclarationSeat::new(Some(36741), "Permanent", 32672, 0xbf1773c90b82ee32),
    DeclarationSeat::new(Some(36741), "TimeBound", 38721, 0x87bb01f2d7fdf554),
    DeclarationSeat::new(None, "ChannelGrant", 38611, 0x38c7fd145bd42af6),
    DeclarationSeat::new(None, "ChannelExtension", 2285, 0x47c83b9b8afe4e53),
    DeclarationSeat::new(None, "ChannelRevocation", 20229, 0x338cab9db52e4aed),
    DeclarationSeat::new(None, "AdjudicationDenial", 35910, 0xf68b2552ce8c4315),
    DeclarationSeat::new(None, "GrantedChannel", 28479, 0x9979e0f794191a12),
    DeclarationSeat::new(None, "ExtendedChannel", 11511, 0x52b836a7b0cb541d),
    DeclarationSeat::new(None, "RevokedChannel", 23350, 0x153bd71cb9877173),
    DeclarationSeat::new(None, "DeniedAdjudication", 7453, 0xad95b5392c158941),
    DeclarationSeat::new(None, "OperationKind", 7488, 0xc3f7187e2dd9dd27),
    DeclarationSeat::new(Some(7488), "Grant", 41501, 0xb733ccfca06dc2a6),
    DeclarationSeat::new(Some(7488), "Extend", 63980, 0xc56c3ec91832d0a6),
    DeclarationSeat::new(Some(7488), "Revoke", 41814, 0xd7c23a8c27e19751),
    DeclarationSeat::new(Some(7488), "Deny", 33784, 0xae9ef018a20b675e),
    DeclarationSeat::new(
        None,
        "ChannelOrderRejectionReason",
        1043,
        0x067c71aa4abd3691,
    ),
    DeclarationSeat::new(
        Some(1043),
        "MetaAuthorityRequired",
        61905,
        0x125a7d66fb19725f,
    ),
    DeclarationSeat::new(
        Some(1043),
        "ChannelAlreadyExists",
        34483,
        0xc216ff891554a50c,
    ),
    DeclarationSeat::new(Some(1043), "ChannelMissing", 43413, 0x12acd9c994f34bb6),
    DeclarationSeat::new(
        Some(1043),
        "AdjudicationRequestMissing",
        24851,
        0x365ac7d93e917e05,
    ),
    DeclarationSeat::new(Some(1043), "PolicyRefused", 39276, 0xab0bc98353ef4a36),
    DeclarationSeat::new(None, "RejectedChannelOrder", 51947, 0xc2b744e74f94958b),
    DeclarationSeat::new(None, "UnimplementedReason", 38334, 0x0c988c6316f142a5),
    DeclarationSeat::new(Some(38334), "NotBuiltYet", 2589, 0xf892b293bbdcc043),
    DeclarationSeat::new(Some(38334), "DependencyNotReady", 17732, 0xfc48f2856d4d255f),
    DeclarationSeat::new(
        Some(38334),
        "PolicyStoreUnavailable",
        43821,
        0xe02f5b296f59bf73,
    ),
    DeclarationSeat::new(None, "UnimplementedRequest", 24005, 0x10ba3d3e01912b2f),
    DeclarationSeat::new(None, "MetaRouterRequest", 32159, 0xeab15aecacb34f1e),
    DeclarationSeat::new(Some(32159), "Grant", 16782, 0xc5246f63063ba8ff),
    DeclarationSeat::new(Some(32159), "Extend", 2858, 0x18a3b055417d9c88),
    DeclarationSeat::new(Some(32159), "Revoke", 26446, 0xb5250505bc46cf39),
    DeclarationSeat::new(Some(32159), "Deny", 627, 0x7d84d55cc9e2214e),
    DeclarationSeat::new(Some(32159), "SetMirrorEnabled", 43051, 0xa65052f27e68e6b2),
    DeclarationSeat::new(None, "MetaRouterReply", 45712, 0xe7909e672f964a14),
    DeclarationSeat::new(Some(45712), "ChannelGranted", 18606, 0x6bc86122d321dcde),
    DeclarationSeat::new(Some(45712), "ChannelExtended", 47885, 0x9a1f67206d0d8d7e),
    DeclarationSeat::new(Some(45712), "ChannelRevoked", 40785, 0x878a6c1dcee9f032),
    DeclarationSeat::new(Some(45712), "AdjudicationDenied", 22983, 0x75a0da8d01508149),
    DeclarationSeat::new(
        Some(45712),
        "ChannelOrderRejected",
        12133,
        0x6852fa964bba3b4f,
    ),
    DeclarationSeat::new(
        Some(45712),
        "RequestUnimplemented",
        1584,
        0xf812249168d18e52,
    ),
    DeclarationSeat::new(Some(45712), "MirrorEnabledSet", 35736, 0x78cc00f8727407aa),
];
