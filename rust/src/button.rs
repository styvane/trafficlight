//! This module contains a pedestrian traffic light push button.

use std::io::{Error, ErrorKind};
use std::net::UdpSocket;

use clap::Parser;
use crossterm::event::{self, Event, KeyCode, KeyEvent};

#[derive(Debug, Parser)]
/// A traffic light pedestrian's push button.
///
/// When you press return/enter on the keyboard, it
/// sends a UDP packet to the host/port specified.
///
/// The specified host/port must an up and running UDP server.
#[command(rename_all = "kebab-case", name = "button")]
pub struct ButtonArgs {
    #[arg(short, long, default_value = "23000")]
    port: usize,

    #[arg(short, long, default_value = "127.0.0.1")]
    bind_ip: String,
}

/// The type `Button` represents a pedestrian push button for traffic light.
pub struct Button {
    opt: ButtonArgs,
    sock: UdpSocket,
}

impl Button {
    /// Create a new push button.
    /// It also create a UDP socket and bind it to an ephemeral port.
    pub fn new(opt: ButtonArgs) -> Self {
        let sock = UdpSocket::bind("0.0.0.0:0").unwrap();
        Button { opt, sock }
    }

    fn handle_key_event(&mut self, message: &[u8], addr: &str) -> Result<(), Error> {
        if let Err(error) = self.sock.send_to(message, addr) {
            match error.kind() {
                ErrorKind::NotConnected => eprintln!("Error: not connected"),
                ErrorKind::UnexpectedEof => return Err(error),
                _ => return Ok(()),
            }
        } else {
            println!("pressed button");
        }
        Ok(())
    }

    /// Push a button.
    pub fn push(&mut self) -> Result<(), Error> {
        println!("Button [Press Return]");
        match event::read().unwrap() {
            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                ..
            }) => {
                let addr = format!("{}:{}", self.opt.bind_ip, self.opt.port);
                let message = format!("press button ({})", addr);
                let message = message.as_bytes();
                self.handle_key_event(message, &addr)
            }
            _ => Ok(()),
        }
    }
}
