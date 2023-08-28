use std::env;

mod desktop;
mod lockscreen;

fn main() {
    let args: Vec<String> = env::args().collect();

    let target = args
        .get(1)
        .expect("Missing target: `desktop` or `lockscreen`");

    let path = args.get(2).expect("Missing image path");

    match &target[..] {
        "desktop" => unsafe { desktop::set_image(&path) },
        "lockscreen" => lockscreen::set_image(&path),
        other => panic!("Invalid target `{}`", other),
    }
}
