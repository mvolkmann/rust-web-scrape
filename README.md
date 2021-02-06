# rust-web-scrape

This demonstrates using Rust to scrape web sites.
The main purpose is really to show how to use threads in Rust.
It scrapes the web sites listed in the file `web-sites.txt`
and reports the number of `img` tags found at each site.
It does this first with a single thread and
then with a separate thread for each site.
The elapsed time for each approach is output
to show the speed benefit of using multiple threads.

To build this, enter `cargo build --release`.

To run this, enter `./target/release/rust-web-scrape`.
