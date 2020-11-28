use std::collections::HashMap;
use mio::{Token, Events, Interest};
use mio::net::TcpStream;
use std::time::Duration;
use std::{mem, io};
use mio::event::Event;

pub struct Worker {
    poll: mio::Poll,
    marker: usize,
    statistic: Vec<u32>,
    time:u64,
    tasks: (),
}

const POLL_INTERVAL: u64 = 1;

impl Worker {
    pub fn new() {

    }

    pub fn next_token(&mut self) -> Token {
        let token = self.marker;
        self.marker += 1;
        return Token(token);
    }

    // pub fn register(&mut self, mut conn: TcpStream) {
    //     let token = self.next_token();
    //     self.poll.registry().register(&mut conn, token, Interest::READABLE | Interest::WRITABLE );
    //     self.connections.insert(token, conn);
    //
    //
    // }

    pub fn swap(&mut self, vec: &mut Vec<u32>) {
        mem::swap(&mut self.statistic, vec);
    }

    fn run(&mut self, connections: &mut HashMap<Token, TcpStream>) -> io::Result<()>{
        let mut events = Events::with_capacity(128);
        loop {
            self.poll.poll(&mut events, Some(Duration::from_micros(POLL_INTERVAL)))?;
            self.time += POLL_INTERVAL;

            for event in events.iter() {
                let token = event.token();
                if let Some(connection) = connections.get_mut(&token) {
                    self.handle_connection_event(event, connection);
                }

            }

        }
    }

    #[inline]
    fn handle_connection_event(&self, event: &Event, Connection: &mut TcpStream) {
        if event.is_readable() {

        }

        if event.is_error() {

        }
    }

    fn handle_tasks(&mut self) {

    }

}