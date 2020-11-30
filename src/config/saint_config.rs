use std::fs::File;
use std::net::SocketAddr;
use std::time::Duration;

pub struct Config {
    pub parallelism: u32,

    pub connection: u32,

    pub thread: u32,

    pub request_count: Option<u64>,

    pub interval: u32,

    pub duration: Option<Duration>,

    pub host: SocketAddr,

    pub command_file: File,
}
