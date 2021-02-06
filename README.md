# rust-web-scrape

This demonstrates using Rust to scrape web sites.
The main purpose is really to show how to use threads in Rust.

The web sites to scrape listed in the file `web-sites.txt`.
The code reports the number of `img` tags found at each site.
First it does this with a single thread and
then spawning a separate tasks for each site
to enable using multiple threads.
The elapsed time for each approach is output
to show the speed benefit of using multiple threads.

With 19 web sites listed in `web-sites.txt` and running on
a 2019 MacBook Pro laptop with 32GB of memory and 8 cores,
the single-threaded approach took 16.18 seconds
and the multi-threaded approach took 707 milliseconds.

To build this, enter `cargo build --release`.

To run this, enter `./target/release/rust-web-scrape`.
