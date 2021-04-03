use clap::{App, clap_app, ArgMatches};
use std::path::Path;
// use std::env;
use std::{fs, io, io::{Write}, collections::BTreeMap};
use subprocess::{Exec, ExitStatus};
use dirs::home_dir;
use crate::Tunnel;

use crate::tunnel_list::TunnelList;

pub struct TunnelOpen;
impl TunnelOpen {

    pub fn app() -> App<'static>  {        
        let core_app = clap_app!(open =>
            (visible_alias: "o")
            (version: "0.1")
            (author: "Bruno Murino <bfsmurino@gmail.com>")
            (about: "Opens pre-defined SSH tunnels")
        );
        let app = core_app;
        app
    }

    pub fn run_from_matches(matches: &ArgMatches) {
        println!("Tunnel Open matches: {:?}", matches);
        TunnelOpen::run()
    }

    fn get_lines_with_include(lines_from_file: &Vec<&str>) -> Vec<String> {
        lines_from_file
            .iter()
            .filter_map(|line| {
                if line.contains("Include ") {
                    Some(line.replace("Include ",""))
                } else { None }
            })
            .collect::<Vec<_>>()
    }

    fn get_lines_with_host(lines_from_file: &Vec<&str>) -> Vec<String> {
        lines_from_file
            .iter()
            .filter_map(|line| {
                if line.contains("Host ") & !line.starts_with("Host *") {
                    Some(line.replace("Host ",""))
                } else { None }
            })
            .collect::<Vec<_>>()
    }

    fn parse_file_lines(filepath: &Path) -> (Vec<String>,Vec<String>) {
        let data = fs::read_to_string(filepath).expect("Unable to read file");
        let lines_from_file: Vec<&str> = data.lines().clone().collect::<Vec<_>>();
        (
            Self::get_lines_with_host(&lines_from_file),
            Self::get_lines_with_include(&lines_from_file),
        )
    }

    fn process_file(filepath: &Path) -> Vec<Tunnel> {

        println!("Processing {}", filepath.to_str().unwrap());

        let str_filepath = &filepath.to_str().unwrap().to_string();

        let mut hash_of_hosts: Vec<Tunnel> = Vec::new();

        let (hosts, includes) = Self::parse_file_lines(filepath);

        for host in hosts.iter() {
            hash_of_hosts.push(Tunnel{host_name: host.to_string(), source_file: str_filepath.to_string()})
        }

        for file_to_look in includes {
            let mut temp_map_of_tunnels_from_file = Self::process_file(&Path::new(&file_to_look));
            hash_of_hosts.append(&mut temp_map_of_tunnels_from_file);
        }
        hash_of_hosts
    }

    fn print_tunnel_list_to_user(btree_of_hosts: &BTreeMap<usize, &Tunnel>) {
        // println!("{:#?}", btree_of_hosts);
        println!("List of SSH Tunnels configured:\n");
        for (i, tunnel) in btree_of_hosts {
            println!("  {}) {} ({})", i, tunnel.host_name, tunnel.source_file);
        }
    }

    pub fn run() {
        let home = home_dir().unwrap();
        let filepath = home.join(".ssh/config");

        let list_of_hosts = Self::process_file(&filepath);
        let mut btree_of_hosts = BTreeMap::new();
        for (i, host) in list_of_hosts.iter().enumerate() {
            btree_of_hosts.insert(i+1, host);
        }

        Self::print_tunnel_list_to_user(&btree_of_hosts);

        print!("\nSelect which tunnel to open: ");
        io::stdout().flush().unwrap();
        let mut user_choice = String::new();
        io::stdin()
            .read_line(&mut user_choice)
            .expect("Failed to read line");
        let user_choice: usize = user_choice.trim().parse().expect("Please type a number!");

        let chosen_host = btree_of_hosts.get(&user_choice).expect("Key not found");

        println!("You choose: {}, Host: {}", user_choice, chosen_host.host_name);

        let exit_status = Exec::cmd("ssh").arg("-f").arg("-N").arg(&chosen_host.host_name).join().unwrap();

        match exit_status {
            ExitStatus::Exited(exi) => println!("Exited with {}", exi),
            ExitStatus::Signaled(sig) => println!("Signaled with {}", sig),
            ExitStatus::Other(oth) => println!("Other with {}", oth),
            ExitStatus::Undetermined => println!("Undetermined"),
        }

        TunnelList::run();
        
        println!("\nReached End Gracefully");
    }
}