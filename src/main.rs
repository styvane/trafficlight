use structopt::StructOpt;

use traffic::runtime::{self, Runtime};

fn main() {
    let opt = runtime::RuntimeArgs::from_args();
    let mut rtime = runtime::LightRuntime::new(opt).unwrap();
    rtime.start();
}
