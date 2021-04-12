mod button;
use structopt::StructOpt;

fn main() {
    let opt = button::ButtonOption::from_args();
    let mut btn = button::Button::new(opt);
    loop {
        if let Ok(_) = btn.push() {
            continue;
        } else {
            break;
        }
    }
}
