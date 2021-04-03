
// use std::env;
use subprocess::{Exec, PopenError};
use clap::{App, clap_app, ArgMatches};

pub struct TunnelList;
impl TunnelList {

    pub fn app() -> App<'static>  {        
        let core_app = clap_app!(list =>
            (visible_alias: "ls")
            (version: "0.1")
            (author: "Bruno Murino <bfsmurino@gmail.com>")
            (about: "Lists currently open SSH tunnels started via \"ssh -f\"")
            (@arg verbose: -v --verbose "Print test information verbosely")
        );
        let app = core_app;
        app
    }

    pub fn run_from_matches(matches: &ArgMatches) -> Vec<String> {
        println!("Tunnel List matches: {:?}", matches);
        TunnelList::run()
    }

    pub fn run() -> Vec<String> {

        let lines = Self::get_list_of_open_tunnels().unwrap();

        // println!("\n{:#?}", lines);
        
        if lines.len() > 0 {
            for lin in &lines {
                println!("{}", lin);
            }
        } else {
            println!("No SSH tunnels open");
        }

        println!("Reached End Gracefully");

        lines
    }

    pub fn get_list_of_open_tunnels() -> Result<Vec<String>, PopenError> {
        let out = {
            Exec::cmd("ps").arg("-eo").arg("pid,command")
            | Exec::cmd("grep").arg("ssh -f")
            | Exec::cmd("grep").arg("-v").arg("grep")
        }.capture()?.stdout_str();
        let lines: Vec<String> = out.lines().map(str::to_string).collect();
        Ok(lines)
    }
}