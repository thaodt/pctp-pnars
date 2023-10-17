# pctp-pnars
Following along the PingCAP's Talent Plan course - Practical Networked Applications in Rust.

## Project by project
- [x] Project 1: Create an in-memory key/value store that passes simple tests and responds to command-line arguments.
    - I'm gonna write a blog about using `clap` v4 also.
- [] Project 2: Create a persistent key/value store that can be accessed from the command line. (should upgrade from the previous project).
- [] Project 3: Create a single-threaded, persistent key/value store server and client with synchronous networking over a custom protocol.
I may want to use Noise protocol (for secure channels) and Stream Multiplexing (Yamux) for this one. But for the simple version, I may do a TCP
protocol. Then upgrade it for next version.
- [] Project 4: Create a multi-threaded, persistent key/value store server and client with _synchronous_ networking over a custom protocol.
- [] Project 5: Create a multi-threaded, persistent key/value store server and client with _asynchronous_ networking over a custom protocol.


Let's see how far I can go.

Happy coding! :heart:
