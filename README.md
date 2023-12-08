
# Roborace 2023

https://www.ist.uni-stuttgart.de/roborace

Task was building a robot that enters a circle, while following a black line.
Then we needed to platoon behind another robot, and after a certain time that
vehicle left the track. Then we needed to exit the circle and continue following
the black line and finally stop.

We ran the robot using the `ev3dev` project.
We put the `ev3dev-stretch-ev3-generic-2020-04-10.img` on the microSD of the robot,
and then uploaded our compiled rust binary onto it with scp. We also used scp to copy
over the `robot_settings.toml`, which contained our config values.