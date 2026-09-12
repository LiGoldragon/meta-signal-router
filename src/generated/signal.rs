#![allow(dead_code, non_camel_case_types, non_snake_case)]
#[rustfmt::skip]
pub type AdjudicationRequestIdentifier = String;
#[rustfmt::skip]
pub type SystemPrincipal = String;
#[rustfmt::skip]
pub type TextBody = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum OperationKind {
    Deny,
    Grant,
    Extend,
    Revoke,
}
#[rustfmt::skip]
pub type MirrorEnabled = bool;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct OtherPersonaEngine {
    pub engine_identifier: signal_router::EngineIdentifier,
    pub host_name: signal::HostName,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ConnectionClass {
    Network(signal::NetworkEndpoint),
    System(SystemPrincipal),
    NonOwnerUser(signal_router::UnixUserIdentifier),
    OtherPersona(OtherPersonaEngine),
    Owner,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ChannelEndpoint {
    External(ConnectionClass),
    Internal(signal::ComponentKind),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ChannelMessageKind {
    DeliveryNotification,
    MessageDelivery,
    AdjudicationRequest,
    PromptBufferObservation,
    MessageSubmission,
    TerminalResize,
    TerminalInput,
    MessageIngressSubmission,
    FocusObservation,
    TerminalCapture,
    InboxQuery,
    TranscriptEvent,
}
#[rustfmt::skip]
pub type ChannelMessageKinds = std::vec::Vec<ChannelMessageKind>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ChannelDuration {
    TimeBound(signal_router::TimestampNanos),
    OneShot,
    Permanent,
}
#[rustfmt::skip]
pub type GrantSource = ChannelEndpoint;
#[rustfmt::skip]
pub type GrantTarget = ChannelEndpoint;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ChannelGrant {
    pub grant_source: GrantSource,
    pub grant_target: GrantTarget,
    pub channel_message_kinds: ChannelMessageKinds,
    pub channel_duration: ChannelDuration,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ChannelExtension {
    pub channel_identifier: signal_router::ChannelIdentifier,
    pub channel_duration: ChannelDuration,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ChannelRevocation {
    pub channel_identifier: signal_router::ChannelIdentifier,
    pub text_body: TextBody,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct AdjudicationDenial {
    pub adjudication_request_identifier: AdjudicationRequestIdentifier,
    pub text_body: TextBody,
}
#[rustfmt::skip]
pub type GrantedChannel = signal_router::ChannelIdentifier;
#[rustfmt::skip]
pub type ExtendedChannel = signal_router::ChannelIdentifier;
#[rustfmt::skip]
pub type RevokedChannel = signal_router::ChannelIdentifier;
#[rustfmt::skip]
pub type DeniedAdjudication = AdjudicationRequestIdentifier;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ChannelOrderRejectionReason {
    MetaAuthorityRequired,
    ChannelMissing,
    AdjudicationRequestMissing,
    PolicyRefused,
    ChannelAlreadyExists,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct RejectedChannelOrder {
    pub operation_kind: OperationKind,
    pub channel_order_rejection_reason: ChannelOrderRejectionReason,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum UnimplementedReason {
    PolicyStoreUnavailable,
    NotBuiltYet,
    DependencyNotReady,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct UnimplementedRequest {
    pub operation_kind: OperationKind,
    pub unimplemented_reason: UnimplementedReason,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Query {
    Extend(ChannelExtension),
    Deny(AdjudicationDenial),
    SetMirrorEnabled(MirrorEnabled),
    Revoke(ChannelRevocation),
    Grant(ChannelGrant),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Response {
    ChannelOrderRejected(RejectedChannelOrder),
    ChannelGranted(GrantedChannel),
    AdjudicationDenied(DeniedAdjudication),
    MirrorEnabledSet(MirrorEnabled),
    ChannelRevoked(RevokedChannel),
    ChannelExtended(ExtendedChannel),
    RequestUnimplemented(UnimplementedRequest),
}
