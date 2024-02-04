mod lights;

use std::fs;
use std::fs::File;
use std::io::Read;

extern crate log;
extern crate pretty_env_logger;

mod cli;

#[allow(unused)]
use log::{debug, error, info, trace, warn};

fn get_max_brightness() -> std::io::Result<i32> {
    // TODO: should be max_brightness as parsed... or something that does that
    let mut max_brightness_file =
        File::open("/sys/class/backlight/intel_backlight/max_brightness")?;
    let mut max_brightness_buffer = String::new();
    // POP removes trailing newline
    max_brightness_file.read_to_string(&mut max_brightness_buffer)?;
    max_brightness_buffer.pop();
    Ok(max_brightness_buffer
        .parse::<i32>()
        .expect("max brightnesss does not cleanly parse as i32"))
}

fn set_brightness_percentage(brightness: i32) -> std::io::Result<()> {
    let percentage = (get_max_brightness().expect("max brightnesss does not cleanly parse as i32")
        / 100)
        * brightness;
    fs::write(
        "/sys/class/backlight/intel_backlight/brightness",
        percentage.to_string(),
    )?;

    Ok(())
}

fn main() -> std::io::Result<()> {
    pretty_env_logger::init();

    let matches = crate::cli::build_cli().get_matches();

    if let Some(brightness) = matches.get_one::<i32>("percentage") {
        set_brightness_percentage(*brightness)?;
    } else {
        set_brightness_percentage(
            get_max_brightness().expect("max brightnesss does not cleanly parse as i32"),
        )?;
    }

    Ok(())
}
