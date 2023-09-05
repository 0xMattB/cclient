# cclient

A simple practice console chat program written in Rust; client program, paired with the "cserver" program.

## Description

Run this program after running the "cserver" program. It expects an argument of the IP Address/Port Number of the server.

The server IP/Port is currently hard-coded as "192.168.1.121:4915".

## Getting Started

### Dependencies

There are currently no depencies for this project.

### Installing

Currently, downloading the source code and compiling locally is the only way to run this program.
```
// with Rust installed:
cargo new cclient
cd cclient
// copy source code into this directory
// compile the program
cargo build
```

### Executing program

* The program can be executed via the Rust environment:
```
cargo run -- 192.168.1.121:4915
```
* The program can also be executed via the .exe file:
```
cclient.exe 192.168.1.121:4915
```

When the client connects, follow the instructions sent by the server.

## Authors

0xMattB

## Version History

* 0.4.0
    * Implemented colorized text
* 0.3.0
    * Cleaned up and refactored code
    * Minor improvements
    * Works with "cserver 0.3.0"
* 0.2.0
    * Small updates to support modifications made in "cserver"
    * Works with "cserver 0.2.0"
* 0.1.0
    * Preliminary design
    * Basic code structure
    * Single-client communication
    * Log-in routine
    * Works with "cserver 0.1.0"

## Improvement Ideas

* ~~Text colorization~~
* Password hiding during input
* Implementation of commands
* "!exit" command closes local connection
* "!terminate" command shuts down server
* Broadcast user log-in (in a different colored text)

## License

T.B.D.
