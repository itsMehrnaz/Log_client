# Log_client

- **Source Code Structure:**
    
    - All Rust projects and exercises are organized in separate folders.
        
    - Each folder contains its own `main.rs` and relevant modules.
        
    - Projects are kept as samples and exercises for review and execution.
        
- **Projects Included:**
    
    - **TCP & Async TCP:** Sending and receiving messages using `TcpStream` and `tokio::net::TcpStream`.
        
    - **Log Handling:** Reading log files, structuring `LogPacket`, and serializing with `bincode`.
        
    - **Syslog Parsing:** Working with system logs, extracting PRI, timestamp, facility, and severity.
        
    - **Packet Structure:** Designing and using `LogPacket` structs and encoding/decoding them.
        
    - **File I/O:** Reading and writing files, appending logs, and maintaining history in `Vec<Log>`.
    - I will add projects when I write them...
        
- ** Usage Notes:**
    
    - Exercises are independent and can be executed separately.
        
    - Some examples include TCP/UDP networking, async/await, and Rust modularization.
        
- ** Highlights:**
    
    - Focus on **modular programming** using `serde`, `bincode`, and `tokio`.
        
    - Practical examples for **sending and receiving data over the network**.
        
    - Educational samples for **reading and processing system logs**.
