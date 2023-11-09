#!/bin/bash

cargo build --release && scp target/armv5te-unknown-linux-musleabi/release/roborace2023 europa:/home/robot/
