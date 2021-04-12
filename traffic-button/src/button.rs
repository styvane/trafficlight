//! This module contains a pedestrian traffic light push button.
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use std::io::ErrorKind;
use std::net::UdpSocket;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    rename_all = "kebab-case",
    name = "button",
    about = "A traffic light pedestrian's push button."
)]
pub struct ButtonOption {
    #[structopt(short, long)]
    port: usize,

    #[structopt(short, long)]
    hostname: String,

    #[structopt(short, long, default_value = "12000")]
    local_port: usize,
}

/// The type `Button` represents a pedestrian push button for traffic light.
pub struct Button {
    opt: ButtonOption,
    sock: UdpSocket,
}

impl Button {
    /// Create a new push button.
    pub fn new(opt: ButtonOption) -> Self {
        let sock = UdpSocket::bind(format!("127.0.0.1:{}", opt.local_port)).unwrap();
        Button { opt, sock }
    }

    fn handle_key_event(&mut self, message: &[u8], addr: &str) {
        if let Err(error) = self.sock.send_to(message, addr) {
            match error.kind() {
                ErrorKind::NotConnected => eprintln!("Error: not connected"),
                ErrorKind::UnexpectedEof => (),
                _ => return,
            }
        }
    }

    /// Push a button.
    pub fn push(&mut self) {
        loop {
            println!("Button: [Press Return]");
            match event::read().unwrap() {
                Event::Key(KeyEvent { code, .. }) if code == KeyCode::Enter => {
                    let addr = format!("{}:{}", self.opt.hostname, self.opt.port);
                    let message = format!("press button ({})", addr);
                    let message = message.as_bytes();
                    println!("pressed, {}", addr);
                    self.handle_key_event(message, &addr);
                }
                _ => return,
            }
        }
    }
}
