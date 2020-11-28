use std::time::Duration;
use std::net::SocketAddr;
use std::fs::File;

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

