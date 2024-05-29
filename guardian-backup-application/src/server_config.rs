use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::sync::{Mutex, OnceLock};
use std::sync::atomic::{AtomicU16, Ordering};

pub struct ServerConfig {
    pub bind_to: SocketAddr,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            bind_to: "0.0.0.0:8998".parse().unwrap(),
        }
    }
}

#[cfg(any(test, feature = "mocks"))]
impl ServerConfig {
    pub fn test_config() -> Self {
        static PORT_COUNTER: AtomicU16 = AtomicU16::new(18998);

        Self {
            bind_to: SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127,0,0,1), PORT_COUNTER.fetch_add(1, Ordering::SeqCst))),
        }
    }
}