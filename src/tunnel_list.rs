
// use std::env;
use subprocess::{Exec, PopenError};
use clap::{App, clap_app, ArgMatches};
use std::collections::{HashMap};

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

    pub fn run_from_matches(_matches: &ArgMatches) -> HashMap<String, HashMap<String, String>> {
        // println!("Tunnel List matches: {:?}", matches);
        TunnelList::run()
    }

    pub fn run() -> HashMap<String, HashMap<String, String>> {

        let lines = Self::get_list_of_open_tunnels().unwrap();
        
        if lines.keys().len() > 0 {
            for (pid, proc) in &lines {
                println!("{}\t{}\t({})", pid, proc.get("name").unwrap(), proc.get("host_port").unwrap());
            }
        } else {
            println!("No SSH tunnels open");
        }

        // println!("Reached End Gracefully");  // only on debug

        lines
    }

    pub fn get_list_of_open_tunnels() -> Result<HashMap<String, HashMap<String, String>>, PopenError> {
        let mut open_tunnels = HashMap::new();

        // ps -eo pid,command | grep "ssh -f" | grep -v grep
        let out = {
            Exec::cmd("ps").arg("-eo").arg("pid,command")
            | Exec::cmd("grep").arg("ssh -f")
            | Exec::cmd("grep").arg("-v").arg("grep")
        }.capture()?.stdout_str();        
        let lines: Vec<String> = out.lines().map(str::to_string).collect();
        for lin in &lines {
            let mut proc_map = HashMap::new();
            let proc_pid = lin.split_whitespace().nth(0).unwrap().to_string();
            let proc_name = lin.split_whitespace().nth(4).unwrap().to_string();
            proc_map.insert("name".to_string(), proc_name);
            let proc_full = lin.to_string();
            proc_map.insert("full".to_string(), proc_full);

            open_tunnels.insert(proc_pid, proc_map);
        }


        // lsof -aPi4 | grep ssh | grep localhost
        let out2 = {
            Exec::cmd("lsof").arg("-aPi4")
            | Exec::cmd("grep").arg("ssh")
            | Exec::cmd("grep").arg("localhost")
        }.capture()?.stdout_str();
        let lines2: Vec<String> = out2.lines().map(str::to_string).collect();
        for proc in &lines2 {
            let proc_pid = proc.split_whitespace().nth(1).unwrap().to_string();
            let proc_host_port = proc.split_whitespace().nth(8).unwrap().to_string();
            open_tunnels.get_mut(&proc_pid).unwrap().insert("host_port".to_string(), proc_host_port);
        }

        // println!("{:#?}", open_tunnels);

        Ok(open_tunnels)
    }
}