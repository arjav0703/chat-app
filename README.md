## 🎀 Rusty Chat 

Rusty Chat is a simple, TUI-based chat application built with Rust using the `cursive` crate for the user interface and `tokio` for asynchronous networking. It allows users to connect to a chat server and exchange messages in real-time.

### Project Structure
- `src/bin/server,rs`: The server-side implementation that handles incoming connections and broadcasts messages to all connected clients.
- `src/bin/client.rs`: The client-side implementation that connects to the server, sends messages, and displays incoming messages in a TUI.


### Supported Platforms:
- Linux
- macOS
- Windows (untested)

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
