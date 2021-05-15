# **Raft**
**Raft**, an alternative to Paxos, is a consensus algorithm which is similar to Paxos in fault-tolerance and performance, but decomposed into relatively independent subproblems and practically easier to implement.

You can learn more about this algorithm [here](https://raft.github.io/), which is a guide to the Raft [paper](https://raft.github.io/raft.pdf).

## **Significance**
The significance of this implementation is to understand the levers and gears of the Raft algorithm and to serve as the backbone for the cloud native distributed paraller executor, RaEx(Rust Parallel Executor), which would essentially be the compute engine for our experimental Ray Tracer which you can find [here](https://github.com/vyuham/rtrcrs/). 

The implementation is written in [Rust](https://www.rust-lang.org/).

## **Related Projects**
RaEX (under development)

[DStore](https://github.com/vyuham/dstore)

[RayTracer](https://github.com/vyuham/rtrcrs/)