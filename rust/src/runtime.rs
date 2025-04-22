//! Traffic light runtime
//!
//! This module implements the traffic light runtime.

use std::io;
use std::net::UdpSocket;
use std::sync::mpsc::{self, Sender};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use clap::Parser;

use crate::controller::{self, Event};

const NBYTES: usize = 1024;

/// The Direction type is the traffic light direction
#[derive(Debug, Eq, PartialEq)]
pub enum Direction {
    NorthSouth,
    EastWest,
}

/// The `Runtime` trait defines the runtime behavior.
pub trait Runtime {
    /// Set the light color for a direction based on a command.
    fn set_color(&mut self, direction: Direction, command: &str);

    /// Start runtime system.
    fn start(&mut self);
}

#[derive(Debug, Parser)]
#[command(name = "runtime", rename_all = "kebab-case")]
pub struct RuntimeArgs {
    /// North-South light socket address
    #[arg(short, long, default_value = "127.0.0.1:21000")]
    north_south_addr: String,

    /// East-West light socket address
    #[arg(short, long, default_value = "127.0.0.1:22000")]
    east_west_addr: String,

    /// Pedestrian button socket address
    #[arg(short, long, default_value = "127.0.0.1:23000")]
    button_addr: String,
}

/// The `LightRuntime` type is the runtime system for the traffic lights.
pub struct LightRuntime {
    options: RuntimeArgs,
    sock: Arc<UdpSocket>,
}

impl LightRuntime {
    /// Create a new light runtime.
    pub fn new(options: RuntimeArgs) -> io::Result<Self> {
        let sock = UdpSocket::bind(&options.button_addr)?;
        let sock = Arc::new(sock);
        Ok(LightRuntime { options, sock })
    }

    /// Emit clock event.
    fn emit_clock(sender: Sender<Event>) {
        loop {
            if let Err(e) = sender.send(Event::Clock) {
                eprintln!("{:#?}", e);
            }
            thread::sleep(Duration::from_secs(1));
        }
    }

    /// Watch button event.
    fn watch_button(sender: Sender<Event>, sock: Arc<UdpSocket>) {
        loop {
            let mut buf = [0; NBYTES];
            if let Err(e) = sock.recv(&mut buf) {
                eprintln!("{:#?}", e);
                continue;
            }

            sender.send(Event::Button).unwrap();
        }
    }

    fn north_south_addr(&self) -> &str {
        &self.options.north_south_addr
    }

    pub fn east_west_addr(&self) -> &str {
        &self.options.east_west_addr
    }
}

impl Runtime for LightRuntime {
    /// Set the light color for a direction by sending a command.
    /// The command is sent to a UDP socket.
    fn set_color(&mut self, direction: Direction, command: &str) {
        let addr = match direction {
            Direction::NorthSouth => self.north_south_addr(),
            Direction::EastWest => self.east_west_addr(),
        };

        let msg = command.as_bytes();
        self.sock.send_to(msg, addr).unwrap();
    }

    fn start(&mut self) {
        let mut controller = controller::Controller::default();
        let (sender, receiver) = mpsc::channel();

        let clock_sender = sender.clone();
        thread::spawn(move || {
            Self::emit_clock(clock_sender);
        });

        let sock = self.sock.clone();
        thread::spawn(move || {
            Self::watch_button(sender, sock);
        });

        for evt in receiver {
            println!("{}", controller);
            let (ns_light, ew_light) = (controller.ns_light.clone(), controller.ew_light.clone());
            controller.event_handler(evt);

            if ns_light != controller.ns_light {
                self.set_color(Direction::NorthSouth, &controller.ns_light.to_string());
            }

            if ew_light != controller.ew_light {
                self.set_color(Direction::EastWest, &controller.ew_light.to_string());
            }
        }
    }
}
