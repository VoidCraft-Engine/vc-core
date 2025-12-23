use crate::derive::impl_reflect_opaque;

impl_reflect_opaque!(::core::net::SocketAddr(
    serde, mini, partial_eq, hash, debug
));
