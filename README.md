## Servidor HTTP

This project was created in order to understand better how the HTTP/1.x works as well as learn a bit more about the Rust programming language. It is a personal project and isn't ready for any production enviroment, as the project is created with educational purposes, trying to use the least amount of external libraries possible.

### What works?

- Basic connection handling
    * Single threaded connection handling (The server blocks while handling a connection)
- Basic route handling
    * Routers
    * Different HTTP methods
    * Static files and routes
- Basic request handling
    * Handle querys
    * Handle request body
- Basic response handling
    * Added support for sending files

### What's going to be implemented?

- Multiple threaded route handling
- Connection rejection (Basic DoS handling, Slow loris...)
- Basic templating?
