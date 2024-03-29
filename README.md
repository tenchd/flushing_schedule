# LERT Flushing Schedule Visualizer
A visual simulator for the flushing schedule of the LERT. Running the program plays an animation which shows the status of each bin in the LERT at some epoch. You can change the epoch with the left and right arrow keys.

## Compatibility
Tested with rustc 1.64.0-nightly on Linux Mint 20.3. 
This visualizer uses [Termion](https://github.com/redox-os/termion), a library for controlling terminal output. Termion is supported on Redox, Mac OS X, BSD, and Linux (or, in general, ANSI terminals). 

## Installing rust
Follow the instructions at https://doc.rust-lang.org/stable/book/ch01-01-installation.html.

## Installing the visualizer
Clone the repository and cd into it. You're done!

## Running the simulator
```cargo run``` will start the simulation. It will prompt you to either run the simulation with a default LERT configuration or to specify the memory size, disk size, expansion factor, and timestretch yourself.  Running the program plays an animation which shows the status of each bin in the LERT at some epoch. You can change the epoch with the left and right arrow keys.

## Explaining the flushing schedule

![Flushing Schedule explanation](https://github.com/tenchd/flushing_schedule/blob/main/flush0.PNG?raw=true)

![Flushing-to Schedule explanation](https://github.com/tenchd/flushing_schedule/blob/main/flush1.PNG?raw=true)

![Flushing Schedule rosetta stone](https://github.com/tenchd/flushing_schedule/blob/main/flush2.PNG?raw=true)
