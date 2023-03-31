// Import the `fs` module so we can read the HTML data from `index.html`. We use this data to
// respond to requests
use std::fs;

// Bring in the TcpListener struct from the standard library so we can listen to TCP requests
use std::net::TcpListener;

// Bring in the TcpStream struct.
use std::net::TcpStream;

// Bring in everything from io prelude
use std::io::prelude::*;

fn main() {
    // Bind a listener to port `7878`
    // TODO: Handle errors gracefully instead of using `unwrap()`.
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // Loop through the connections. `listener.incoming()` gives us an iterator over the connections
    // being received by the listener in the form of a TcpStream
    for stream in listener.incoming() {
        // Shadow the stream variable. Get the TcpStream or panic
        let stream = stream.unwrap();

        // Print that we have a connection
        println!("Connection established.");

        // Pass the stream to our handler function
        handle_connection(stream);
    }
}

// Take a mutable TcpStream. Reads data from the stream and prints it out. The `stream` argument is
// mutable because the `stream.read()` method takes a mutable reference to self. This is because
// when you're reading from a stream, some internal state gets modified.
fn handle_connection(mut stream: TcpStream) {
    // Create a buffer to hold the data that is read. It is `1024` bytes long so it can hold the
    // basic request we're working with.
    // TODO: The buffer should handle requests of any size
    let mut buffer = [0; 1024];

    // Read data from the stream. Populates the buffer with data from the stream.
    stream.read(&mut buffer).unwrap();

    // Depending on the request, we are going to set a specific status line and content. We can save
    // these two pieces of information in a tuple.
    let (status_line, filename) =
        // If the request starts with this specific string (as bytes), the request is looking for
        // the root/homepage
        if buffer.starts_with(b"GET / HTTP/1.1\r\n") {
            // Set the status to OK and the filename is the homepage
            ("HTTP/1.1 200 OK", "index.html")
        } else {
            // If the request isn't for root/homepage, it is looking for something we don't have.
            // Response with a 404 NOT FOUND and the 404 HTML file
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        };

    // Get the contents of the requested HTML file
    let contents = fs::read_to_string(filename).unwrap();

    // Generate the response
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        // The status line (ex: `HTTP/1.1 200 OK`) comes first
        status_line,
        // The `Content-Length` is the length of the content in bytes
        contents.len(),
        // The content itself (the HTML) comes last
        contents
    );

    // Write the response to the stream as bytes
    stream.write(response.as_bytes()).unwrap();

    // Make sure all data has been written to the stream
    stream.flush().unwrap();

    // Print out the contents of the request buffer. `from_utf8_lossy` converts a slice of bytes
    // into a string, including invalid characters. We give it a slice of our buffer spanning the
    // entire buffer
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}
