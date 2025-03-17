# RVserver

RVserver is a simple and lightweight HTML server written in Rust. It dynamically serves frontend assets like HTML, CSS, JavaScript, and images when placed inside the public/ directory. This makes it easy to develop and deploy static websites or frontend applications.

### Features

✅ Serves HTML, CSS, JavaScript, images, and other static assets ✅ Automatically detects and loads files placed in the public/ folder ✅ Handles common HTTP methods like GET ✅ Simple and efficient request parsing ✅ Custom error pages for unsupported file types and server errors

### Installation

To use RVserver, ensure you have Rust installed on your system. If you haven’t installed Rust yet, get it from Rust's official site.

Then, clone the repository and build the server:

````
git clone https://github.com/Voxny404/RVserver.git
cd RVserver
cargo build --release
````

### Usage

Run the server using:
````
cargo run
````
Or, if using the compiled binary:

target/release/RVserver

The server will start and listen on http://localhost:8080 by default.

Serving Static Files

Simply place your frontend files inside the public/ directory. Example:
````
RVserver/
├── src/
├── public/
│   ├── index.html
│   ├── style.css
│   ├── script.js
│   ├── images/
│   │   ├── logo.png
````
Now, when you access http://localhost:8080/index.html, it will serve public/index.html.

### Configuration

You can modify the default settings by editing the main.rs file, including:

Port number (default: 8080)

Public folder path (default: public/)

Error handling behavior

Error Handling

RVserver automatically handles common errors:

404 Not Found: When a requested file is missing

500 Internal Server Error: For unexpected failures

If a request is made for an unsupported file type, the server logs the issue but does not serve the file.

Roadmap

✅ Basic HTTP request handling

✅ Dynamic file serving

⏳ Customizable error pages

⏳ Logging improvements

⏳ Support for additional HTTP methods (POST, PUT, DELETE)

License

This project is licensed under the MIT License.

Enjoy using RVserver! 🚀 If you have suggestions or want to contribute, feel free to open an issue or a pull request!
