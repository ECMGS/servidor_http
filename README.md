## Servidor HTTP

This project was created in order to understand better how the HTTP/1.x works as well as learn a bit more about the Rust programming language. It is a personal project and isn't ready for any production enviroment, as the project is created with educational purposes, trying to use the least ammount of external libraries possible.

### What works?

- Basic connection handling
    * Single threaded connection handling (The server blocks while handling a connection)
- Basic route handling
    * Routers
    * Different HTTP methods
- Basic request handling
    * Handle querys
    * Handle request body

### What's going to be implemented?

- Multiple threaded route handling
- Connection rejection (Basic DoS handling, Slow loris...)
- Serve static files
- Send http files as response
- Basic templating?