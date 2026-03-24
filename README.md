# LERT Flushing Schedule Visualizer
This repo is a visualizer for the flushing schedule used by an in-development variant of the Leveled External-Memory Reporting Table (LERT) first described in [Timely Reporting of Heavy Hitters Using External Memory](https://dl.acm.org/doi/epdf/10.1145/3472392) by Singh et. al. It is intended to assist researchers/developers by simulating (via closed-form calculation) the sequence of external memory data movement operations that a LERT will execute during operation. 

## Motivation
Real-time monitoring of high-rate data streams, with the goal of detecting and preventing malicious events, is a critical component of defense systems for cybersecurity as well as for physical systems, e.g., for water or power distribution. In such a monitoring system, the stream of observations represent the changes to the state of the system. An event is a (not necessarily contiguous) sequence of observations which may indicate malicious activity on the network. Each detected/reported event triggers an intervention. Analysts use more specialized tools to gauge the actual threat level. Some systems automatically take defensive actions, such as blocking a remote host, based on detected events. Accuracy (i.e., few false-positives and no false-negatives) and timeliness of event reporting are essential to these systems.

This task is particularly difficult when the data stream is high-bandwidth and of unbounded length. Suspicious events often emerge gradually: each component observation in a suspicious event may only occur very rarely, and they may be separated by
billions of other, unrelated intervening observations. Such events are called low and slow. Detecting a pattern of low and slow events is memory-intensive; if the system stores its data structures in RAM only, even for large RAM, by the time the final
observation in an event occurs, it is almost certain that some of the earlier observations have been discarded for lack of space.  As an example of a high-bandwidth application, a system that monitors host-based data for all devices in a large secure network must contend with the high volume of data these devices generate and detect all events (even low and slow ones) with low latency.

## What is a LERT?
A Leveled External-Memory Reporting Table (LERT) is an external-memory data structure designed to detect events with provable correctness and latency guarantees. The paper cited above provides a description of the original LERT design; a full description of the improved variant will be made available after publication. The important thing to understand is that a LERT is composed of arrays of bins which are organized into levels. The array of bins at the first level are of a (relatively) small size and this array can be stored in RAM. However, the size of each subsequent array of bins increases geometrically. Cybersecurity observations are streamed into the LERT and inserted into the top array of bins. When all the bins at level i are filled, the oldest bin at this level flushes, meaning it moves its data to one of the larger bins in level i+1. Flushes may also trigger event detection algorithms, which are beyond the scope of this repo.


## Project Goals
The goal of this repo is, given specific parameters that govern the size, throughput, and reporting latency of a LERT, to predict and concisely visualize the times at which various bins will flush. This is a crucial function for designing and testing a LERT implementation.

## Flushing Schedule Simulator Details
The time granularity of this visualizer is the epoch, which is the time required for one of the the top-level bins to completely fill with observations from an initially empty state. The simulator displays all bins in the LERT, initially at epoch 0, which by convention we define to be the moment that the first bin at the first level fills for the first time during the stream. The user can advance or rewind the simulator to different epochs, and it will display the current status of each bin (empty, nonempty, flushing) at that epoch.

The simulator allows the user to specify several key parameters that affect LERT size, layout, and flushing behavior:
- memory size: the size of RAM on the simulated system.
- disk size: the size of available high-bandwidth storage on the simulated system.
- expansion factor: the multiplicative factor size increase between the bins at level i and level i+1.
- timestretch: a parameter between 0 and 1 that controls how quickly the system is guaranteed to report an event after it occurs.
- depth: the number of levels in the LERT.
- c: the number of bins in each level of the LERT.

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

These are notes used to explain the closed-form flushing schedule expression, left here for posterity.

![Flushing Schedule explanation](https://github.com/tenchd/flushing_schedule/blob/main/flush0.PNG?raw=true)

![Flushing-to Schedule explanation](https://github.com/tenchd/flushing_schedule/blob/main/flush1.PNG?raw=true)

![Flushing Schedule rosetta stone](https://github.com/tenchd/flushing_schedule/blob/main/flush2.PNG?raw=true)
