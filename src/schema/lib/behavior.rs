// Handwritten operational behavior for the authority-verified owner Router Interface.
//
// The strict bootstrap projection owns every structural type below. This file
// owns only behavior the current bootstrap language cannot yet express:
// structural runtime traits, the ordinary Input/Output role seating, and the
// allocated Signal frame boundary.

use rkyv::{
    Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize,
    rancor::Source as _,
};

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
#[rkyv(serialize_bounds(
    __S: rkyv::ser::Writer + rkyv::ser::Allocator,
    __S::Error: rkyv::rancor::Source,
))]
#[rkyv(deserialize_bounds(__D::Error: rkyv::rancor::Source))]
#[rkyv(bytecheck(bounds(__C: rkyv::validation::ArchiveContext)))]
#[doc(hidden)]
pub enum WireValue {
    Text(std::string::String), Integer(u64), Boolean(bool),
    Sequence(#[rkyv(omit_bounds)] Vec<WireValue>),
    Absent, Present(#[rkyv(omit_bounds)] Box<WireValue>),
    Product(#[rkyv(omit_bounds)] Vec<WireValue>),
    Variant { ordinal: u16, #[rkyv(omit_bounds)] fields: Vec<WireValue> },
}
#[derive(Debug, thiserror::Error)]
#[error("structural wire value does not match the authority-verified Interface")]
#[doc(hidden)]
pub struct WireShapeError;

/// Current-stage structural behavior shared by Interfaces that import these
/// producer-owned types.
#[doc(hidden)]
pub trait WireShape: Sized {
    fn to_wire(&self) -> WireValue;
    fn from_wire(value: WireValue) -> Result<Self, WireShapeError>;
}

fn router_wire_into_local(value: signal_router::WireValue) -> WireValue {
    match value {
        signal_router::WireValue::Text(value) => WireValue::Text(value),
        signal_router::WireValue::Integer(value) => WireValue::Integer(value),
        signal_router::WireValue::Boolean(value) => WireValue::Boolean(value),
        signal_router::WireValue::Sequence(values) => WireValue::Sequence(
            values.into_iter().map(router_wire_into_local).collect(),
        ),
        signal_router::WireValue::Absent => WireValue::Absent,
        signal_router::WireValue::Present(value) => {
            WireValue::Present(Box::new(router_wire_into_local(*value)))
        }
        signal_router::WireValue::Product(values) => WireValue::Product(
            values.into_iter().map(router_wire_into_local).collect(),
        ),
        signal_router::WireValue::Variant { ordinal, fields } => WireValue::Variant {
            ordinal,
            fields: fields.into_iter().map(router_wire_into_local).collect(),
        },
    }
}

fn local_wire_into_router(value: WireValue) -> signal_router::WireValue {
    match value {
        WireValue::Text(value) => signal_router::WireValue::Text(value),
        WireValue::Integer(value) => signal_router::WireValue::Integer(value),
        WireValue::Boolean(value) => signal_router::WireValue::Boolean(value),
        WireValue::Sequence(values) => signal_router::WireValue::Sequence(
            values.into_iter().map(local_wire_into_router).collect(),
        ),
        WireValue::Absent => signal_router::WireValue::Absent,
        WireValue::Present(value) => {
            signal_router::WireValue::Present(Box::new(local_wire_into_router(*value)))
        }
        WireValue::Product(values) => signal_router::WireValue::Product(
            values.into_iter().map(local_wire_into_router).collect(),
        ),
        WireValue::Variant { ordinal, fields } => signal_router::WireValue::Variant {
            ordinal,
            fields: fields.into_iter().map(local_wire_into_router).collect(),
        },
    }
}

fn standard_wire_into_local(value: signal_standard::WireValue) -> WireValue {
    match value {
        signal_standard::WireValue::Text(value) => WireValue::Text(value),
        signal_standard::WireValue::Integer(value) => WireValue::Integer(value),
        signal_standard::WireValue::Boolean(value) => WireValue::Boolean(value),
        signal_standard::WireValue::Sequence(values) => WireValue::Sequence(
            values.into_iter().map(standard_wire_into_local).collect(),
        ),
        signal_standard::WireValue::Absent => WireValue::Absent,
        signal_standard::WireValue::Present(value) => {
            WireValue::Present(Box::new(standard_wire_into_local(*value)))
        }
        signal_standard::WireValue::Product(values) => WireValue::Product(
            values.into_iter().map(standard_wire_into_local).collect(),
        ),
        signal_standard::WireValue::Variant { ordinal, fields } => WireValue::Variant {
            ordinal,
            fields: fields.into_iter().map(standard_wire_into_local).collect(),
        },
    }
}

fn local_wire_into_standard(value: WireValue) -> signal_standard::WireValue {
    match value {
        WireValue::Text(value) => signal_standard::WireValue::Text(value),
        WireValue::Integer(value) => signal_standard::WireValue::Integer(value),
        WireValue::Boolean(value) => signal_standard::WireValue::Boolean(value),
        WireValue::Sequence(values) => signal_standard::WireValue::Sequence(
            values.into_iter().map(local_wire_into_standard).collect(),
        ),
        WireValue::Absent => signal_standard::WireValue::Absent,
        WireValue::Present(value) => {
            signal_standard::WireValue::Present(Box::new(local_wire_into_standard(*value)))
        }
        WireValue::Product(values) => signal_standard::WireValue::Product(
            values.into_iter().map(local_wire_into_standard).collect(),
        ),
        WireValue::Variant { ordinal, fields } => signal_standard::WireValue::Variant {
            ordinal,
            fields: fields.into_iter().map(local_wire_into_standard).collect(),
        },
    }
}

macro_rules! bridge_router_wire_shape {
    ($($type:ty),+ $(,)?) => {
        $(impl WireShape for $type {
            fn to_wire(&self) -> WireValue {
                router_wire_into_local(signal_router::WireShape::to_wire(self))
            }

            fn from_wire(value: WireValue) -> Result<Self, WireShapeError> {
                signal_router::WireShape::from_wire(local_wire_into_router(value))
                    .map_err(|_| WireShapeError)
            }
        })+
    };
}

macro_rules! bridge_standard_wire_shape {
    ($($type:ty),+ $(,)?) => {
        $(impl WireShape for $type {
            fn to_wire(&self) -> WireValue {
                standard_wire_into_local(signal_standard::WireShape::to_wire(self))
            }

            fn from_wire(value: WireValue) -> Result<Self, WireShapeError> {
                signal_standard::WireShape::from_wire(local_wire_into_standard(value))
                    .map_err(|_| WireShapeError)
            }
        })+
    };
}

bridge_router_wire_shape!(
    signal_router::schema::lib::z2VUhk,
    signal_router::schema::lib::z2VbUg,
    signal_router::schema::lib::z2VQGK,
    signal_router::schema::lib::z2Vf91,
);

bridge_standard_wire_shape!(
    signal_standard::schema::lib::z2VWWD,
    signal_standard::schema::lib::z2VLyh,
    signal_standard::schema::lib::z2VaVE,
);

impl WireShape for std::string::String {
    fn to_wire(&self) -> WireValue { WireValue::Text(self.clone()) }
    fn from_wire(value: WireValue) -> Result<Self, WireShapeError> { match value { WireValue::Text(value) => Ok(value), _ => Err(WireShapeError) } }
}
impl WireShape for u64 {
    fn to_wire(&self) -> WireValue { WireValue::Integer(*self) }
    fn from_wire(value: WireValue) -> Result<Self, WireShapeError> { match value { WireValue::Integer(value) => Ok(value), _ => Err(WireShapeError) } }
}
impl WireShape for bool {
    fn to_wire(&self) -> WireValue { WireValue::Boolean(*self) }
    fn from_wire(value: WireValue) -> Result<Self, WireShapeError> { match value { WireValue::Boolean(value) => Ok(value), _ => Err(WireShapeError) } }
}
impl<Value: WireShape> WireShape for Vec<Value> {
    fn to_wire(&self) -> WireValue { WireValue::Sequence(self.iter().map(WireShape::to_wire).collect()) }
    fn from_wire(value: WireValue) -> Result<Self, WireShapeError> {
        let WireValue::Sequence(values) = value else { return Err(WireShapeError) };
        values.into_iter().map(Value::from_wire).collect()
    }
}
impl<Value: WireShape> WireShape for Option<Value> {
    fn to_wire(&self) -> WireValue { match self { Some(value) => WireValue::Present(Box::new(value.to_wire())), None => WireValue::Absent } }
    fn from_wire(value: WireValue) -> Result<Self, WireShapeError> {
        match value { WireValue::Present(value) => Ok(Some(Value::from_wire(*value)?)), WireValue::Absent => Ok(None), _ => Err(WireShapeError) }
    }
}
fn one_field(mut fields: Vec<WireValue>) -> Result<WireValue, WireShapeError> {
    if fields.len() != 1 { return Err(WireShapeError); }
    Ok(fields.pop().expect("one field checked"))
}

macro_rules! wire_traits {
    ($name:ident) => {
        impl Clone for $name { fn clone(&self) -> Self { Self::from_wire(self.to_wire()).expect("a projected value revalidates") } }
        impl std::fmt::Debug for $name { fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { self.to_wire().fmt(formatter) } }
        impl PartialEq for $name { fn eq(&self, other: &Self) -> bool { self.to_wire() == other.to_wire() } }
        impl Eq for $name {}
    };
}
macro_rules! wire_external_newtype {
    ($name:ident, $inner:ty) => {
        impl WireShape for $name {
            fn to_wire(&self) -> WireValue { self.payload().to_wire() }
            fn from_wire(value: WireValue) -> Result<Self, WireShapeError> { Ok(Self::new(<$inner as WireShape>::from_wire(value)?)) }
        }
        wire_traits!($name);
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosEncode for $name {
            fn to_dotos(&self) -> std::string::String {
                dotos::DotosEncode::to_dotos(self.payload())
            }
        }
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosDecode for $name {
            fn from_dotos_block(block: &dotos::Block) -> Result<Self, dotos::DotosDecodeError> {
                <$inner as dotos::DotosDecode>::from_dotos_block(block).map(Self::new)
            }
        }
    };
}
macro_rules! wire_newtype {
    ($name:ident, $inner:ty) => {
        impl $name {
            pub fn new(payload: $inner) -> Self { Self(payload) }
            pub fn payload(&self) -> &$inner { &self.0 }
            pub fn into_payload(self) -> $inner { self.0 }
        }
        impl WireShape for $name {
            fn to_wire(&self) -> WireValue { self.0.to_wire() }
            fn from_wire(value: WireValue) -> Result<Self, WireShapeError> { Ok(Self(<$inner as WireShape>::from_wire(value)?)) }
        }
        wire_traits!($name);
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosEncode for $name {
            fn to_dotos(&self) -> std::string::String {
                dotos::DotosEncode::to_dotos(&self.0)
            }
        }
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosDecode for $name {
            fn from_dotos_block(block: &dotos::Block) -> Result<Self, dotos::DotosDecodeError> {
                <$inner as dotos::DotosDecode>::from_dotos_block(block).map(Self)
            }
        }
    };
}
macro_rules! wire_struct {
    ($name:ident { $($field:ident: $field_type:ty),* $(,)? }) => {
        impl WireShape for $name {
            fn to_wire(&self) -> WireValue { WireValue::Product(vec![$(self.$field.to_wire()),*]) }
            fn from_wire(value: WireValue) -> Result<Self, WireShapeError> {
                let WireValue::Product(fields) = value else { return Err(WireShapeError) };
                let mut fields = fields.into_iter();
                let result = Self { $($field: <$field_type as WireShape>::from_wire(fields.next().ok_or(WireShapeError)?)?),* };
                if fields.next().is_some() { return Err(WireShapeError); }
                Ok(result)
            }
        }
        wire_traits!($name);
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosEncode for $name {
            fn to_dotos(&self) -> std::string::String {
                dotos::Delimiter::Parenthesis.wrap([
                    $(dotos::DotosEncode::to_dotos(&self.$field)),*
                ])
            }
        }
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosDecode for $name {
            fn from_dotos_block(block: &dotos::Block) -> Result<Self, dotos::DotosDecodeError> {
                let body = dotos::DotosBody::from_delimited(
                    block,
                    dotos::Delimiter::Parenthesis,
                    stringify!($name),
                )?;
                let expected = 0usize $(+ {
                    let _ = stringify!($field);
                    1usize
                })*;
                #[allow(unused_mut, unused_variables)]
                let mut fields = body.expect_fields(stringify!($name), expected)?.iter();
                Ok(Self {
                    $($field: <$field_type as dotos::DotosDecode>::from_dotos_block(
                        fields.next().expect("field count checked"),
                    )?),*
                })
            }
        }
    };
}
macro_rules! wire_enum {
    ($name:ident {
        unit { $($unit_ordinal:literal => $unit:ident : $unit_visible:literal),* $(,)? }
        unary { $($unary_ordinal:literal => $unary:ident($payload:ty) : $unary_visible:literal),* $(,)? }
    }) => {
        impl WireShape for $name {
            fn to_wire(&self) -> WireValue {
                match self {
                    $(Self::$unit => WireValue::Variant { ordinal: $unit_ordinal, fields: Vec::new() },)*
                    $(Self::$unary(payload) => WireValue::Variant { ordinal: $unary_ordinal, fields: vec![payload.to_wire()] },)*
                }
            }
            fn from_wire(value: WireValue) -> Result<Self, WireShapeError> {
                let WireValue::Variant { ordinal, fields } = value else { return Err(WireShapeError) };
                match ordinal {
                    $($unit_ordinal if fields.is_empty() => Ok(Self::$unit),)*
                    $($unary_ordinal => Ok(Self::$unary(<$payload as WireShape>::from_wire(one_field(fields)?)?)),)*
                    _ => Err(WireShapeError),
                }
            }
        }
        wire_traits!($name);
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosEncode for $name {
            fn to_dotos(&self) -> std::string::String {
                match self {
                    $(Self::$unit => $unit_visible.to_owned(),)*
                    $(Self::$unary(payload) => format!(
                        "{}.{}",
                        $unary_visible,
                        dotos::DotosEncode::to_dotos(payload),
                    ),)*
                }
            }
        }
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosDecode for $name {
            fn from_dotos_block(block: &dotos::Block) -> Result<Self, dotos::DotosDecodeError> {
                if let Some(variant) = block.demote_to_string() {
                    return match variant {
                        $($unit_visible => Ok(Self::$unit),)*
                        _ => Err(dotos::DotosDecodeError::UnknownVariant {
                            enum_name: stringify!($name),
                            variant: variant.to_owned(),
                        }),
                    };
                }
                let (head, payload) = block.as_application().ok_or(
                    dotos::DotosDecodeError::ExpectedAtom { type_name: stringify!($name) },
                )?;
                let _ = &payload;
                let variant = head.demote_to_string().ok_or(
                    dotos::DotosDecodeError::ExpectedAtom { type_name: stringify!($name) },
                )?;
                match variant {
                    $($unary_visible => Ok(Self::$unary(
                        <$payload as dotos::DotosDecode>::from_dotos_block(payload)?,
                    )),)*
                    _ => Err(dotos::DotosDecodeError::UnknownVariant {
                        enum_name: stringify!($name),
                        variant: variant.to_owned(),
                    }),
                }
            }
        }
    };
}
wire_enum!(z2VL5G { unit { 0 => z2VeAc : "MetaAuthorityRequired", 1 => z2VYfn : "ChannelMissing", 2 => z2VT9k : "AdjudicationRequestMissing", 3 => z2VXST : "PolicyRefused", 4 => z2VW1p : "ChannelAlreadyExists" } unary {  } });
wire_enum!(z2VXAD { unit { 0 => z2VYnp : "PolicyStoreUnavailable", 1 => z2VLXv : "NotBuiltYet", 2 => z2VR31 : "DependencyNotReady" } unary {  } });
wire_struct!(z2VSuA { field_0: z2VMzP, field_1: z2VXAD });
wire_external_newtype!(z2VShs, signal_router::schema::lib::z2VUhk);
wire_external_newtype!(z2VSet, std::string::String);
wire_external_newtype!(z2VYfp, std::string::String);
wire_struct!(z2VRn4 { field_0: signal_router::schema::lib::z2VUhk, field_1: z2VXGr });
wire_struct!(z2VXEz { field_0: z2Vbxp, field_1: z2Vbxp, field_2: Vec< z2VUWE>, field_3: z2VWgk });
wire_struct!(z2VLSg { field_0: signal_router::schema::lib::z2VUhk, field_1: z2VWgk });
wire_external_newtype!(z2VPBk, signal_router::schema::lib::z2VUhk);
wire_enum!(z2Vbxp { unit {  } unary { 0 => z2Va3A(z2VVrv) : "External", 1 => z2VWQw(signal_standard::schema::lib::z2VWWD) : "Internal" } });
wire_external_newtype!(z2VXGr, std::string::String);
wire_external_newtype!(z2VUEJ, signal_router::schema::lib::z2VUhk);
wire_enum!(z2VUWE { unit { 0 => z2VLgY : "DeliveryNotification", 1 => z2VQ7g : "MessageDelivery", 2 => z2VQpt : "AdjudicationRequest", 3 => z2VUk4 : "PromptBufferObservation", 4 => z2VQst : "MessageSubmission", 5 => z2VMHW : "TerminalResize", 6 => z2VUNf : "TerminalInput", 7 => z2VKpa : "MessageIngressSubmission", 8 => z2VPd4 : "FocusObservation", 9 => z2Vciy : "TerminalCapture", 10 => z2VL9V : "InboxQuery", 11 => z2VYrV : "TranscriptEvent" } unary {  } });
wire_enum!(z2VWgk { unit { 1 => z2VbFb : "OneShot", 2 => z2VVUb : "Permanent" } unary { 0 => z2VXGt(signal_router::schema::lib::z2VQGK) : "TimeBound" } });
wire_newtype!(z2VMyn, z2VSet);
wire_struct!(z2VbCv { field_0: z2VMzP, field_1: z2VL5G });
wire_enum!(z2VMzP { unit { 0 => z2VVom : "Deny", 1 => z2VY6p : "Grant", 2 => z2VenP : "Extend", 3 => z2VYCD : "Revoke" } unary {  } });
wire_external_newtype!(z2VZs4, bool);
wire_enum!(z2VVrv { unit { 4 => z2VR7t : "Owner" } unary { 0 => z2VRvM(signal_standard::schema::lib::z2VaVE) : "Network", 1 => z2VZTe(z2VYfp) : "System", 2 => z2VSk9(signal_router::schema::lib::z2Vf91) : "NonOwnerUser", 3 => z2VUGE(z2VaYL) : "OtherPersona" } });
wire_struct!(z2VaYL { field_0: signal_router::schema::lib::z2VbUg, field_1: signal_standard::schema::lib::z2VLyh });
wire_enum!(z2VZMR { unit {  } unary { 0 => z2VPNU(z2VbCv) : "ChannelOrderRejected", 1 => z2VRJ5(z2VUEJ) : "ChannelGranted", 2 => z2VSbY(z2VMyn) : "AdjudicationDenied", 3 => z2VWPR(z2VZs4) : "MirrorEnabledSet", 4 => z2VXtU(z2VShs) : "ChannelRevoked", 5 => z2VZzt(z2VPBk) : "ChannelExtended", 6 => z2VLEb(z2VSuA) : "RequestUnimplemented" } });
wire_enum!(z2VVKk { unit {  } unary { 0 => z2VLcZ(z2VLSg) : "Extend", 1 => z2VKx6(z2VWSR) : "Deny", 2 => z2VYZY(z2VZs4) : "SetMirrorEnabled", 3 => z2VTdF(z2VRn4) : "Revoke", 4 => z2VQkd(z2VXEz) : "Grant" } });
wire_struct!(z2VWSR { field_0: z2VSet, field_1: z2VXGr });

macro_rules! archive_root {
    ($root:ident) => {
        impl Archive for $root {
            type Archived = <WireValue as Archive>::Archived;
            type Resolver = <WireValue as Archive>::Resolver;
            fn resolve(&self, resolver: Self::Resolver, out: rkyv::Place<Self::Archived>) {
                self.to_wire().resolve(resolver, out);
            }
        }
        impl<Serializer> RkyvSerialize<Serializer> for $root
        where
            Serializer: rkyv::rancor::Fallible + ?Sized,
            WireValue: RkyvSerialize<Serializer>,
        {
            fn serialize(
                &self,
                serializer: &mut Serializer,
            ) -> Result<Self::Resolver, Serializer::Error> {
                self.to_wire().serialize(serializer)
            }
        }
        impl<Deserializer> RkyvDeserialize<$root, Deserializer> for ArchivedWireValue
        where
            Deserializer: rkyv::rancor::Fallible + ?Sized,
            Deserializer::Error: rkyv::rancor::Source,
            ArchivedWireValue: RkyvDeserialize<WireValue, Deserializer>,
        {
            fn deserialize(
                &self,
                deserializer: &mut Deserializer,
            ) -> Result<$root, Deserializer::Error> {
                let wire = <ArchivedWireValue as RkyvDeserialize<
                    WireValue,
                    Deserializer,
                >>::deserialize(self, deserializer)?;
                <$root as WireShape>::from_wire(wire).map_err(Deserializer::Error::new)
            }
        }
    };
}
archive_root!(z2VL5G);
archive_root!(z2VXAD);
archive_root!(z2VSuA);
archive_root!(z2VShs);
archive_root!(z2VSet);
archive_root!(z2VYfp);
archive_root!(z2VRn4);
archive_root!(z2VXEz);
archive_root!(z2VLSg);
archive_root!(z2VPBk);
archive_root!(z2Vbxp);
archive_root!(z2VXGr);
archive_root!(z2VUEJ);
archive_root!(z2VUWE);
archive_root!(z2VWgk);
archive_root!(z2VMyn);
archive_root!(z2VbCv);
archive_root!(z2VMzP);
archive_root!(z2VZs4);
archive_root!(z2VVrv);
archive_root!(z2VaYL);
archive_root!(z2VZMR);
archive_root!(z2VVKk);
archive_root!(z2VWSR);


pub enum ContractMarker {}

impl signal_frame::WireContract for ContractMarker {
    const BINDING: signal_frame::ContractBinding = signal_frame::ContractBinding::new(
        match signal_frame::ContractId::try_new(8) {
            Ok(value) => value,
            Err(_) => panic!("contract ID is allocated"),
        },
        match signal_frame::WireRevision::try_new(2) {
            Ok(value) => value,
            Err(_) => panic!("wire revision is allocated"),
        },
    );
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum EngineRefusalReason {
    Rejected,
    Unavailable,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct EngineRefusal {
    pub reason: EngineRefusalReason,
    pub detail: std::string::String,
}

impl EngineRefusal {
    pub fn rejected(detail: std::string::String) -> Self {
        Self { reason: EngineRefusalReason::Rejected, detail }
    }

    pub fn unavailable(detail: std::string::String) -> Self {
        Self { reason: EngineRefusalReason::Unavailable, detail }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum SignalFrameError {
    #[error("failed to encode bound signal frame")]
    FrameEncode,
    #[error("failed to decode bound signal frame")]
    ArchiveDecode,
    #[error("unexpected signal frame body")]
    UnexpectedFrameBody,
    #[error("expected one request operation, found {found}")]
    OperationCount { found: usize },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum InputRoute {
    Extend,
    Deny,
    SetMirrorEnabled,
    Revoke,
    Grant,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OutputRoute {
    ChannelOrderRejected,
    ChannelGranted,
    AdjudicationDenied,
    MirrorEnabledSet,
    ChannelRevoked,
    ChannelExtended,
    RequestUnimplemented,
}

impl z2VVKk {
    pub fn route(&self) -> InputRoute {
        match self {
            Self::z2VLcZ(_) => InputRoute::Extend,
            Self::z2VKx6(_) => InputRoute::Deny,
            Self::z2VYZY(_) => InputRoute::SetMirrorEnabled,
            Self::z2VTdF(_) => InputRoute::Revoke,
            Self::z2VQkd(_) => InputRoute::Grant,
        }
    }

    pub fn wire_route(&self) -> signal_frame::WireRoute {
        signal_frame::WireRoute::new(
            signal_frame::RootCode::new(0),
            signal_frame::VariantCode::new(self.route() as u8),
        )
    }

    pub fn into_frame(self, exchange: signal_frame::ExchangeIdentifier) -> Frame {
        let route = self.wire_route();
        Frame::new(
            route,
            FrameBody::Request {
                exchange,
                request: signal_frame::Request::from_payload(self),
            },
        )
    }

    pub fn encode_request_frame(
        self,
        exchange: signal_frame::ExchangeIdentifier,
    ) -> Result<Vec<u8>, SignalFrameError> {
        self.into_frame(exchange)
            .encode()
            .map_err(|_| SignalFrameError::FrameEncode)
    }
}

impl z2VZMR {
    pub fn route(&self) -> OutputRoute {
        match self {
            Self::z2VPNU(_) => OutputRoute::ChannelOrderRejected,
            Self::z2VRJ5(_) => OutputRoute::ChannelGranted,
            Self::z2VSbY(_) => OutputRoute::AdjudicationDenied,
            Self::z2VWPR(_) => OutputRoute::MirrorEnabledSet,
            Self::z2VXtU(_) => OutputRoute::ChannelRevoked,
            Self::z2VZzt(_) => OutputRoute::ChannelExtended,
            Self::z2VLEb(_) => OutputRoute::RequestUnimplemented,
        }
    }

    pub fn wire_route(&self) -> signal_frame::WireRoute {
        signal_frame::WireRoute::new(
            signal_frame::RootCode::new(1),
            signal_frame::VariantCode::new(self.route() as u8),
        )
    }

    pub fn into_reply_frame(self, exchange: signal_frame::ExchangeIdentifier) -> Frame {
        let route = self.wire_route();
        let reply = signal_frame::Reply::committed(
            signal_frame::NonEmpty::single(signal_frame::SubReply::Ok(self)),
        );
        Frame::new(route, FrameBody::Reply { exchange, reply })
    }

    pub fn encode_reply_frame(
        self,
        exchange: signal_frame::ExchangeIdentifier,
    ) -> Result<Vec<u8>, SignalFrameError> {
        self.into_reply_frame(exchange)
            .encode()
            .map_err(|_| SignalFrameError::FrameEncode)
    }
}

impl signal_frame::RequestPayload for z2VVKk {}

impl signal_frame::SignalOperationHeads for z2VVKk {
    const HEADS: &'static [&'static str] = &["Extend", "Deny", "SetMirrorEnabled", "Revoke", "Grant"];
}

impl signal_frame::LogVariant for z2VVKk {
    fn log_variant(&self) -> u64 {
        let route = self.wire_route();
        u64::from(route.root().value()) | (u64::from(route.variant().value()) << 8)
    }
}

pub type Frame = signal_frame::BoundExchangeFrame<ContractMarker, z2VVKk, z2VZMR>;
pub type FrameBody = signal_frame::ExchangeFrameBody<z2VVKk, z2VZMR>;
pub type Request = signal_frame::Request<z2VVKk>;
pub type ReplyEnvelope = signal_frame::Reply<z2VZMR>;
pub type RequestBuilder = signal_frame::RequestBuilder<z2VVKk>;

impl ContractMarker {
    pub fn decode_frame(bytes: &[u8]) -> Result<Frame, SignalFrameError> {
        Frame::decode(bytes).map_err(|_| SignalFrameError::ArchiveDecode)
    }

    pub fn decode_single_request(
        bytes: &[u8],
    ) -> Result<(signal_frame::ExchangeIdentifier, z2VVKk), SignalFrameError> {
        match Self::decode_frame(bytes)?.into_body() {
            FrameBody::Request { exchange, request } => {
                let found = request.payloads().len();
                if found != 1 {
                    return Err(SignalFrameError::OperationCount { found });
                }
                Ok((exchange, request.payloads.into_head()))
            }
            _ => Err(SignalFrameError::UnexpectedFrameBody),
        }
    }
}
