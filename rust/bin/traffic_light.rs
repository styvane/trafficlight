use clap::Parser;

use traffic::light;

fn main() {
    let opt = light::LightArgs::parse();
    let mut light = light::Light::new(opt);
    loop {
        light.change_color();
    }
}
