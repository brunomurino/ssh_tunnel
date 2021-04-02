
use ::ssh_tunnel::Tunnel;

fn main() {
    println!("Hello from tunnel main");
    let app = Tunnel::app();
    let matches = app.get_matches();
    Tunnel::run(&matches);
}