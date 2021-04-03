use clap::{clap_app, ArgMatches, App};

pub mod tunnel_list;
use tunnel_list::TunnelList;

pub mod tunnel_open;
use tunnel_open::TunnelOpen;

pub mod tunnel_kill_all;
use tunnel_kill_all::TunnelKillAll;

// use crate::log_matches;
#[derive(Debug)]
pub struct Tunnel {
    host_name: String,
    source_file: String,
}

impl Tunnel {
    pub fn app() -> App<'static>  {
        let core_app = clap_app!(tunnel =>
            (version: "0.1")
            (author: "Bruno Murino <bfsmurino@gmail.com>")
            (about: "controls testing features")
            (@setting ArgRequiredElseHelp)
        );

        let app = core_app
            .subcommand(TunnelList::app())
            .subcommand(TunnelOpen::app())
            .subcommand(TunnelKillAll::app());

        app
    }

    pub fn run(matches: &ArgMatches) {
        // log_matches(matches);
        match matches.subcommand() {
            Some(("open", matches)) => { TunnelOpen::run_from_matches(matches); },
            Some(("list", matches)) => { TunnelList::run_from_matches(matches); },
            Some(("kill", matches)) => { TunnelKillAll::run_from_matches(matches); },
            _                       => { println!("No"); },
        }
    }

}
