use clap::Parser;

use traffic::button;

fn main() {
    let opt = button::ButtonArgs::parse();
    let mut btn = button::Button::new(opt);
    while btn.push().is_ok() {}
}
