mod line_follower;
mod robot;
mod menu;
mod lcd;

use anyhow::{Context, Result};
use std::time::{Duration, Instant};
use crate::robot::Robot;


fn main() -> Result<()> {

    let mut robot = Robot::new()
        .with_context(|| "Failed to create robot")?;

    let tick_time = Duration::from_millis(50);
    loop {
        let start = Instant::now();

        if robot.tick().with_context(|| "Failed to tick robot")? {
            break;
        }

        let end = start.elapsed();

        if let Some(dur) = tick_time.checked_sub(end) {
            std::thread::sleep(dur)
        }
    }

    Ok(())
}
