# cclient

A simple practice console chat program written in Rust; client program, paired with the "cserver" program.

* 0.1.0: Preliminary design: Basic code structure, single-client communication, log-in routine; works with "cserver 0.1.0".
* 0.2.0: Small updates to support modifications made in "cserver"; works with "cserver 0.2.0"
* 0.3.0: Cleaned up and refactored code, minor improvements; works with "cserver 0.3.0"
* 0.4.0: Added coloring to certain messages

Improvement ideas:
* ~~Text colorization~~
* Password hiding during input
* Implementation of commands
* "!exit" command closes local connection
* "!terminate" command shuts down server
* Broadcast user log-in (in a different colored text)
