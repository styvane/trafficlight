//! Traffic light runtime
//!
//! This module implements the traffic light runtime.

use crate::controller::{self, Event};
use std::io;
use std::net::UdpSocket;
use std::sync::mpsc::{self, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

const NBYTES: usize = 8192;

/// The Direction type is the traffic light direction
#[derive(Eq, PartialEq)]
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

/// The `LightRuntime` type is the runtime system for the traffic lights.
#[derive(Debug)]
pub struct LightRuntime<'a> {
    north_south_addr: &'a str,
    east_west_addr: &'a str,
    button_addr: &'a str,
    sock: Arc<Mutex<UdpSocket>>,
}

impl<'a> LightRuntime<'a> {
    /// Create a new light runtime.
    pub fn new(
        north_south_addr: &'a str,
        east_west_addr: &'a str,
        button_addr: &'a str,
    ) -> io::Result<Self> {
        let sock = Arc::new(Mutex::new(UdpSocket::bind("0.0.0.0:0")?));

        Ok(LightRuntime {
            north_south_addr,
            east_west_addr,
            button_addr,
            sock,
        })
    }

    fn emit_clock(sender: Sender<Event>) {
        loop {
            sender.send(Event::Clock).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    }

    fn watch_button(sender: Sender<Event>, sock: Arc<Mutex<UdpSocket>>) {
        let mut buf = [0; NBYTES];
        loop {
            {
                sock.lock().unwrap().recv(&mut buf).unwrap();
                sender.send(Event::Button).unwrap();
            }
        }
    }
}

impl<'a> Runtime for LightRuntime<'a> {
    /// Set the light color for a direction by sending a command.
    /// The command is sent to a UDP socket.
    fn set_color(&mut self, direction: Direction, command: &str) {
        let addr = match direction {
            Direction::NorthSouth => self.north_south_addr,
            Direction::EastWest => self.east_west_addr,
        };

        let msg = command.as_bytes();
        self.sock.lock().unwrap().send_to(&msg, addr).unwrap();
    }

    fn start(&mut self) {
        let mut controller = controller::Controller::default();
        let (sender, receiver) = mpsc::channel();

        let clock_sender = sender.clone();
        let mut handles = Vec::with_capacity(2);
        let clock_evt = thread::spawn(move || {
            Self::emit_clock(clock_sender);
        });

        handles.push(clock_evt);
        let button_sender = sender.clone();
        let sock = self.sock.clone();
        let button_evt = thread::spawn(move || {
            Self::watch_button(button_sender, sock);
        });

        for evt in receiver {
            let (ns_light, ew_light) = (controller.ns_light.clone(), controller.ew_light.clone());
            controller.event_handler(evt);
            if ns_light != controller.ns_light && ew_light != controller.ew_light {
                self.set_color(Direction::NorthSouth, &controller.ns_light.to_string());
                self.set_color(Direction::EastWest, &controller.ew_light.to_string());
            }
        }

        handles.push(button_evt);
        for h in handles {
            h.join().unwrap();
        }
    }
}
