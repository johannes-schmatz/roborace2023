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

fn get_initial_robot_state() -> Result<Option<RobotState>> {
    let string = std::env::args()
        .skip(1)
        .next()
        .unwrap_or(String::from("menu"));
    match string.as_str() {
        "menu" => Ok(Some(RobotState::InMenu)),
        "test" => Ok(Some(RobotState::Test)),
        x => {
            let vec = [
                "menu",
                "test",
            ];
            bail!("Cannot parse argument {x:?}, we only know the following: {vec:?}")
        },
    }
}

fn main() -> Result<()> {
    std::env::set_var("RUST_BACKTRACE", "full");

    let initial_state = get_initial_robot_state()
        .context("Failed to parse arguments")?
        .unwrap_or(RobotState::InMenu);

    let mut settings = Settings::get()?;

    let mut robot = Robot::new(initial_state, &settings)
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
