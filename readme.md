# Carina

Carina is a sample blockchain written in rust.
The main purpose of this project is to learn rust and do something interesting.

## Goal

There is no specific goal.
I wanted to code something with rust and was very interessted in blockchin.
So I decided to do something with that.
This should not be another crypto currency, there are enough.

## Sub projects
The main project consits of mutliple sub projects. Each project is specialized for one thing.

Name         | Description
------------ | --------------------------------------------------------------------------------
debug        | Helps me to check if everything is ok. For example if the full mesh is build up.
hole_puncher | Global server that supports two peers behind NAT to build up a connection.
logging      | Small crate for logging. Contains some macros for logging across all crates.
peer         | Peer in the network for generating the next block
peer_cli     | Terminal application for a peer
protocol     | Crate for parsing the used protocol

## License
This project is dual licensed under Apache 2.0 and MIT. Please see the license files for more information.