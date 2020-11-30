use crate::worker::Worker;
use mio::net::{TcpListener, TcpStream};
use mio::{Events, Token};
use std::collections::HashMap;
use std::io;
use std::io::Error;
use std::net::SocketAddr;

pub struct Accept {
    poll: mio::Poll,
    listeners: HashMap<Token, TcpListener>,
    wrks: [Worker],
}

impl Accept {
    pub fn run(&mut self) -> io::Result<()> {
        let mut events = Events::with_capacity(128);
        loop {
            self.poll.poll(&mut events, None)?;

            for event in events.iter() {
                let token = event.token();
                if let Some(lis) = self.listeners.get_mut(&token) {
                    match lis.accept() {
                        Ok(_) => {}
                        Err(_) => {}
                    }
                }
            }
        }
    }
}
