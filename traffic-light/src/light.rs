//! This module contains an internet connected traffic light that displays three colors.
//!

use crossterm::style::{style, Attribute, Color};
use std::net::UdpSocket;
use structopt::StructOpt;

/// An internet connected traffic light that displays Red, Green and Yellow colors.
///
/// To change the color, you have to send "green", "red" or "yellow" via the UDP socket.
///
#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct LightOpt {
    #[structopt(short, long, default_value = "12000")]
    port: usize,

    /// The light direction
    #[structopt(short, long)]
    direction: String,
}

/// Light represent a traffic light.
pub struct Light {
    direction: String,
    sock: UdpSocket,
}

impl Light {
    /// Create a new light.
    pub fn new(opt: LightOpt) -> Self {
        let sock = UdpSocket::bind(format!("0.0.0.0:{}", opt.port)).unwrap();
        Light {
            direction: opt.direction,
            sock,
        }
    }

    /// Change light color based on received message.
    pub fn change_color(&mut self) {
        let mut buf = [0; 8192];
        if let Ok(received) = self.sock.recv(&mut buf) {
            let color = String::from_utf8(buf[..received].to_vec()).unwrap();
            let color: Color = color.parse().unwrap();
            match color {
                Color::Red | Color::Green | Color::Yellow => {
                    println!(
                        "{}",
                        style(&self.direction)
                            .with(color)
                            .attribute(Attribute::Bold)
                    )
                }
                _ => (),
            }
        }
    }
}
