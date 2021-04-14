mod button;
use structopt::StructOpt;

fn main() {
    let opt = button::ButtonOption::from_args();
    let mut btn = button::Button::new(opt);
    while let Ok(_) = btn.push() {}
}
