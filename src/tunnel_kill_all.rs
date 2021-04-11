
// use std::env;
use subprocess::{Exec};
use clap::{App, clap_app, ArgMatches};

use crate::tunnel_list::TunnelList;

pub struct TunnelKillAll;
impl TunnelKillAll {

    pub fn app() -> App<'static>  {        
        let core_app = clap_app!(kill =>
            (visible_alias: "k")
            (version: "0.1")
            (author: "Bruno Murino <bfsmurino@gmail.com>")
            (about: "Kills all ssh tunnels opened with \"ssh -f\"")
            (@arg verbose: -v --verbose "Print test information verbosely")
        );
        let app = core_app;
        app
    }

    pub fn run_from_matches(_matches: &ArgMatches) {
        // println!("Tunnel List matches: {:?}", matches); // only on debug
        TunnelKillAll::run()
    }

    pub fn run() {
        let list_of_open_tunnels = TunnelList::run();
        for open_tunnel in list_of_open_tunnels.keys() {
            println!("{}", open_tunnel);
            Exec::cmd("kill").arg(&open_tunnel).join().unwrap();
        }
        // println!("Reached End Gracefully")  // only on debug
    }
}