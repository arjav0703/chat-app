## 🎀 Rusty Chat 

Rusty Chat is a simple, TUI-based chat application built with Rust using the `cursive` crate for the user interface and `tokio` for asynchronous networking. It allows users to connect to a chat server and exchange messages in real-time.

### Project Structure
- `src/bin/server,rs`: The server-side implementation that handles incoming connections and broadcasts messages to all connected clients.
- `src/bin/client.rs`: The client-side implementation that connects to the server, sends messages, and displays incoming messages in a TUI.


### Supported Platforms:
- Linux
- macOS
- Windows (untested)

### Installation
1. Ensure you have Rust and Cargo installed. You can install it from [rustup.rs](https://rustup.rs/).
2. Install using cargo:
   ```bash
   cargo install rusty-chat-app
    ```
3. Run the server:
    ```bash
    rusty-chat-server
    ```
4. In a new terminal, run the client:
    ```bash
    rusty-chat-client <username>
    ```

### Running the Application locally
1. Make sure you have Rust and Cargo installed. You can install it from [rustup.rs](https://rustup.rs/).
2. Clone the repository:
   ```bash
   git clone https://github.com/arjav0703/chat-app.git
   ```

3. Navigate to the project directory:
   ```bash
   cd chat-app
   ```

4. Start the server:
   ```bash
    cargo run --bin server
    ```

5. In a new terminal, start the client:
    ```bash
    cargo run --bin client <username>
    ```

### Acknowledgements
I used this (https://youtu.be/653rafFNBmA?si=mjYuBcQNu69gOYtM) video as a reference to build this project.
