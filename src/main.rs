mod robot;
mod menu;
mod pid;
mod program;
mod io;
mod state;

use anyhow::{Context, Result};

use crate::robot::Robot;

// TODO: scp the run.sh over as well!
//
//TODO: For actual running do the following:
// - Measure the black and white values.
//   Values of about 8 for black and about 94 for white are good. (done!)
// - Find the matching circle: 78cm, 100cm or 129cm:
//   - Small one:
//     We hope that they didn't chose this circle.
//     We need a relatively large k_p, and a large k_i.
//     Also is the maximum speed somewhat limited.
//   - Medium one:
//     A normal k_p and a normal k_i are sufficient.
//     We cannot drive with v = 80 because of the problems
//     with the speed of the right motor. (this one!)
//   - Large one:
//     This is the easiest to drive on.
//     We use our normal values everywhere.
//     The speed can go up to v = 85 probably.
// - Ensure that the rotating arm matches their ball position.

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
