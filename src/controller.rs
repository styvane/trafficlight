//! This module implements the traffic light controller

use std::fmt;
use std::time::Duration;

/// The type `Color` represents a traffic light color.
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Color {
    Red,
    Green,
    Yellow,
}

impl ToString for Color {
    fn to_string(&self) -> String {
        match self {
            Self::Red => String::from("red"),
            Self::Green => String::from("green"),
            Self::Yellow => String::from("yellow"),
        }
    }
}

/// The type `Event` is a controller event.
#[derive(Debug, Clone)]
pub enum Event {
    Clock,
    Button,
}

/// The `Controller` type represents the traffic light controller.
///
/// It controls the "North-South" and "East-West" traffic lights.
#[derive(Debug)]
pub struct Controller {
    pub ns_light: Color,
    pub ew_light: Color,
    pub clock: Duration,
    pub button_is_pressed: bool,
}

impl Default for Controller {
    fn default() -> Self {
        Controller {
            ew_light: Color::Green,
            ns_light: Color::Red,
            clock: Duration::from_secs(0),
            button_is_pressed: false,
        }
    }
}

impl fmt::Display for Controller {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Controller<ns_light={}, ew_light={}, clock={}, button_is_pressed={}",
            self.ns_light.to_string(),
            self.ew_light.to_string(),
            self.clock.as_secs(),
            self.button_is_pressed
        )
    }
}

impl Controller {
    const EAST_WEST_GREEN: Duration = Duration::from_secs(30);
    const YELLOW: Duration = Duration::from_secs(5);
    const NORTH_SOUTH: Duration = Duration::from_secs(60);

    /// Create a new controller.
    pub fn new(ns_light: Color, ew_light: Color, clock: Duration) -> Self {
        Controller {
            ns_light,
            ew_light,
            clock,
            button_is_pressed: false,
        }
    }

    /// Set the value of "button_is_pressed" when the pedestrian button is pressed.
    pub fn press_button(&mut self) {
        self.button_is_pressed = true;
        if self.clock >= Self::EAST_WEST_GREEN {
            self.ns_light = Color::Yellow;
            self.button_is_pressed = false;
            self.reset_clock();
        }
    }

    /// Reset the clock.
    pub fn reset_clock(&mut self) {
        self.clock = Duration::from_secs(0);
    }

    /// Handle timer of button event.
    pub fn event_handler(&mut self, event: Event) {
        match event {
            Event::Clock => self.next_clock(),
            Event::Button => self.press_button(),
        }
    }

    /// Increment the clock
    pub fn next_clock(&mut self) {
        self.clock += Duration::from_secs(1);
        let mut must_reset_clock = true;

        match (&self.ns_light, &self.ew_light) {
            (Color::Red, Color::Green) if self.clock >= Self::EAST_WEST_GREEN => {
                self.ew_light = Color::Yellow
            }

            (Color::Red, Color::Yellow) if self.clock == Self::YELLOW => {
                self.ns_light = Color::Green;
                self.ew_light = Color::Red;
            }
            (Color::Green, Color::Red)
                if self.clock >= Self::EAST_WEST_GREEN && self.button_is_pressed
                    || self.clock == Self::NORTH_SOUTH =>
            {
                self.ns_light = Color::Yellow;
                self.ew_light = Color::Red;
                self.button_is_pressed = false;
            }

            (Color::Yellow, Color::Red) if self.clock == Self::YELLOW => {
                self.ns_light = Color::Red;
                self.ew_light = Color::Green;
            }
            _ => {
                must_reset_clock = false;
            }
        }
        if must_reset_clock {
            self.reset_clock();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let ctl = Controller::default();
        assert_eq!(ctl.ns_light, Color::Red);
        assert_eq!(ctl.ew_light, Color::Green);
        assert_eq!(ctl.clock.as_secs(), 0);
        assert!(!ctl.button_is_pressed);
    }
    #[test]
    fn test_red_green_to_red_yellow() {
        let mut control = Controller::new(Color::Red, Color::Green, Duration::from_secs(29));
        control.next_clock();
        assert_eq!(control.ns_light, Color::Red);
        assert_eq!(control.ew_light, Color::Yellow);
        assert_eq!(control.clock.as_secs(), 0);
    }
    #[test]
    fn test_red_yellow_to_green_red() {
        let mut control = Controller::new(Color::Red, Color::Yellow, Duration::from_secs(4));
        control.next_clock();
        assert_eq!(control.ns_light, Color::Green);
        assert_eq!(control.ew_light, Color::Red);
        assert_eq!(control.clock.as_secs(), 0);
    }

    #[test]
    fn test_green_red_to_yellow_red_with_button() {
        let mut control = Controller::new(Color::Green, Color::Red, Duration::from_secs(28));
        control.next_clock();
        assert_eq!(control.ns_light, Color::Green);
        assert_eq!(control.ew_light, Color::Red);
        control.press_button();
        assert!(control.button_is_pressed);
        control.next_clock();
        assert_eq!(control.ns_light, Color::Yellow);
        assert_eq!(control.ew_light, Color::Red);
        assert_eq!(control.clock.as_secs(), 0);
    }

    #[test]
    fn test_green_red_to_red_yellow() {
        let mut control = Controller::new(Color::Green, Color::Red, Duration::from_secs(59));
        control.next_clock();
        assert_eq!(control.ns_light, Color::Yellow);
        assert_eq!(control.ew_light, Color::Red);
        assert_eq!(control.clock.as_secs(), 0);
    }

    #[test]
    fn test_yellow_red_to_red_green() {
        let mut control = Controller::new(Color::Yellow, Color::Red, Duration::from_secs(4));
        control.next_clock();
        assert_eq!(control.ns_light, Color::Red);
        assert_eq!(control.ew_light, Color::Green);
    }
}
