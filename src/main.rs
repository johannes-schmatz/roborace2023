mod line_follower;
mod gradient_follower;

mod robot;
mod menu;
#[cfg(feature = "menu")]
mod lcd;
mod pid;
mod motor;
mod settings;

use anyhow::{bail, Context, Result};
use std::time::{Duration, Instant};
use crate::robot::{Robot, RobotState};
use crate::settings::Settings;


fn main() -> Result<()> {
    std::env::set_var("RUST_BACKTRACE", "full");

    let mut settings = Settings::get()?;
    settings.state = RobotState::get_initial()
        .context("Failed to parse command line arguments")?;

    let robot = Robot::new()
        .context("Failed to create robot")?;

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
