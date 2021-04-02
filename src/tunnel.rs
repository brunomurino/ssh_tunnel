use clap::{clap_app, ArgMatches, App};

pub mod tunnel_list;
use tunnel_list::TunnelList;

pub mod tunnel_open;
use tunnel_open::TunnelOpen;

// use crate::log_matches;

pub struct Tunnel;
impl Tunnel {
    pub fn app() -> App<'static>  {
        let core_app = clap_app!(tunnel =>
            (version: "1.3")
            (author: "Someone E. <someone_else@other.com>")
            (about: "controls testing features")
            // (@setting ArgRequiredElseHelp)
        );

        let app = core_app
            .subcommand(TunnelList::app())
            .subcommand(TunnelOpen::app());

        app
    }

    pub fn run(matches: &ArgMatches) {
        // log_matches(matches);
        match matches.subcommand() {
            Some(("open", matches)) => { TunnelOpen::run_from_matches(matches); },
            Some(("list", matches)) => { TunnelList::run_from_matches(matches); },
            _                         => { println!("No"); },
        }
    }

}

// fn main() {
//     println!("Hello from tunnel main");
//     let app = Tunnel::app();
//     let matches = app.get_matches();
//     Tunnel::run(&matches);
// }
