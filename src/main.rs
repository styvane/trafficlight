use clap::Parser;

use traffic::runtime::{self, Runtime};

fn main() {
    let opt = runtime::RuntimeArgs::parse();
    let mut rtime = runtime::LightRuntime::new(opt).unwrap();
    rtime.start();
}
