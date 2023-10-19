mod line_follower;
mod robot;
mod menu;
#[cfg(feature = "menu")]
mod lcd;

use anyhow::{Context, Result};
use std::time::{Duration, Instant};
use crate::robot::Robot;


fn main() -> Result<()> {
    std::env::set_var("RUST_BACKTRACE", "full");
    eprintln!("Started Program");

    let mut robot = Robot::new()
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

    Ok(())
}
