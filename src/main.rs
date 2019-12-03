//use 
extern crate wayland_client;

fn main() {
    // critical error if can't connect to display
    let display = wayland_client::Display::connect_to_env().unwrap();

    println!("got a display");
    // display disconnects at end of scope
}
