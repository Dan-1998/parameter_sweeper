Parameter Sweeper

Overview

The idea of this project is to provide an abstract framework for managing model experiments/Monte Carlo Simulations.

How To Use

1) Add the rust model to be used to ./src/main.rs
2) Implement the Model trait for your model
3) Compile with `cargo build`
4) Run with `cargo run`

Planned Features
1) Add an example of a real life rust model being integrated into the framework
2) Add multithreading support to run multiple sets of parameters against multiple instances of the model at the same time.
3) Add serialization/deserialization support to save experiment state
4) Add database support for storing model results/experiment state
5) Add support for rust model dll to avoid having to compile model into framework
6) Add a user-friendly GUI



