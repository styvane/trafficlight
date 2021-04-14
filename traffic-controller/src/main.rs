use structopt::StructOpt;

use traffic_controller::runtime::{self, Runtime};

fn main() {
    let opt = runtime::RuntimeOpt::from_args();
    let mut rtime = runtime::LightRuntime::new(opt).unwrap();
    rtime.start();
}
