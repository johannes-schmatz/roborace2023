mod robot;
mod menu;
mod pid;
mod program;
mod io;
mod state;

use anyhow::{Context, Result};

use crate::robot::Robot;

fn main() -> Result<()> {
    // We want long stack traces.
    std::env::set_var("RUST_BACKTRACE", "full");

    // Only run this we there's no argument (first one is the program itself).
    if std::env::args().len() == 1 {
        #[cfg(target_arch = "arm")]
        // setup the fonts
        std::process::Command::new("setfont")
            .arg("/usr/share/consolefonts/Lat2-Terminus14.psf.gz")
            .status()?;
    }

    //TODO: consider running via ssh
    // We run it over ssh on the robot. Not sure if they find that strange or not.
    //TODO: find a way to sync the robot_settings.toml
    // We just manually copy over the robot_settings.toml before starting and after
    // being done.

    let bot = Robot::new().context("Failed to create robot")?;

    let mut program = io::read().context("Failed to read the config file")?;

    let res = program.main(&bot);
    // Before looking at the result, we stop all the motors.
    // This ensures that when the program exits (besides panic), we stop the motors.
    let _ = bot.left.stop();
    let _ = bot.right.stop();
    let _ = bot.top_arm.stop();
    res?;

    Ok(())
}
