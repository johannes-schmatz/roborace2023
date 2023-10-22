mod line_follower;
mod gradient_follower;

mod robot;
mod menu;
#[cfg(feature = "menu")]
mod lcd;
mod pid;
mod motor;
mod settings;

use anyhow::{Context, Result};
use std::time::{Duration, Instant};
use crate::robot::Robot;
use crate::settings::Settings;

fn main() -> Result<()> {
    std::env::set_var("RUST_BACKTRACE", "full");

    let mut settings = Settings::get()?;

    let mut robot = Robot::new(&settings)
        .context("Failed to create robot")?;

    //robot.test()?;
    //return Ok(());

    let tick_time = Duration::from_millis(50);
    loop {
        let start = Instant::now();

        if robot.tick().context("Failed to tick robot")? {
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
