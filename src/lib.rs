//! Meta policy Signal contract for the PersonaRouter channel-authority
//! surface.
//!
//! Wire-only: the rkyv + NOTA codec and the signal-frame mail envelope for
//! the meta channel-policy operations — granting, extending, revoking a
//! router channel, and denying an adjudication request
//! (Grant/Extend/Revoke/Deny). The peer-callable read/observe surface lives
//! in the ordinary `signal-router` contract; runtime actors, policy
//! evaluation, socket binding, and durable grant tables live in the
//! `router` daemon.
//!
//! The wire types, the `Input`/`Output` enums, the route witnesses, the
//! short headers, and the `encode_signal_frame`/`decode_signal_frame` mail
//! envelope are all emitted from `schema/lib.schema` by `schema-rust`;
//! the checked-in artifacts live in `src/schema/`.

#[allow(dead_code, private_interfaces)]
pub mod schema;

pub use schema::lib::*;

impl ChannelGrant {
    pub fn new(
        source: ChannelEndpoint,
        destination: ChannelEndpoint,
        kinds: Vec<ChannelMessageKind>,
        duration: ChannelDuration,
    ) -> Self {
        Self {
            source,
            destination,
            kinds: Kinds::new(kinds),
            duration: Duration::new(duration),
        }
    }

    pub fn kinds(&self) -> &[ChannelMessageKind] {
        self.kinds.payload().as_slice()
    }

    pub fn duration(&self) -> &ChannelDuration {
        self.duration.payload()
    }
}

impl ChannelExtension {
    pub fn new(channel: ChannelIdentifier, duration: ChannelDuration) -> Self {
        Self {
            channel: Channel::new(channel),
            duration: Duration::new(duration),
        }
    }

    pub fn channel(&self) -> &ChannelIdentifier {
        self.channel.payload()
    }

    pub fn duration(&self) -> &ChannelDuration {
        self.duration.payload()
    }
}

impl ChannelRevocation {
    pub fn new(channel: ChannelIdentifier, reason: TextBody) -> Self {
        Self {
            channel: Channel::new(channel),
            reason: Reason::new(reason),
        }
    }

    pub fn channel(&self) -> &ChannelIdentifier {
        self.channel.payload()
    }

    pub fn reason(&self) -> &TextBody {
        self.reason.payload()
    }
}

impl AdjudicationDenial {
    pub fn new(request: AdjudicationRequestIdentifier, reason: TextBody) -> Self {
        Self {
            denied_request: DeniedRequest::new(request),
            reason: Reason::new(reason),
        }
    }

    pub fn request(&self) -> &AdjudicationRequestIdentifier {
        self.denied_request.payload()
    }

    pub fn reason(&self) -> &TextBody {
        self.reason.payload()
    }
}

impl RejectedChannelOrder {
    pub fn new(operation: OperationKind, reason: ChannelOrderRejectionReason) -> Self {
        Self {
            operation: Operation::new(operation),
            order_rejection_reason: OrderRejectionReason::new(reason),
        }
    }

    pub fn operation(&self) -> &OperationKind {
        self.operation.payload()
    }

    pub fn reason(&self) -> &ChannelOrderRejectionReason {
        self.order_rejection_reason.payload()
    }
}

impl UnimplementedRequest {
    pub fn new(operation: OperationKind, reason: UnimplementedReason) -> Self {
        Self {
            operation: Operation::new(operation),
            implementation_reason: ImplementationReason::new(reason),
        }
    }

    pub fn operation(&self) -> &OperationKind {
        self.operation.payload()
    }

    pub fn reason(&self) -> &UnimplementedReason {
        self.implementation_reason.payload()
    }
}
