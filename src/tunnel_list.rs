
// use std::env;
use subprocess::{Exec};
use clap::{App, clap_app, ArgMatches};

pub struct TunnelList;
impl TunnelList {

    pub fn app() -> App<'static>  {        
        let core_app = clap_app!(list =>
            (visible_alias: "ls")
            (about: "Lists currently open SSH tunnels started via \"ssh -f\"")
            (version: "1.2")
            (author: "Someone E. <someone_else@other.com>")
            (@arg verbose: -v --verbose "Print test information verbosely")
        );
        let app = core_app;
        app
    }

    pub fn run_from_matches(matches: &ArgMatches) {
        println!("Tunnel List matches: {:?}", matches);
        TunnelList::run()
    }

    pub fn run() {
        
        match {
            Exec::cmd("ps").arg("-eo").arg("pid,command")
            | Exec::cmd("grep").arg("ssh -f")
            | Exec::cmd("grep").arg("-v").arg("grep")
        }.capture() {
            Err(why) => {
                println!("couldn't spawn wc: {}", why);
            },
            Ok(out) => {
                let out = out.stdout_str();
                let lines: Vec<&str> = out.lines().collect();
                match lines.len() {
                    0 => println!("No SSH tunnels open"),
                    _ => {
                        for line in lines {
                            println!("{}", line);
                        }
                    }
                }
            },
        };
        println!("Reached End Gracefully")
    }
}