mod light;
use structopt::StructOpt;

fn main() {
    let opt = light::LightOpt::from_args();
    let mut light = light::Light::new(opt);
    loop {
        light.change_color();
    }
}
