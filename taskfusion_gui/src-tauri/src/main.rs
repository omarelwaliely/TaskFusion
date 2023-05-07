use serde::Serialize;
use tauri::{Manager, Window};
use sysinfo::{System, SystemExt, CpuExt};
//use std::fs::{self, File};
// use serde_json::Value;
// use std::io::Read;
use procfs::process::Process;
use procfs::process::all_processes;
//include all the necessary libraries
use colored::*;
use std::thread::sleep;
use std::time::{Duration, Instant};
use prettytable::{Table, row};
use std::io::{stdout, Write};
use termion::clear;
use nix::unistd::Pid;
use nix::sys::signal::{kill, Signal};
use nix::sys::signal::{SIGTERM};
use std::time::SystemTime;
use chrono::prelude::*;
use chrono::Local;
use nix::sys::signal::SIGSTOP;
use nix::sys::signal::SIGCONT;
use std::fs::File;
use regex::Regex;


// Prevents additional console window on Windows in release, DO NOT REMOVE!!
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
#[tauri::command]
fn listprocesses() -> String {
    let mut timeofprocesses=0.0;
    for process in all_processes().unwrap() {
        if let Ok(stat) = process.unwrap().stat() {
            timeofprocesses=timeofprocesses+((stat.utime+stat.stime) as f64);
        }
    }
    let mut json = String::from("[");
    for process in all_processes().unwrap() {
        if let Ok(stat) = process.unwrap().stat() {
            let process = psutil::process::Process::new(stat.pid as u32).unwrap();
            let memory_usage = process.memory_info().unwrap().rss();
            let memory_usage_mb = memory_usage as f64 / 1024.0 / 1024.0;
            let jsonprocess = format!(
                r#"{{"pid":{},"state":"{}","ppid":{},"pgrp":{},"session":{},"tty_nr":{},"tpgid":{},"flags":{},"utime":{},"stime":{},"priority":{},"nice":{},"num_threads":{},"starttime":{},"vsize":{},"cmd":"{}","memory_usage_mb":{:.2},"cpu_usage":{:.2}}}"#,
                stat.pid, stat.state, stat.ppid,stat.pgrp,stat.session,stat.tty_nr,stat.tpgid,stat.flags,stat.utime,stat.stime,stat.priority,stat.nice,stat.num_threads,stat.starttime,stat.vsize, stat.comm,memory_usage_mb,(((stat.utime+stat.stime) as f64)/((timeofprocesses))*100.0)
            );
            json.push_str(&jsonprocess);
            json.push_str(",");
        }
    }
    json.pop();
    json.push_str("]");
    println!("{}", json);
    json
}

#[tauri::command]
fn filterByPid(pid: i32) -> String {
    let mut timeofprocesses=0.0;
    for process in all_processes().unwrap() {
        if let Ok(stat) = process.unwrap().stat() {
            timeofprocesses=timeofprocesses+((stat.utime+stat.stime) as f64);
        }
    }
    let mut json = String::from("[");
    for process in all_processes().unwrap() {
        if let Ok(stat) = process.unwrap().stat() {
            if stat.pid==pid 
            {
                let process = psutil::process::Process::new(stat.pid as u32).unwrap();
                let memory_usage = process.memory_info().unwrap().rss();
                let memory_usage_mb = memory_usage as f64 / 1024.0 / 1024.0;
                let jsonprocess = format!(
                    r#"{{"pid":{},"state":"{}","ppid":{},"pgrp":{},"session":{},"tty_nr":{},"tpgid":{},"flags":{},"utime":{},"stime":{},"priority":{},"nice":{},"num_threads":{},"starttime":{},"vsize":{},"cmd":"{}","memory_usage_mb":{:.2},"cpu_usage":{:.2}}}"#,
                    stat.pid, stat.state, stat.ppid,stat.pgrp,stat.session,stat.tty_nr,stat.tpgid,stat.flags,stat.utime,stat.stime,stat.priority,stat.nice,stat.num_threads,stat.starttime,stat.vsize, stat.comm,memory_usage_mb,(((stat.utime+stat.stime) as f64)/((timeofprocesses))*100.0)
                );
                json.push_str(&jsonprocess);
                json.push_str(",");
            }
        }
    }
    json.pop();
    json.push_str("]");
    println!("{}", json);
    json
}


#[tauri::command]
fn filterByState(state: char) -> String {
    println!("over here\n");
    let mut timeofprocesses=0.0;
    for process in all_processes().unwrap() {
        if let Ok(stat) = process.unwrap().stat() {
            timeofprocesses=timeofprocesses+((stat.utime+stat.stime) as f64);
        }
    }
    let mut json = String::from("[");
    for process in all_processes().unwrap() {
        if let Ok(stat) = process.unwrap().stat() {
            if stat.state==state 
            {
                let process = psutil::process::Process::new(stat.pid as u32).unwrap();
                let memory_usage = process.memory_info().unwrap().rss();
                let memory_usage_mb = memory_usage as f64 / 1024.0 / 1024.0;
                let jsonprocess = format!(
                    r#"{{"pid":{},"state":"{}","ppid":{},"pgrp":{},"session":{},"tty_nr":{},"tpgid":{},"flags":{},"utime":{},"stime":{},"priority":{},"nice":{},"num_threads":{},"starttime":{},"vsize":{},"cmd":"{}","memory_usage_mb":{:.2},"cpu_usage":{:.2}}}"#,
                    stat.pid, stat.state, stat.ppid,stat.pgrp,stat.session,stat.tty_nr,stat.tpgid,stat.flags,stat.utime,stat.stime,stat.priority,stat.nice,stat.num_threads,stat.starttime,stat.vsize, stat.comm,memory_usage_mb,(((stat.utime+stat.stime) as f64)/((timeofprocesses))*100.0)
                );
                json.push_str(&jsonprocess);
                json.push_str(",");
            }
        }
    }
    json.pop();
    json.push_str("]");
    println!("{}", json);
    json
}


#[tauri::command]
fn filterby_cmd(cmdyo: String) -> String {
       let mut timeofprocesses=0.0;
       let regex=Regex::new(cmdyo.as_str()).unwrap();
    for process in all_processes().unwrap() {
        if let Ok(stat) = process.unwrap().stat() {
            timeofprocesses=timeofprocesses+((stat.utime+stat.stime) as f64);
        }
    }
    
    let mut json = String::from("[");
    for process in all_processes().unwrap() {
        if let Ok(stat) = process.unwrap().stat() {
            if regex.is_match(&stat.comm) 
            {
                let process = psutil::process::Process::new(stat.pid as u32).unwrap();
                let memory_usage = process.memory_info().unwrap().rss();
                let memory_usage_mb = memory_usage as f64 / 1024.0 / 1024.0;
                let jsonprocess = format!(
                    r#"{{"pid":{},"state":"{}","ppid":{},"pgrp":{},"session":{},"tty_nr":{},"tpgid":{},"flags":{},"utime":{},"stime":{},"priority":{},"nice":{},"num_threads":{},"starttime":{},"vsize":{},"cmd":"{}","memory_usage_mb":{:.2},"cpu_usage":{:.2}}}"#,
                    stat.pid, stat.state, stat.ppid,stat.pgrp,stat.session,stat.tty_nr,stat.tpgid,stat.flags,stat.utime,stat.stime,stat.priority,stat.nice,stat.num_threads,stat.starttime,stat.vsize, stat.comm,memory_usage_mb,(((stat.utime+stat.stime) as f64)/((timeofprocesses))*100.0)
                );
                json.push_str(&jsonprocess);
                json.push_str(",");
            }
        }
    }
    json.pop();
    json.push_str("]");
    println!("{}", json);
    json
}
// #[tauri::command]
// fn filterby_cmd(cmd: String) -> String {
//     let mut timeofprocesses=0.0;
//     for process in all_processes().unwrap() {
//         if let Ok(stat) = process.unwrap().stat() {
//             timeofprocesses=timeofprocesses+((stat.utime+stat.stime) as f64);
//         }
//     }
//     let mut json = String::from("[");
//     for process in all_processes().unwrap() {
//         if let Ok(stat) = process.unwrap().stat() {
//             if stat.comm==cmd 
//             {
//                 let process = psutil::process::Process::new(stat.pid as u32).unwrap();
//                 let memory_usage = process.memory_info().unwrap().rss();
//                 let memory_usage_mb = memory_usage as f64 / 1024.0 / 1024.0;
//                 let jsonprocess = format!(
//                     r#"{{"pid":{},"state":"{}","ppid":{},"pgrp":{},"session":{},"tty_nr":{},"tpgid":{},"flags":{},"utime":{},"stime":{},"priority":{},"nice":{},"num_threads":{},"starttime":{},"vsize":{},"cmd":"{}","memory_usage_mb":{:.2},"cpu_usage":{:.2}}}"#,
//                     stat.pid, stat.state, stat.ppid,stat.pgrp,stat.session,stat.tty_nr,stat.tpgid,stat.flags,stat.utime,stat.stime,stat.priority,stat.nice,stat.num_threads,stat.starttime,stat.vsize, stat.comm,memory_usage_mb,(((stat.utime+stat.stime) as f64)/((timeofprocesses))*100.0)
//                 );
//                 json.push_str(&jsonprocess);
//                 json.push_str(",");
//             }
//         }
//     }
//     json.pop();
//     json.push_str("]");
//     println!("{}", json);
//     json
// }

#[tauri::command]
fn filterByPpid(ppid: i32)->String{
    let mut timeofprocesses=0.0;
    let mut json = String::from("[");
    for process in all_processes().unwrap() {
        if let Ok(stat) = process.unwrap().stat() {
            timeofprocesses=timeofprocesses+((stat.utime+stat.stime) as f64);
        }
    }
    for process in all_processes().unwrap() {
        if let Ok(stat) = process.unwrap().stat() {
            if stat.ppid==ppid 
            {
                let process = psutil::process::Process::new(stat.pid as u32).unwrap();
                let memory_usage = process.memory_info().unwrap().rss();
                let memory_usage_mb = memory_usage as f64 / 1024.0 / 1024.0;
                let jsonprocess=format!(
                    r#"{{"pid":{0},"state":"{1}","ppid":{2},"pgrp":{3},"session":{4},"tty_nr":{5},"tpgid":{6},"flags":{7},"utime":{8},"stime":{9},"priority":{10},"nice":{11},"num_threads":{12},"starttime":{13},"vsize":{14},"comm":"{15}","memory_usage_mb":{16},"cpu_usage":{17}}}"#,
                    stat.pid, stat.state, stat.ppid,stat.pgrp,stat.session,stat.tty_nr,stat.tpgid,stat.flags,stat.utime,stat.stime,stat.priority,stat.nice,stat.num_threads,stat.starttime,stat.vsize, stat.comm,memory_usage_mb,(((stat.utime+stat.stime) as f64)/((timeofprocesses))*100.0)
                );
                json.push_str(&jsonprocess);
                json.push_str(",");
            } 
        }
    }
    json.push_str("]");
    json
}


#[tauri::command]
fn filterByGID(gid: i32)->String{
    let mut timeofprocesses=0.0;
    let mut json = String::from("[");
    for process in all_processes().unwrap() {
        if let Ok(stat) = process.unwrap().stat() {
            timeofprocesses=timeofprocesses+((stat.utime+stat.stime) as f64);
        }
    }
    for process in all_processes().unwrap() {
        if let Ok(stat) = process.unwrap().stat() {
            if stat.pgrp==gid 
            {
                let process = psutil::process::Process::new(stat.pid as u32).unwrap();
                let memory_usage = process.memory_info().unwrap().rss();
                let memory_usage_mb = memory_usage as f64 / 1024.0 / 1024.0;
                let jsonprocess=format!(
                    r#"{{"pid":{0},"state":"{1}","ppid":{2},"pgrp":{3},"session":{4},"tty_nr":{5},"tpgid":{6},"flags":{7},"utime":{8},"stime":{9},"priority":{10},"nice":{11},"num_threads":{12},"starttime":{13},"vsize":{14},"comm":"{15}","memory_usage_mb":{16},"cpu_usage":{17}}}"#,
                    stat.pid, stat.state, stat.ppid,stat.pgrp,stat.session,stat.tty_nr,stat.tpgid,stat.flags,stat.utime,stat.stime,stat.priority,stat.nice,stat.num_threads,stat.starttime,stat.vsize, stat.comm,memory_usage_mb,(((stat.utime+stat.stime) as f64)/((timeofprocesses))*100.0)
                );
                json.push_str(&jsonprocess);
                json.push_str(",");
            } 
        }
    }
    json.push_str("]");
    json
}

//create a tauri command get_system_info that returns a JSON string
#[tauri::command]
fn get_system_info() -> String {
  let system = System::new();
  let system_name = system.name().unwrap_or_default();
  let kernel_version = system.kernel_version().unwrap_or_default();
  let os_version = system.os_version().unwrap_or_default();
  let host_name = system.host_name().unwrap_or_default();
  let json = format!(
      r#"{{"hostname":"{}","system_name":"{}","kernel_version":"{}","os_version":"{}"}}"#,
      host_name, system_name, kernel_version, os_version
  );
 println!("{}", json);
  json
}

#[tauri::command]
fn get_system_usage() -> String {
    let mut sys = System::new();
    sys.refresh_all();
    let total_mem = sys.total_memory();
    let used_mem = sys.used_memory();
    let mem_percent = (used_mem as f32 / total_mem as f32) * 100.0;
    let total_swap = sys.total_swap();
    let free_swap = sys.free_swap();
    let used_swap = total_swap - free_swap;
    let swap_percent = used_swap as f32 / total_swap as f32 * 100.0;
    let mut cpu_use = 0.0;
    let num_cpus = sys.cpus().len() as f64;
    let load_avg = sys.load_average();
    let cpu_percent = (load_avg.one / num_cpus)*100.0;
    let json = format!(
        r#"{{"mem_percent":{:.1},"swap_percent":{:.1},"cpu_percent":{:.1}}}"#,
        mem_percent, swap_percent, cpu_percent
    );
    json
}

#[tauri::command]
fn sortby_session() -> String {
    let mut timeofprocesses=0.0;
    for process in all_processes().unwrap() {
        if let Ok(stat) = process.unwrap().stat() {
            timeofprocesses=timeofprocesses+((stat.utime+stat.stime) as f64);
        }
    }
    
    let processes = all_processes().unwrap();
    let mut processes_vector: Vec<Process>=Vec::new(); 
    
    for process in processes
    {
        match process {
            Ok(p) => {
                if let Ok(stat) = p.stat() {
                    processes_vector.push(p);
                }
            }
            Err(e) => {
                println!("Error getting process information: {}", e);
            }
        }
    }
    processes_vector.sort_by_key(|p| {
        if let Ok(stat) = p.stat() {   
            return stat.session;
        }
        else
        {
            return 0;
        }
       
    });
    let mut json = String::from("[");
    for process in &processes_vector {
        if let Ok(stat) = process.stat() {
            let process = psutil::process::Process::new(stat.pid as u32).unwrap();
            let memory_usage = process.memory_info().unwrap().rss();
            let memory_usage_mb = memory_usage as f64 / 1024.0 / 1024.0;
            let jsonprocess = format!(
                r#"{{"pid":{},"state":"{}","ppid":{},"pgrp":{},"session":{},"tty_nr":{},"tpgid":{},"flags":{},"utime":{},"stime":{},"priority":{},"nice":{},"num_threads":{},"starttime":{},"vsize":{},"cmd":"{}","memory_usage_mb":{:.2},"cpu_usage":{:.2}}}"#,
                stat.pid, stat.state, stat.ppid,stat.pgrp,stat.session,stat.tty_nr,stat.tpgid,stat.flags,stat.utime,stat.stime,stat.priority,stat.nice,stat.num_threads,stat.starttime,stat.vsize, stat.comm,memory_usage_mb,(((stat.utime+stat.stime) as f64)/((timeofprocesses))*100.0)
            );
            json.push_str(&jsonprocess);
            json.push_str(",");
        }
    }
    json.pop();
    json.push_str("]");
    println!("{}", json);
    return json;
}


#[tauri::command]
fn sortby_pid() -> String {
    let mut timeofprocesses=0.0;
    for process in all_processes().unwrap() {
        if let Ok(stat) = process.unwrap().stat() {
            timeofprocesses=timeofprocesses+((stat.utime+stat.stime) as f64);
        }
    }
    
    let processes = all_processes().unwrap();
    let mut processes_vector: Vec<Process>=Vec::new(); 
    
    for process in processes
    {
        match process {
            Ok(p) => {
                if let Ok(stat) = p.stat() {
                    processes_vector.push(p);
                }
            }
            Err(e) => {
                println!("Error getting process information: {}", e);
            }
        }
    }
    processes_vector.sort_by_key(|p| p.pid());
    let mut json = String::from("[");
    for process in &processes_vector {
        if let Ok(stat) = process.stat() {
            let process = psutil::process::Process::new(stat.pid as u32).unwrap();
            let memory_usage = process.memory_info().unwrap().rss();
            let memory_usage_mb = memory_usage as f64 / 1024.0 / 1024.0;
            let jsonprocess = format!(
                r#"{{"pid":{},"state":"{}","ppid":{},"pgrp":{},"session":{},"tty_nr":{},"tpgid":{},"flags":{},"utime":{},"stime":{},"priority":{},"nice":{},"num_threads":{},"starttime":{},"vsize":{},"cmd":"{}","memory_usage_mb":{:.2},"cpu_usage":{:.2}}}"#,
                stat.pid, stat.state, stat.ppid,stat.pgrp,stat.session,stat.tty_nr,stat.tpgid,stat.flags,stat.utime,stat.stime,stat.priority,stat.nice,stat.num_threads,stat.starttime,stat.vsize, stat.comm,memory_usage_mb,(((stat.utime+stat.stime) as f64)/((timeofprocesses))*100.0)
            );
            json.push_str(&jsonprocess);
            json.push_str(",");
        }
    }
    json.pop();
    json.push_str("]");
    println!("{}", json);
    json
}

#[tauri::command]
fn sortby_priority() -> String{
    let mut timeofprocesses=0.0;
    for process in all_processes().unwrap() {
        if let Ok(stat) = process.unwrap().stat() {
            timeofprocesses=timeofprocesses+((stat.utime+stat.stime) as f64);
        }
    }
    let processes = all_processes().unwrap();
    let mut processes_vector: Vec<Process>=Vec::new(); 
    
    for process in processes
    {
        match process {
            Ok(p) => {
                if let Ok(stat) = p.stat() {
                    processes_vector.push(p);
                }
            }
            Err(e) => {
                println!("Error getting process information: {}", e);
            }
        }
    }
    processes_vector.sort_by_key(|p| {
        if let Ok(stat) = p.stat() {
            return stat.priority;
        }
        else
        {
            return 0;
        }
       
    });
    let mut json = String::from("[");
    for process in &processes_vector {
        if let Ok(stat) = process.stat() {
            let process = psutil::process::Process::new(stat.pid as u32).unwrap();
            let memory_usage = process.memory_info().unwrap().rss();
            let memory_usage_mb = memory_usage as f64 / 1024.0 / 1024.0;
            let jsonprocess = format!(
                r#"{{"pid":{},"state":"{}","ppid":{},"pgrp":{},"session":{},"tty_nr":{},"tpgid":{},"flags":{},"utime":{},"stime":{},"priority":{},"nice":{},"num_threads":{},"starttime":{},"vsize":{},"cmd":"{}","memory_usage_mb":{:.2},"cpu_usage":{:.2}}}"#,
                stat.pid, stat.state, stat.ppid,stat.pgrp,stat.session,stat.tty_nr,stat.tpgid,stat.flags,stat.utime,stat.stime,stat.priority,stat.nice,stat.num_threads,stat.starttime,stat.vsize, stat.comm,memory_usage_mb,(((stat.utime+stat.stime) as f64)/((timeofprocesses))*100.0)
            );
            json.push_str(&jsonprocess);
            json.push_str(",");
        }
    }
    json.pop();
    json.push_str("]");
    println!("{}", json);
    json
}

#[tauri::command]
fn sortby_parent() ->String
{
    let mut timeofprocesses=0.0;
    for process in all_processes().unwrap() {
        if let Ok(stat) = process.unwrap().stat() {
            timeofprocesses=timeofprocesses+((stat.utime+stat.stime) as f64);
        }
    }
   
    let processes = all_processes().unwrap();
    let mut processes_vector: Vec<Process>=Vec::new(); 
    
    for process in processes
    {
        match process {
            Ok(p) => {
                if let Ok(stat) = p.stat() {
                    processes_vector.push(p);
                }
            }
            Err(e) => {
                println!("Error getting process information: {}", e);
            }
        }
    }
    processes_vector.sort_by_key(|p| {
        if let Ok(stat) = p.stat() {
            return stat.ppid;
        }
        else
        {
            return 0;
        }
       
    });
    let mut json = String::from("[");
    for process in &processes_vector {
        if let Ok(stat) = process.stat() {
            let process = psutil::process::Process::new(stat.pid as u32).unwrap();
            let memory_usage = process.memory_info().unwrap().rss();
            let memory_usage_mb = memory_usage as f64 / 1024.0 / 1024.0;
            let jsonprocess = format!(
                r#"{{"pid":{},"state":"{}","ppid":{},"pgrp":{},"session":{},"tty_nr":{},"tpgid":{},"flags":{},"utime":{},"stime":{},"priority":{},"nice":{},"num_threads":{},"starttime":{},"vsize":{},"cmd":"{}","memory_usage_mb":{:.2},"cpu_usage":{:.2}}}"#,
                stat.pid, stat.state, stat.ppid,stat.pgrp,stat.session,stat.tty_nr,stat.tpgid,stat.flags,stat.utime,stat.stime,stat.priority,stat.nice,stat.num_threads,stat.starttime,stat.vsize, stat.comm,memory_usage_mb,(((stat.utime+stat.stime) as f64)/((timeofprocesses))*100.0)
            );
            json.push_str(&jsonprocess);
            json.push_str(",");
        }
    }
    json.pop();
    json.push_str("]");
    println!("{}", json);
    json
}

#[tauri::command]
fn sortby_group()-> String{
    let mut timeofprocesses=0.0;
    for process in all_processes().unwrap() {
        if let Ok(stat) = process.unwrap().stat() {
            timeofprocesses=timeofprocesses+((stat.utime+stat.stime) as f64);
        }
    }
    
    let processes = all_processes().unwrap();
    let mut processes_vector: Vec<Process>=Vec::new(); 
    
    for process in processes
    {
        match process {
            Ok(p) => {
                if let Ok(stat) = p.stat() {
                    processes_vector.push(p);
                }
            }
            Err(e) => {
                println!("Error getting process information: {}", e);
            }
        }
    }
    processes_vector.sort_by_key(|p| {
        if let Ok(stat) = p.stat() {
            return stat.pgrp;
        }
        else
        {
            return 0;
        }
       
    });
    let mut json = String::from("[");
    for process in &processes_vector {
        if let Ok(stat) = process.stat() {
            let process = psutil::process::Process::new(stat.pid as u32).unwrap();
            let memory_usage = process.memory_info().unwrap().rss();
            let memory_usage_mb = memory_usage as f64 / 1024.0 / 1024.0;
            let jsonprocess = format!(
                r#"{{"pid":{},"state":"{}","ppid":{},"pgrp":{},"session":{},"tty_nr":{},"tpgid":{},"flags":{},"utime":{},"stime":{},"priority":{},"nice":{},"num_threads":{},"starttime":{},"vsize":{},"cmd":"{}","memory_usage_mb":{:.2},"cpu_usage":{:.2}}}"#,
                stat.pid, stat.state, stat.ppid,stat.pgrp,stat.session,stat.tty_nr,stat.tpgid,stat.flags,stat.utime,stat.stime,stat.priority,stat.nice,stat.num_threads,stat.starttime,stat.vsize, stat.comm,memory_usage_mb,(((stat.utime+stat.stime) as f64)/((timeofprocesses))*100.0)
            );
            json.push_str(&jsonprocess);
            json.push_str(",");
        }
    }
    json.pop();
    json.push_str("]");
    println!("{}", json);
    json
}
#[tauri::command]
fn log_to_terminal(message: String) {
  println!("{}", message);
}
#[tauri::command]
fn process_kill(pid: i32){
    println!("yyeyeyeyeye\n\n\n");
    let pid = Pid::from_raw(pid); 

    match kill(pid, SIGTERM) {
        Ok(_) => println!("Process killed"),
        Err(e) => eprintln!("Error killing process: {}", e),
    }
}

#[tauri::command]
fn killbythreshold(threshold: f64){
    let mut timeofprocesses=0.0;
    for process in all_processes().unwrap() {
        if let Ok(stat) = process.unwrap().stat() {
            timeofprocesses=timeofprocesses+((stat.utime+stat.stime) as f64);
        }
    }
    for process in all_processes().unwrap() {
        
        if let Ok(stat) = process.unwrap().stat() {
            if((((stat.utime+stat.stime) as f64)/((timeofprocesses))*100.0)>50.0)
            {
                let pid = Pid::from_raw(stat.pid); 

                match kill(pid, SIGTERM) {
                    Ok(_) => println!("Process killed"),
                    Err(e) => eprintln!("Error killing process: {}", e),
                }
            }
            
        }
    }
}
#[tauri::command]
fn changepriority(pid :String , priority: String){
    unsafe 
    {
        let pid_i = pid.parse::<i32>().unwrap();
        let pr_i = priority.parse::<i32>().unwrap();
        let result = libc::setpriority(libc::PRIO_PROCESS, pid_i as libc::id_t, pr_i);
        if result != 0 {
            println!("Error changing process priority: {}", std::io::Error::last_os_error());
        } else {
            println!("Process priority changed successfully");
        }
    }
}
#[tauri::command]
fn takesnapshot(){
    let local: DateTime<Local> = Local::now();
    let formatted_date_time = local.format("%H:%M %d/%m/%Y").to_string();
    let mut file = File::create("snapshot.txt").unwrap();
    let mut timeofprocesses=0.0;
    for process in all_processes().unwrap() {
        if let Ok(stat) = process.unwrap().stat() {
            timeofprocesses=timeofprocesses+((stat.utime+stat.stime) as f64);
        }
    }
    file.write_all(format!("Time: {}\n",formatted_date_time).as_bytes()).unwrap();
    for process in all_processes().unwrap() {
        if let Ok(stat) = process.unwrap().stat() {
            let memory_usage_mb = (stat.rss as f64) / 1024.0;
            let cpu_usage=((stat.utime+stat.stime) as f64/timeofprocesses)*100.0;
            let process_info=format!("PID: {}\t PPID: {}\t Name: {}\t State: {}\t TTYGID: {}\t Flags: {}\t Utime: {}\t Stime: {}\tPriority: {}\tNice: {}\tThreads: {}\tStart: {}\tVmemory: {}\tCMD: {}\tMemory: {:.2} MB\tCPU: {:.2}%\n",stat.pid,stat.ppid,stat.comm,stat.state,stat.tpgid,stat.flags,stat.utime,stat.stime,stat.priority,stat.nice,stat.num_threads,stat.starttime,stat.vsize,stat.comm,memory_usage_mb,cpu_usage);
            file.write_all(process_info.as_bytes()).unwrap();
        }
    }

}

#[tauri::command]
fn resume_process(id:i32)
{
    let pid = Pid::from_raw(id); 

    match kill(pid, SIGCONT) {
        Ok(_) => println!("Process resumed"),
        Err(e) => eprintln!("Error resuming process: {}", e),
    }
}

#[tauri::command]
fn pause_process(id: i32)
{
    let pid = Pid::from_raw(id); 

    match kill(pid, SIGSTOP) {
        Ok(_) => println!("Process paused"),
        Err(e) => eprintln!("Error pausing process: {}", e),
    }
}

#[tauri::command]
fn recursive_kill(id: i32)
{
    let mut children = Vec::new();
    for process in all_processes().unwrap() {
        if let Ok(stat) = process.unwrap().stat() {
            if(stat.ppid==id)
            {
                children.push(stat.pid);
            }
        }
    }
    for child in children
    {
        process_kill(child);
    }
    process_kill(id);
}
#[tauri::command]
fn recursive_pause(id: i32)
{
    let mut children = Vec::new();
    for process in all_processes().unwrap() {
        if let Ok(stat) = process.unwrap().stat() {
            if(stat.ppid==id)
            {
                children.push(stat.pid);
            }
        }
    }
    for child in children
    {
        pause_process(child);
    }
    pause_process(id);

}
#[tauri::command]
fn recursive_resume(id: i32)
{
    let mut children = Vec::new();
    for process in all_processes().unwrap() {
        if let Ok(stat) = process.unwrap().stat() {
            if(stat.ppid==id)
            {
                children.push(stat.pid);
            }
        }
    }
    for child in children
    {
        resume_process(child);
    }
    resume_process(id);

}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_system_info, get_system_usage, log_to_terminal, listprocesses, filterByPid, filterByState, sortby_pid, sortby_priority, sortby_parent, sortby_session, sortby_group, 
            process_kill, killbythreshold, changepriority, filterByPpid, filterByGID, takesnapshot, recursive_kill, resume_process, pause_process, recursive_pause, recursive_resume, filterby_cmd])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
