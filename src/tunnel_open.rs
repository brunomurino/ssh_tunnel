use clap::{App, clap_app, ArgMatches};
use std::path::Path;
// use std::env;
use std::{fs, io, io::{Write}, collections::{BTreeMap, HashMap}};
use itertools::Itertools;
use subprocess::{Exec, ExitStatus};
use dirs::home_dir;

use crate::tunnel_list::TunnelList;

pub struct TunnelOpen;
impl TunnelOpen {

    pub fn app() -> App<'static>  {        
        let core_app = clap_app!(open =>
            (visible_alias: "o")
            (version: "0.1")
            (author: "Bruno Murino <bfsmurino@gmail.com>")
            (about: "Opens pre-defined SSH tunnels")
            (@arg debug: -d --debug "Print information verbosely")
        );
        let app = core_app;
        app
    }

    pub fn run_from_matches(matches: &ArgMatches) {
        let debug = matches.is_present("debug");
        if debug {println!("Tunnel Open matches: {:?}", matches);}
        TunnelOpen::run(debug)
    }

    fn get_lines_with_include(lines_from_file: &Vec<String>) -> Vec<String> {
        lines_from_file
            .iter()
            .filter_map(|line| {
                if line.contains("Include ") {
                    Some(line.replace("Include ",""))
                } else { None }
            })
            .collect::<Vec<_>>()
    }

    fn get_lines_with_host<'a>(lines_from_file: &'a Vec<String>) -> HashMap<String, HashMap<String,String>> {

        let mut current_host = String::from("");
        let mut all_hosts_data = HashMap::new();

        for lin in lines_from_file {
            if lin.contains("Host ") & !lin.starts_with("Host *") {
                current_host = lin.split_whitespace().nth(1).unwrap().to_string();
                let current_host_data = HashMap::new();
                all_hosts_data.insert(current_host.clone(), current_host_data);
            }
            if lin == &"" {
                current_host = String::from("");
            }
            if &current_host != &String::from("") {
                let host_data_name = lin.trim().split_whitespace().nth(0).unwrap().to_string();
                let foo = lin.replace(&host_data_name,"").to_string();
                let host_data_content = foo.trim().to_string();
                all_hosts_data.get_mut(&current_host).unwrap().insert(host_data_name, host_data_content);
                // println!("{} - {} ---- {}", current_host, host_data_name, host_data_content);
            }
        }
        
        all_hosts_data.retain(|_, v| v.contains_key("LocalForward"));
        
        // println!("{:#?}", all_hosts_data);

        all_hosts_data
    }

    fn parse_file_lines(filepath: &Path) -> (HashMap<String, HashMap<String,String>>,Vec<String>) {
        let data = fs::read_to_string(filepath).expect("Unable to read file");
        let lines_from_file: Vec<String> = data.lines().clone().map(str::to_string).collect::<Vec<_>>();
        
        let lines_with_host = Self::get_lines_with_host(&lines_from_file);
        let lines_with_include = Self::get_lines_with_include(&lines_from_file);
        (
            lines_with_host,
            lines_with_include,
        )
    }

    fn process_file(filepath: &Path) -> HashMap<String, HashMap<String,String>> {

        // println!("Processing {}", filepath.to_str().unwrap()); // only on debug

        let str_filepath = &filepath.to_str().unwrap().to_string();

        let mut final_hosts = HashMap::new();

        let (mut hosts, includes) = Self::parse_file_lines(filepath);
        
        for (key, _) in hosts.clone().into_iter() {
            hosts.get_mut(&key).unwrap().insert("source_file".to_string(), str_filepath.to_string());
        }
        
        final_hosts.extend(hosts);

        for file_to_look in includes {
            let temp_map_of_tunnels_from_file = Self::process_file(&Path::new(&file_to_look));
            final_hosts.extend(temp_map_of_tunnels_from_file);
        }

        final_hosts
    }

    fn print_tunnel_list_to_user(btree_of_hosts: &BTreeMap<usize, &HashMap<String, String>>) {
        // println!("{:#?}", btree_of_hosts);
        println!("List of SSH Tunnels configured:\n");
        for (i, tunnel) in btree_of_hosts {
            println!("  {}) {}", i, tunnel.get("Host").unwrap());
        }
    }

    pub fn run(debug: bool) {
        if debug {println!("DEBUG is {}", debug);}
        let home = home_dir().unwrap();
        let filepath = home.join(".ssh/config");

        let list_of_hosts = Self::process_file(&filepath);
        // println!("{:#?}", list_of_hosts);
        let mut btree_of_hosts = BTreeMap::new();

        for (i, host) in list_of_hosts.keys().sorted().enumerate() {
            btree_of_hosts.insert(i+1, list_of_hosts.get(host).unwrap());
        }

        Self::print_tunnel_list_to_user(&btree_of_hosts);

        print!("\nSelect which tunnel to open: ");
        io::stdout().flush().unwrap();
        let mut user_choice = String::new();
        io::stdin()
            .read_line(&mut user_choice)
            .expect("Failed to read line");
        let user_choice: usize = user_choice.trim().parse().expect("Please type a number!");

        let chosen_host = btree_of_hosts.get(&user_choice).expect("Key not found").get("Host").unwrap();

        println!("You choose: {}, Host: {}", user_choice, chosen_host);

        let exit_status = Exec::cmd("ssh").arg("-f").arg("-N").arg(&chosen_host).join().unwrap();

        match exit_status {
            ExitStatus::Exited(exi) => println!("Exited with {}", exi),
            ExitStatus::Signaled(sig) => println!("Signaled with {}", sig),
            ExitStatus::Other(oth) => println!("Other with {}", oth),
            ExitStatus::Undetermined => println!("Undetermined"),
        }

        TunnelList::run();
        
        // println!("\nReached End Gracefully");  // only on debug
    }
}