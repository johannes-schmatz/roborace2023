mod line_follower;

mod robot;
mod menu;
#[cfg(feature = "menu")]
mod lcd;
mod pid;
mod settings;

use anyhow::{Context, Result};
use std::time::{Duration, Instant};
use crate::robot::Robot;
use crate::settings::Settings;
use crate::robot::state::RobotState;


fn main() -> Result<()> {
    std::env::set_var("RUST_BACKTRACE", "full");

    #[cfg(target_arch = "arm")]
    // setup the fonts
    std::process::Command::new("setfont")
        .arg("/usr/share/consolefonts/Lat2-Terminus14.psf.gz")
        .status()?;

    let mut settings = Settings::get()?;

    let next_state = RobotState::get_initial()
        .context("Failed to parse command line arguments")?;

    let robot = Robot::new()
        .context("Failed to create robot")?;

    settings.next_state(&robot, next_state)?;

    //robot.test()?;
    //return Ok(());

    let tick_time = Duration::from_millis(50);
    loop {
        let start = Instant::now();

        if settings.tick(&robot).context("Failed to tick robot")? {
            break;
        }

        let end = start.elapsed();

        if let Some(dur) = tick_time.checked_sub(end) {
            std::thread::sleep(dur)
        }
    }

    settings.write()?;

    Ok(())
}
