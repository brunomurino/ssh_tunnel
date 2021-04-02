use clap::{App, clap_app, ArgMatches};
// use std::path::Path;
// use std::env;
use std::{fs, io, io::{Write}, collections::BTreeMap};
use subprocess::{Exec, ExitStatus};
use dirs::home_dir;

use crate::tunnel_list::TunnelList;

pub struct TunnelOpen;
impl TunnelOpen {

    pub fn app() -> App<'static>  {        
        let core_app = clap_app!(open =>
            (visible_alias: "o")
            (about: "Opens pre-defined SSH tunnels")
            (version: "1.2")
            (author: "Someone E. <someone_else@other.com>")
        );
        let app = core_app;
        app
    }

    pub fn run_from_matches(matches: &ArgMatches) {
        println!("Tunnel Open matches: {:?}", matches);
        TunnelOpen::run()
    }

    pub fn run() {
        let home = home_dir().unwrap();
        let filepath = home.join(".ssh/landbay_tunnels");
        println!("{}", filepath.to_str().unwrap());

        let data = fs::read_to_string(filepath).expect("Unable to read file");

        let mut lines: Vec<&str> = data.lines().collect();
        lines.retain(|&line| line.contains("Host ") );

        let ready_lines: Vec<_> = lines.iter().map(|line| line.replace("Host ","")).collect();

        let mut map_of_tunnels = BTreeMap::new();

        for (i, line) in ready_lines.iter().enumerate() {
            map_of_tunnels.insert(i+1, line);
        }
        
        println!("List of possible tunnels to open:\n");
        for (i, host_name) in &map_of_tunnels {
            println!("  {}) {}", i, host_name);
        }

        print!("\nSelect which tunnel to open: ");
        io::stdout().flush().unwrap();
        let mut user_choice = String::new();
        io::stdin()
            .read_line(&mut user_choice)
            .expect("Failed to read line");
        let user_choice: usize = user_choice.trim().parse().expect("Please type a number!");

        let chosen_host = map_of_tunnels.get(&user_choice).expect("Key not found");

        println!("You choose: {}, Host: {}", user_choice, chosen_host);

        let exit_status = Exec::cmd("ssh").arg("-f").arg("-N").arg(&chosen_host).join().unwrap();

        // if let ExitStatus::Exited(exi) = exit_status {
        //     println!("Exited with {}", exi);
        // }
        match exit_status {
            ExitStatus::Exited(exi) => println!("Exited with {}", exi),
            ExitStatus::Signaled(sig) => println!("Signaled with {}", sig),
            ExitStatus::Other(oth) => println!("Other with {}", oth),
            ExitStatus::Undetermined => println!("Undetermined"),
        }
        // println!("{}", exit_status);

        TunnelList::run();

        // read landbay_tunnels file and display options to user
        
        println!("Reached End Gracefully");
    }
}