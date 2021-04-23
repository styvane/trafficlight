use structopt::StructOpt;
use traffic::light;

fn main() {
    let opt = light::LightArgs::from_args();
    let mut light = light::Light::new(opt);
    loop {
        light.change_color();
    }
}
