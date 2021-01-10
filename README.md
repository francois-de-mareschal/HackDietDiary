# The Hacker's Diet Diary

The Hacker's Diet Diary is a diary intended to be an easier alternative to LibreOffice 
Calc / Microsoft Excel sheets-based diet diaries.

## The Hacker's Diet

### What is it?

The [Hacker's Diet](https://www.fourmilab.ch/hackdiet/e4) is a method to lose weight
designed as an engineering project for engineers. The author does not sell anything,
he provides a method and a set of tools to measure and manage the weight loss. 
By measuring the weight on a daily basis, these tools provide a trend, bullish or bearish, 
and the excess or deficit of ingested calories. It only remains to adjust the meals.

### Does it works?

If you're patient enough and adjust your meals following the trend, it works. The best
description of the method is in the book, anyway. It worked for me.

## The Hacker's Diet Diary

The Hacker's Diet Diary is an implementation of the set of tools described in the book. It is
intended to provide an extremely fast way to record the weight on the morning.

### Objectives

- allow to store as fast as possible (from the user's point of view) the measured constants,
like the weight;
- provide feedback to adjust the meals;
- define weight loss objectives;
- view the recorded data to measure progress.

### Build & Contributing

#### Build

The project is developed in Rust; after cloning the repository:

```shell
# Run the unit tests suite.
cargo test
# Build the whole project.
cargo build --release
# Run (command-line arguments and options to be defined).
cargo run --release -- # commands
```

#### Contributing

All the Rust source files are formatted by ```rustfmt``` with all default parameters.

This project uses the [GitLab flow](https://docs.gitlab.com/ee/topics/gitlab_flow.html).


## License

    Licensed under the EUPL

