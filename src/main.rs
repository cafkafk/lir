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

fn get_current_brightness() -> std::io::Result<i32> {
    let mut current_brightness_file =
        File::open("/sys/class/backlight/intel_backlight/brightness")?;
    let mut current_brightness_buffer = String::new();
    // POP removes trailing newline
    current_brightness_file.read_to_string(&mut current_brightness_buffer)?;
    current_brightness_buffer.pop();
    Ok(current_brightness_buffer
        .parse::<i32>()
        .expect("max brightnesss does not cleanly parse as i32"))
}

fn get_current_brightness_percentage() -> std::io::Result<f32> {
    let max_brightness = get_max_brightness()?;
    let current_brightness = get_current_brightness()?;

    Ok((current_brightness as f32 / max_brightness as f32) * 100.)
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

/// Changes the brightness by percentage relative to max brightness
fn change_brightness_percentage(brightness: i32) -> std::io::Result<()> {
    let max_brightness = get_max_brightness()?;
    let current_brightness = get_current_brightness()?;
    let delta_brightness = (max_brightness / 100) * brightness.abs();

    trace!("{}", delta_brightness);
    trace!("{}", brightness);
    trace!("{}", max_brightness);
    trace!("{}", current_brightness);

    let mut new_brightness = current_brightness + delta_brightness * brightness.signum();

    if new_brightness > max_brightness {
        new_brightness = max_brightness;
    }
    trace!("{}", new_brightness);
    fs::write(
        "/sys/class/backlight/intel_backlight/brightness",
        new_brightness.to_string(),
    )?;

    Ok(())
}

fn main() -> std::io::Result<()> {
    pretty_env_logger::init();

    let matches = crate::cli::build_cli().get_matches();

    if let Some(brightness) = matches.get_one::<i32>("percentage") {
        set_brightness_percentage(*brightness)?;
    } else if let Some(brightness) = matches.get_one::<i32>("change") {
        change_brightness_percentage(*brightness)?;
    } else if matches.get_flag("get") {
        println!(
            "{}%",
            get_current_brightness_percentage().expect("failed to get current brightness") as i32
        );
    } else {
        set_brightness_percentage(
            get_max_brightness().expect("max brightnesss does not cleanly parse as i32"),
        )?;
    }

    Ok(())
}
