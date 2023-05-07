use procfs::process::all_processes;
use std::fs;
use procfs::process::Process;
use colored::*;
use std::thread::sleep;
use std::time::{Duration, Instant};
use prettytable::{Table, row};
use std::io::{stdout, Write};
use termion::clear;
use nix::unistd::Pid;
use nix::sys::signal::{kill, Signal};
use nix::sys::signal::{SIGTERM};
extern crate libc;
use sysinfo::{ProcessExt, System, SystemExt};
use std::thread;
use std::fs::File;
use std::io::prelude::*;
extern crate cursive;
extern crate cursive_table_view;
extern crate rand;
use cursive::Cursive;
use cursive::views::{BoxedView, LinearLayout, ScrollView, TextView};
use cursive::theme::{Color, ColorStyle, PaletteColor, Theme};
use cursive::theme::Effect;
use cursive::CursiveExt;
use std::cmp::Ordering;
use cursive::align::HAlign;
use cursive::traits::*;
use cursive::views::{Dialog};
use rand::Rng;
use cursive_table_view::{TableView, TableViewItem};
use sysinfo::{NetworkExt, NetworksExt, CpuExt, UserExt};
use nix::sys::signal::SIGSTOP;
use nix::sys::signal::SIGCONT;
use cursive::direction::Orientation;
use std::time::UNIX_EPOCH;
use std::time::SystemTime;
use chrono::prelude::*;
use std::collections::HashMap;
use cursive_tree_view::{Placement, TreeView};
use cursive::views::Button;
use cursive::views::EditView;
use cursive::views::DummyView;
use std::cell::RefCell;
use std::rc::Rc;
use std::env;
use procfs::CpuInfo;
use cursive::traits::*;
use cursive::theme::*;
use std::path::Path;
use regex::Regex;
use std::process::Command;

fn display_system_info() -> String
{
    let mut sys = System::new_all();
    let mut result="".to_string();
    sys.refresh_all();

    
    let cpu_info = CpuInfo::new().unwrap();



    let computer_name = std::env::var("COMPUTERNAME").unwrap_or_else(|_| "Unknown".to_string());
    let user_name = std::env::var("USERNAME").unwrap_or_else(|_| "Unknown".to_string());

    let uptime = sys.uptime();
    let uptime_duration = Duration::from_secs(uptime as u64);
    let total_memory_gb = sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0;

    result+="Hardware Overview:\n";
    result+=format!("Manufacturer:             {:?}\n", sys.cpus()[0].brand()).as_str();
    result+=format!("CPU ID:             {:?}\n", sys.cpus()[0].vendor_id()).as_str();
    result+=format!("Total Number of Cores:             {:?}\n", sys.physical_core_count()).as_str();  // cpus().len()
    result+=format!("RAM Memory:             {:2 }GB\n", total_memory_gb).as_str();
    

    result+="Software Overview:\n";

    result+=format!("System Name:             {:?}\n", sys.name()).as_str();
    result+=format!("Operating System Version:       {:?}\n", sys.long_os_version()).as_str();
    result+=format!("Kernel Version:   {:?}\n", sys.kernel_version()).as_str();
    result+=format!("System Host Name:        {:?}\n", sys.host_name()).as_str();
    result+=format!("User Name:        {:?}\n", user_name).as_str(); 
    result+=format!("Time since last boot:        {:?}\n", uptime_duration ).as_str(); 

    return result;
}


fn display_resource_usage() ->String
{
    let mut result="".to_string();
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
    result=format!("Memory usage: {:.1}%, Swap usage: {:.1}%, CPU usage: {:.1}%\n", mem_percent, swap_percent, cpu_percent);
    std::thread::sleep(std::time::Duration::from_millis(500));
    
    return result;
}



#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum BasicColumn {
    PID,
    State,
    PPID,
    GID,
    Session,
    TTY,
    TTYGID,
    Flags,
    Utime,
    Stime,
    Priority,
    Nice,
    Threads,
    Start,
    Vmemory,
    CMD,
    Memory,
    CPU,
}

impl BasicColumn {
    fn as_str(&self) -> &str {
        match *self {
            BasicColumn::PID => "PID",
            BasicColumn::State => "State",
            BasicColumn::PPID => "PPID",
            BasicColumn::GID => "GID",
            BasicColumn::Session => "Session",
            BasicColumn::TTY => "TTY",
            BasicColumn::TTYGID => "TTYGID",
            BasicColumn::Flags => "Flags",
            BasicColumn::Utime => "Utime",
            BasicColumn::Stime => "Stime",
            BasicColumn::Priority => "Priority",
            BasicColumn::Nice => "Nice",
            BasicColumn::Threads => "Threads",
            BasicColumn::Start => "Start",
            BasicColumn::Vmemory => "Vmemory",
            BasicColumn::CMD => "CMD",
            BasicColumn::Memory => "Memory",
            BasicColumn::CPU => "CPU",
        }
    }
}

#[derive(Clone, Debug)]
struct Foo {
    PID:String,
    State:String,
    PPID:String,
    GID:String,
    Session:String,
    TTY:String,
    TTYGID:String,
    Flags:String,
    Utime:String,
    Stime:String,
    Priority:String,
    Nice:String,
    Threads:String,
    Start:String,
    Vmemory:String,
    CMD:String,
    Memory:String,
    CPU:String,
}

impl TableViewItem<BasicColumn> for Foo {
    fn to_column(&self, column: BasicColumn) -> String {
        match column {
            BasicColumn::PID => self.PID.to_string(),
            BasicColumn::State => format!("{}", self.State),
            BasicColumn::PPID => format!("{}", self.PPID),
            BasicColumn::GID => format!("{}", self.GID),
            BasicColumn::Session => format!("{}", self.Session),
            BasicColumn::TTY => format!("{}", self.TTY),
            BasicColumn::TTYGID => format!("{}", self.TTYGID),
            BasicColumn::Flags => format!("{}", self.Flags),
            BasicColumn::Utime => format!("{}", self.Utime),
            BasicColumn::Stime => format!("{}", self.Stime),
            BasicColumn::Priority => format!("{}", self.Priority),
            BasicColumn::Nice => format!("{}", self.Nice),
            BasicColumn::Threads => format!("{}", self.Threads),
            BasicColumn::Start => format!("{}", self.Start),
            BasicColumn::Vmemory => format!("{}", self.Vmemory),
            BasicColumn::CMD => format!("{}", self.CMD),
            BasicColumn::Memory => format!("{}", self.Memory),
            BasicColumn::CPU => format!("{}", self.CPU),
        }
    }

    fn cmp(&self, other: &Self, column: BasicColumn) -> Ordering
    where
        Self: Sized,
    {
        match column {
            BasicColumn::PID => self.PID.cmp(&other.PID),
            BasicColumn::State => self.State.cmp(&other.State),
            BasicColumn::PPID => self.PPID.cmp(&other.PPID),
            BasicColumn::GID => self.GID.cmp(&other.GID),
            BasicColumn::Session => self.Session.cmp(&other.Session),
            BasicColumn::TTY => self.TTY.cmp(&other.TTY),
            BasicColumn::TTYGID => self.TTYGID.cmp(&other.TTYGID),
            BasicColumn::Flags => self.Flags.cmp(&other.Flags),
            BasicColumn::Utime => self.Utime.cmp(&other.Utime),
            BasicColumn::Stime => self.Stime.cmp(&other.Stime),
            BasicColumn::Priority => self.Priority.cmp(&other.Priority),
            BasicColumn::Nice => self.Nice.cmp(&other.Nice),
            BasicColumn::Threads => self.Threads.cmp(&other.Threads),
            BasicColumn::Start => self.Start.cmp(&other.Start),
            BasicColumn::Vmemory => self.Vmemory.cmp(&other.Vmemory),
            BasicColumn::CMD => self.CMD.cmp(&other.CMD),
            BasicColumn::Memory => self.Memory.cmp(&other.Memory),
            BasicColumn::CPU => self.CPU.cmp(&other.CPU),
        }
    }
}

fn create_layout() -> LinearLayout{
    let result=display_system_info();
    let mem_result=display_resource_usage();
    let button_take_snapshot = Button::new("Take Snapshot",  move |s| {
        takesnapshot();
        
        
        
    });
    let button_go_home = Button::new("Home",  move |s| {
        list_processes();
        
        
        
    });
    let button_view_tree = Button::new("View Tree",  move |s| {
       display_tree();
        
        
        
    });
    let mut layout = LinearLayout::new(Orientation::Vertical).child(Dialog::around(TextView::new("System Info\n".to_string() + &result))).child(Dialog::around(TextView::new("System resources \n".to_string() + &mem_result))).child(button_take_snapshot).child(button_go_home).child(button_view_tree);

    return layout;
}
fn list_processes() {
    let mut timeofprocesses=0.0;
    let mut rng = rand::thread_rng();
    let mut siv = cursive::default();

    for process in all_processes().unwrap() {
        if let Ok(stat) = process.unwrap().stat() {
            timeofprocesses=timeofprocesses+((stat.utime+stat.stime) as f64);
        }
    }

    //data.push(vec!["PID".to_string(), "State".to_string(), "PPID".to_string(), "GID".to_string(), "Session".to_string(), "TTY".to_string(), "TTY GID".to_string(), "Flags".to_string(), "Utime".to_string(), "Stime".to_string(), "Priority".to_string(), "Nice".to_string(), "Threads".to_string(), "Start".to_string(), "Vmemory".to_string(), "CMD".to_string(), "Memory usage".to_string(), "CPU".to_string()]);
    let mut table = TableView::<Foo, BasicColumn>::new()
    .column(BasicColumn::PID, "PID", |c| c.width_percent(3))
    .column(BasicColumn::State, "State", |c| c.align(HAlign::Center).width_percent(1))
    .column(BasicColumn::PPID, "PPID", |c| {
        c.ordering(Ordering::Greater)
            .align(HAlign::Right)
            .width_percent(2)
    })
    .column(BasicColumn::GID, "GID", |c| {c.align(HAlign::Right).width_percent(2)})
    .column(BasicColumn::Session, "Session", |c| {c.align(HAlign::Right).width_percent(2)})
    .column(BasicColumn::TTY, "TTY", |c| {c.align(HAlign::Right).width_percent(1)})
    .column(BasicColumn::TTYGID, "TTY GID", |c| c.align(HAlign::Right).width_percent(1))
    .column(BasicColumn::Flags, "Flags", |c| c.align(HAlign::Center).width_percent(3))
    .column(BasicColumn::Utime, "Utime", |c| c.align(HAlign::Center).width_percent(2))
    .column(BasicColumn::Stime, "Stime", |c| c.align(HAlign::Center).width_percent(2))
    .column(BasicColumn::Priority, "Priority", |c| c.align(HAlign::Center).width_percent(1))
    .column(BasicColumn::Nice, "Nice", |c| c.align(HAlign::Center).width_percent(1))
    .column(BasicColumn::Threads, "Threads", |c| c.align(HAlign::Center).width_percent(3))
    .column(BasicColumn::Start, "Start", |c| c.align(HAlign::Center).width_percent(3))
    .column(BasicColumn::Vmemory, "Vmemory", |c| c.align(HAlign::Center).width_percent(3))
    .column(BasicColumn::CMD, "CMD", |c| c.align(HAlign::Center).width_percent(20))
    .column(BasicColumn::Memory, "Memory usage", |c| c.align(HAlign::Center).width_percent(3))
    .column(BasicColumn::CPU, "CPU", |c| c.align(HAlign::Center).width_percent(1));
    
    

    table.set_on_sort(|siv: &mut Cursive, column: BasicColumn, order: Ordering| {
        siv.add_layer(
            Dialog::around(TextView::new(format!("{} / {:?}", column.as_str(), order)))
                .title("Sorted by")
                .button("Close", |s| {
                    s.pop_layer();
                }),
        );
    });

    table.set_on_submit(|siv: &mut Cursive, row: usize, index: usize| {
        let value = siv
            .call_on_name("table", move |table: &mut TableView<Foo, BasicColumn>| {
                format!("{:?}", table.borrow_item(index).unwrap())
            })
            .unwrap();

        siv.add_layer(
            Dialog::around(TextView::new(value))
                .title(format!("Row # {}", row))
                .button("Close", move |s| {
                    s.call_on_name("table", |table: &mut TableView<Foo, BasicColumn>| {
                        //table.remove_item(index);
                    });
                    s.pop_layer();
                }).button("Kill", move |s| {
                    s.call_on_name("table", |table: &mut TableView<Foo, BasicColumn>| {
                        process_kill(table.borrow_item(index).unwrap().PID.to_string().parse().unwrap());
                    });
                    s.pop_layer();
                }).button("Pause",move |s|{
                    s.call_on_name("table", |table: &mut TableView<Foo, BasicColumn>| {
                        pause_process(table.borrow_item(index).unwrap().PID.to_string().parse().unwrap());
                    });
                    s.pop_layer();
                }).button("Resume",move |s|{
                    s.call_on_name("table", |table: &mut TableView<Foo, BasicColumn>| {
                        resume_process(table.borrow_item(index).unwrap().PID.to_string().parse().unwrap());
                    });
                    s.pop_layer();
                }),
        );
    });
    let mut items2 = Vec::new();
            for process in all_processes().unwrap() {
                if let Ok(stat) = process.unwrap().stat() {
                    let process = psutil::process::Process::new(stat.pid as u32).unwrap();
                    let memory_usage = process.memory_info().unwrap().rss();
                    let memory_usage_mb = memory_usage as f64 / 1024.0 / 1024.0;
                    items2.push(Foo {
                        PID: format!("{}", stat.pid),
                        State: format!("{}", stat.state),
                        PPID: format!("{}", stat.ppid),
                        GID: format!("{}", stat.pgrp),
                        Session: format!("{}", stat.session),
                        TTY: format!("{}", stat.tty_nr),
                        TTYGID: format!("{}", stat.tpgid),
                        Flags: format!("{}", stat.flags),
                        Utime: format!("{}", stat.utime),
                        Stime: format!("{}", stat.stime),
                        Priority: format!("{}", stat.priority),
                        Nice: format!("{}", stat.nice),
                        Threads: format!("{}", stat.num_threads),
                        Start: format!("{}", stat.starttime),
                        Vmemory: format!("{}", stat.vsize),
                        CMD: format!("{}", stat.comm),
                        Memory: format!("{:.2} MB", memory_usage_mb),
                        CPU: format!("{:.2}%", ((stat.utime+stat.stime) as f64/timeofprocesses)*100.0),
                    });
                }
            }
    table.set_items(items2);
    let table_box =  Dialog::around(table.with_name("table").min_size((150, 120))).title("Process Table");
    let mut layout=create_layout();
    let button_go_gui = Button::new("Switch to GUI",  move |s| {
    let output = Command::new("./taskfusion_gui")
                                .output()
                                .expect("Failed to execute GUI");
    //println!("{}", String::from_utf8_lossy(&output.stdout));
    });
    layout.add_child(button_go_gui);
    let mut layout2=LinearLayout::new(Orientation::Horizontal);
    layout2.add_child(TextView::new("PID: "));
    layout2.add_child(EditView::new().on_submit(move |s, text| {
        if let Ok(x) = text.parse(){
                
            filterby_pid(x);
        }
    }).fixed_width(10));
    layout2.add_child(TextView::new("PPID: "));
    layout2.add_child(EditView::new().on_submit(move |s, text| {
        if let Ok(x) = text.parse(){
                
            filterby_PPID(x);
        }
    }).fixed_width(10));
    layout2.add_child(TextView::new("GID: "));
    layout2.add_child(EditView::new().on_submit(move |s, text| {
        if let Ok(x) = text.parse(){
                
            filterby_GID(x);
        }
    }).fixed_width(10));
    layout2.add_child(TextView::new("State: "));
    layout2.add_child(EditView::new().on_submit(move |s, text| {
        if let Ok(x) = text.parse(){
                
            filterby_state(x);
        }
    }).fixed_width(10));
    layout2.add_child(TextView::new("CMD: "));
    layout2.add_child(EditView::new().on_submit(move |s, text| {
        if let Ok(x) = text.parse(){
                
            filterby_CMD(x);
        }
    }).fixed_width(10));
    
    
    
    layout.add_child(layout2);
    layout.add_child(table_box);
    
    let cb_sink = siv.cb_sink().clone();
    let duration = std::time::Duration::from_millis(5000);
    std::thread::spawn(move || {
        loop {
            let mut items = Vec::new();
            for process in all_processes().unwrap() {
                if let Ok(stat) = process.unwrap().stat() {
                    let process = psutil::process::Process::new(stat.pid as u32).unwrap();
                    let memory_usage = process.memory_info().unwrap().rss();
                    let memory_usage_mb = memory_usage as f64 / 1024.0 / 1024.0;
                    items.push(Foo {
                        PID: format!("{}", stat.pid),
                        State: format!("{}", stat.state),
                        PPID: format!("{}", stat.ppid),
                        GID: format!("{}", stat.pgrp),
                        Session: format!("{}", stat.session),
                        TTY: format!("{}", stat.tty_nr),
                        TTYGID: format!("{}", stat.tpgid),
                        Flags: format!("{}", stat.flags),
                        Utime: format!("{}", stat.utime),
                        Stime: format!("{}", stat.stime),
                        Priority: format!("{}", stat.priority),
                        Nice: format!("{}", stat.nice),
                        Threads: format!("{}", stat.num_threads),
                        Start: format!("{}", stat.starttime),
                        Vmemory: format!("{}", stat.vsize),
                        CMD: format!("{}", stat.comm),
                        Memory: format!("{:.2} MB", memory_usage_mb),
                        CPU: format!("{:.2}%", ((stat.utime+stat.stime) as f64/timeofprocesses)*100.0),
                    });
                }
            }
            std::thread::sleep(duration);
            cb_sink
                .send(Box::new( |s| {
                    s.call_on_name("table", |v: &mut TableView<Foo, BasicColumn>| {
                        v.set_items(items)
                    });
                }))
                .unwrap();
        }
        cb_sink.send(Box::new(|s| s.quit())).unwrap();
    });
    siv.add_layer(layout);
    siv.load_toml(include_str!("/home/cse/Desktop/process/theme.toml")).unwrap();

    



    siv.run();
}
fn filterby_pid(pid: i32)
{
    let mut timeofprocesses=0.0;
    let mut rng = rand::thread_rng();
    let mut siv = cursive::default();
    for process in all_processes().unwrap() {
        if let Ok(stat) = process.unwrap().stat() {
            timeofprocesses=timeofprocesses+((stat.utime+stat.stime) as f64);
        }
    }
    let mut found=false;
    let mut table = TableView::<Foo, BasicColumn>::new()
    .column(BasicColumn::PID, "PID", |c| c.width_percent(3))
    .column(BasicColumn::State, "State", |c| c.align(HAlign::Center).width_percent(1))
    .column(BasicColumn::PPID, "PPID", |c| {
        c.ordering(Ordering::Greater)
            .align(HAlign::Right)
            .width_percent(2)
    })
    .column(BasicColumn::GID, "GID", |c| {c.align(HAlign::Right).width_percent(2)})
    .column(BasicColumn::Session, "Session", |c| {c.align(HAlign::Right).width_percent(2)})
    .column(BasicColumn::TTY, "TTY", |c| {c.align(HAlign::Right).width_percent(1)})
    .column(BasicColumn::TTYGID, "TTY GID", |c| c.align(HAlign::Right).width_percent(1))
    .column(BasicColumn::Flags, "Flags", |c| c.align(HAlign::Center).width_percent(3))
    .column(BasicColumn::Utime, "Utime", |c| c.align(HAlign::Center).width_percent(2))
    .column(BasicColumn::Stime, "Stime", |c| c.align(HAlign::Center).width_percent(2))
    .column(BasicColumn::Priority, "Priority", |c| c.align(HAlign::Center).width_percent(1))
    .column(BasicColumn::Nice, "Nice", |c| c.align(HAlign::Center).width_percent(1))
    .column(BasicColumn::Threads, "Threads", |c| c.align(HAlign::Center).width_percent(3))
    .column(BasicColumn::Start, "Start", |c| c.align(HAlign::Center).width_percent(3))
    .column(BasicColumn::Vmemory, "Vmemory", |c| c.align(HAlign::Center).width_percent(3))
    .column(BasicColumn::CMD, "CMD", |c| c.align(HAlign::Center).width_percent(20))
    .column(BasicColumn::Memory, "Memory usage", |c| c.align(HAlign::Center).width_percent(3))
    .column(BasicColumn::CPU, "CPU", |c| c.align(HAlign::Center).width_percent(1));
    

    table.set_on_sort(|siv: &mut Cursive, column: BasicColumn, order: Ordering| {
        siv.add_layer(
            Dialog::around(TextView::new(format!("{} / {:?}", column.as_str(), order)))
                .title("Sorted by")
                .button("Close", |s| {
                    s.pop_layer();
                }),
        );
    });

    table.set_on_submit(|siv: &mut Cursive, row: usize, index: usize| {
        let value = siv
            .call_on_name("table", move |table: &mut TableView<Foo, BasicColumn>| {
                format!("{:?}", table.borrow_item(index).unwrap())
            })
            .unwrap();

            siv.add_layer(
                Dialog::around(TextView::new(value))
                    .title(format!("Row # {}", row))
                    .button("Close", move |s| {
                        s.call_on_name("table", |table: &mut TableView<Foo, BasicColumn>| {
                            //table.remove_item(index);
                        });
                        s.pop_layer();
                    }).button("Kill", move |s| {
                        s.call_on_name("table", |table: &mut TableView<Foo, BasicColumn>| {
                            process_kill(table.borrow_item(index).unwrap().PID.to_string().parse().unwrap());
                        });
                        s.pop_layer();
                    }).button("Pause",move |s|{
                        s.call_on_name("table", |table: &mut TableView<Foo, BasicColumn>| {
                            pause_process(table.borrow_item(index).unwrap().PID.to_string().parse().unwrap());
                        });
                        s.pop_layer();
                    }).button("Resume",move |s|{
                        s.call_on_name("table", |table: &mut TableView<Foo, BasicColumn>| {
                            resume_process(table.borrow_item(index).unwrap().PID.to_string().parse().unwrap());
                        });
                        s.pop_layer();
                    }),
            );
    });
    let mut items2 = Vec::new();
            for process in all_processes().unwrap() {
                if let Ok(stat) = process.unwrap().stat() {
                    if stat.pid==pid
                    {
                    let process = psutil::process::Process::new(stat.pid as u32).unwrap();
                    let memory_usage = process.memory_info().unwrap().rss();
                    let memory_usage_mb = memory_usage as f64 / 1024.0 / 1024.0;
                    items2.push(Foo {
                        PID: format!("{}", stat.pid),
                        State: format!("{}", stat.state),
                        PPID: format!("{}", stat.ppid),
                        GID: format!("{}", stat.pgrp),
                        Session: format!("{}", stat.session),
                        TTY: format!("{}", stat.tty_nr),
                        TTYGID: format!("{}", stat.tpgid),
                        Flags: format!("{}", stat.flags),
                        Utime: format!("{}", stat.utime),
                        Stime: format!("{}", stat.stime),
                        Priority: format!("{}", stat.priority),
                        Nice: format!("{}", stat.nice),
                        Threads: format!("{}", stat.num_threads),
                        Start: format!("{}", stat.starttime),
                        Vmemory: format!("{}", stat.vsize),
                        CMD: format!("{}", stat.comm),
                        Memory: format!("{:.2} MB", memory_usage_mb),
                        CPU: format!("{:.2}%", ((stat.utime+stat.stime) as f64/timeofprocesses)*100.0),
                    });
                }}
            }
    table.set_items(items2);

    let table_box =  Dialog::around(table.with_name("table").min_size((150, 120))).title("Process Table");
    let mut layout=create_layout();
    layout.add_child(table_box);
    let cb_sink = siv.cb_sink().clone();
    let duration = std::time::Duration::from_millis(5000);
    std::thread::spawn(move || {
        loop {
            let mut items = Vec::new();
            for process in all_processes().unwrap() {
                if let Ok(stat) = process.unwrap().stat() {
                    if stat.pid==pid
                    {

                    
                    let process = psutil::process::Process::new(stat.pid as u32).unwrap();
                    let memory_usage = process.memory_info().unwrap().rss();
                    let memory_usage_mb = memory_usage as f64 / 1024.0 / 1024.0;
                    items.push(Foo {
                        PID: format!("{}", stat.pid),
                        State: format!("{}", stat.state),
                        PPID: format!("{}", stat.ppid),
                        GID: format!("{}", stat.pgrp),
                        Session: format!("{}", stat.session),
                        TTY: format!("{}", stat.tty_nr),
                        TTYGID: format!("{}", stat.tpgid),
                        Flags: format!("{}", stat.flags),
                        Utime: format!("{}", stat.utime),
                        Stime: format!("{}", stat.stime),
                        Priority: format!("{}", stat.priority),
                        Nice: format!("{}", stat.nice),
                        Threads: format!("{}", stat.num_threads),
                        Start: format!("{}", stat.starttime),
                        Vmemory: format!("{}", stat.vsize),
                        CMD: format!("{}", stat.comm),
                        Memory: format!("{:.2} MB", memory_usage_mb),
                        CPU: format!("{:.2}%", ((stat.utime+stat.stime) as f64/timeofprocesses)*100.0),
                    });
                }
                }
            }
            std::thread::sleep(duration);
            cb_sink
                .send(Box::new( |s| {
                    s.call_on_name("table", |v: &mut TableView<Foo, BasicColumn>| {
                        v.set_items(items)
                    });
                }))
                .unwrap();
        }
        cb_sink.send(Box::new(|s| s.quit())).unwrap();
    });
    siv.add_layer(layout);
    siv.load_toml(include_str!("/home/cse/Desktop/process/theme.toml")).unwrap();



    siv.run();       
    

}
fn filterby_state(state: char)
{
    let mut timeofprocesses=0.0;
    let mut rng = rand::thread_rng();
    let mut siv = cursive::default();
    for process in all_processes().unwrap() {
        if let Ok(stat) = process.unwrap().stat() {
            timeofprocesses=timeofprocesses+((stat.utime+stat.stime) as f64);
        }
    }
    let mut found=false;
    let mut table = TableView::<Foo, BasicColumn>::new()
    .column(BasicColumn::PID, "PID", |c| c.width_percent(3))
    .column(BasicColumn::State, "State", |c| c.align(HAlign::Center).width_percent(1))
    .column(BasicColumn::PPID, "PPID", |c| {
        c.ordering(Ordering::Greater)
            .align(HAlign::Right)
            .width_percent(2)
    })
    .column(BasicColumn::GID, "GID", |c| {c.align(HAlign::Right).width_percent(2)})
    .column(BasicColumn::Session, "Session", |c| {c.align(HAlign::Right).width_percent(2)})
    .column(BasicColumn::TTY, "TTY", |c| {c.align(HAlign::Right).width_percent(1)})
    .column(BasicColumn::TTYGID, "TTY GID", |c| c.align(HAlign::Right).width_percent(1))
    .column(BasicColumn::Flags, "Flags", |c| c.align(HAlign::Center).width_percent(3))
    .column(BasicColumn::Utime, "Utime", |c| c.align(HAlign::Center).width_percent(2))
    .column(BasicColumn::Stime, "Stime", |c| c.align(HAlign::Center).width_percent(2))
    .column(BasicColumn::Priority, "Priority", |c| c.align(HAlign::Center).width_percent(1))
    .column(BasicColumn::Nice, "Nice", |c| c.align(HAlign::Center).width_percent(1))
    .column(BasicColumn::Threads, "Threads", |c| c.align(HAlign::Center).width_percent(3))
    .column(BasicColumn::Start, "Start", |c| c.align(HAlign::Center).width_percent(3))
    .column(BasicColumn::Vmemory, "Vmemory", |c| c.align(HAlign::Center).width_percent(3))
    .column(BasicColumn::CMD, "CMD", |c| c.align(HAlign::Center).width_percent(20))
    .column(BasicColumn::Memory, "Memory usage", |c| c.align(HAlign::Center).width_percent(3))
    .column(BasicColumn::CPU, "CPU", |c| c.align(HAlign::Center).width_percent(1));

    table.set_on_sort(|siv: &mut Cursive, column: BasicColumn, order: Ordering| {
        siv.add_layer(
            Dialog::around(TextView::new(format!("{} / {:?}", column.as_str(), order)))
                .title("Sorted by")
                .button("Close", |s| {
                    s.pop_layer();
                }),
        );
    });

    table.set_on_submit(|siv: &mut Cursive, row: usize, index: usize| {
        let value = siv
            .call_on_name("table", move |table: &mut TableView<Foo, BasicColumn>| {
                format!("{:?}", table.borrow_item(index).unwrap())
            })
            .unwrap();

        siv.add_layer(
            Dialog::around(TextView::new(value))
                .title(format!("Row # {}", row))
                .button("Close", move |s| {
                    s.call_on_name("table", |table: &mut TableView<Foo, BasicColumn>| {
                        //table.remove_item(index);
                    });
                    s.pop_layer();
                }).button("Kill", move |s| {
                    s.call_on_name("table", |table: &mut TableView<Foo, BasicColumn>| {
                        process_kill(table.borrow_item(index).unwrap().PID.to_string().parse().unwrap());
                    });
                    s.pop_layer();
                }).button("Pause",move |s|{
                    s.call_on_name("table", |table: &mut TableView<Foo, BasicColumn>| {
                        pause_process(table.borrow_item(index).unwrap().PID.to_string().parse().unwrap());
                    });
                s.pop_layer();
                }).button("Resume",move |s|{
                    s.call_on_name("table", |table: &mut TableView<Foo, BasicColumn>| {
                        resume_process(table.borrow_item(index).unwrap().PID.to_string().parse().unwrap());
                    });
                s.pop_layer();
                }),
            )
});
let mut items2 = Vec::new();
for process in all_processes().unwrap() {
    if let Ok(stat) = process.unwrap().stat() {
        if stat.state==state
        {
        let process = psutil::process::Process::new(stat.pid as u32).unwrap();
        let memory_usage = process.memory_info().unwrap().rss();
        let memory_usage_mb = memory_usage as f64 / 1024.0 / 1024.0;
        items2.push(Foo {
            PID: format!("{}", stat.pid),
            State: format!("{}", stat.state),
            PPID: format!("{}", stat.ppid),
            GID: format!("{}", stat.pgrp),
            Session: format!("{}", stat.session),
            TTY: format!("{}", stat.tty_nr),
            TTYGID: format!("{}", stat.tpgid),
            Flags: format!("{}", stat.flags),
            Utime: format!("{}", stat.utime),
            Stime: format!("{}", stat.stime),
            Priority: format!("{}", stat.priority),
            Nice: format!("{}", stat.nice),
            Threads: format!("{}", stat.num_threads),
            Start: format!("{}", stat.starttime),
            Vmemory: format!("{}", stat.vsize),
            CMD: format!("{}", stat.comm),
            Memory: format!("{:.2} MB", memory_usage_mb),
            CPU: format!("{:.2}%", ((stat.utime+stat.stime) as f64/timeofprocesses)*100.0),
        });
    }}
}
table.set_items(items2);

    let table_box =  Dialog::around(table.with_name("table").min_size((150, 120))).title("Process Table");
    let mut layout=create_layout();
    layout.add_child(table_box);
    let cb_sink = siv.cb_sink().clone();
    let duration = std::time::Duration::from_millis(5000);
    std::thread::spawn(move || {
        loop {
            let mut items = Vec::new();
            for process in all_processes().unwrap() {
                if let Ok(stat) = process.unwrap().stat() {
                    if stat.state==state
                    {
                    let process = psutil::process::Process::new(stat.pid as u32).unwrap();
                    let memory_usage = process.memory_info().unwrap().rss();
                    let memory_usage_mb = memory_usage as f64 / 1024.0 / 1024.0;
                    items.push(Foo {
                        PID: format!("{}", stat.pid),
                        State: format!("{}", stat.state),
                        PPID: format!("{}", stat.ppid),
                        GID: format!("{}", stat.pgrp),
                        Session: format!("{}", stat.session),
                        TTY: format!("{}", stat.tty_nr),
                        TTYGID: format!("{}", stat.tpgid),
                        Flags: format!("{}", stat.flags),
                        Utime: format!("{}", stat.utime),
                        Stime: format!("{}", stat.stime),
                        Priority: format!("{}", stat.priority),
                        Nice: format!("{}", stat.nice),
                        Threads: format!("{}", stat.num_threads),
                        Start: format!("{}", stat.starttime),
                        Vmemory: format!("{}", stat.vsize),
                        CMD: format!("{}", stat.comm),
                        Memory: format!("{:.2} MB", memory_usage_mb),
                        CPU: format!("{:.2}%", ((stat.utime+stat.stime) as f64/timeofprocesses)*100.0),
                    });
                }
                }
            }
            std::thread::sleep(duration);
            cb_sink
                .send(Box::new( |s| {
                    s.call_on_name("table", |v: &mut TableView<Foo, BasicColumn>| {
                        v.set_items(items)
                    });
                }))
                .unwrap();
        }
        cb_sink.send(Box::new(|s| s.quit())).unwrap();
    });
    siv.add_layer(layout);
    siv.load_toml(include_str!("/home/cse/Desktop/process/theme.toml")).unwrap();



    siv.run();      
    

}

fn filterby_CMD(CMD: String)
{
    let mut timeofprocesses=0.0;
    let mut rng = rand::thread_rng();
    let mut siv = cursive::default();
    for process in all_processes().unwrap() {
        if let Ok(stat) = process.unwrap().stat() {
            timeofprocesses=timeofprocesses+((stat.utime+stat.stime) as f64);
        }
    }
    let mut found=false;
    let mut table = TableView::<Foo, BasicColumn>::new()
    .column(BasicColumn::PID, "PID", |c| c.width_percent(3))
    .column(BasicColumn::State, "State", |c| c.align(HAlign::Center).width_percent(1))
    .column(BasicColumn::PPID, "PPID", |c| {
        c.ordering(Ordering::Greater)
            .align(HAlign::Right)
            .width_percent(2)
    })
    .column(BasicColumn::GID, "GID", |c| {c.align(HAlign::Right).width_percent(2)})
    .column(BasicColumn::Session, "Session", |c| {c.align(HAlign::Right).width_percent(2)})
    .column(BasicColumn::TTY, "TTY", |c| {c.align(HAlign::Right).width_percent(1)})
    .column(BasicColumn::TTYGID, "TTY GID", |c| c.align(HAlign::Right).width_percent(1))
    .column(BasicColumn::Flags, "Flags", |c| c.align(HAlign::Center).width_percent(3))
    .column(BasicColumn::Utime, "Utime", |c| c.align(HAlign::Center).width_percent(2))
    .column(BasicColumn::Stime, "Stime", |c| c.align(HAlign::Center).width_percent(2))
    .column(BasicColumn::Priority, "Priority", |c| c.align(HAlign::Center).width_percent(1))
    .column(BasicColumn::Nice, "Nice", |c| c.align(HAlign::Center).width_percent(1))
    .column(BasicColumn::Threads, "Threads", |c| c.align(HAlign::Center).width_percent(3))
    .column(BasicColumn::Start, "Start", |c| c.align(HAlign::Center).width_percent(3))
    .column(BasicColumn::Vmemory, "Vmemory", |c| c.align(HAlign::Center).width_percent(3))
    .column(BasicColumn::CMD, "CMD", |c| c.align(HAlign::Center).width_percent(20))
    .column(BasicColumn::Memory, "Memory usage", |c| c.align(HAlign::Center).width_percent(3))
    .column(BasicColumn::CPU, "CPU", |c| c.align(HAlign::Center).width_percent(1));

    table.set_on_sort(|siv: &mut Cursive, column: BasicColumn, order: Ordering| {
        siv.add_layer(
            Dialog::around(TextView::new(format!("{} / {:?}", column.as_str(), order)))
                .title("Sorted by")
                .button("Close", |s| {
                    s.pop_layer();
                }),
        );
    });

    table.set_on_submit(|siv: &mut Cursive, row: usize, index: usize| {
        let value = siv
            .call_on_name("table", move |table: &mut TableView<Foo, BasicColumn>| {
                format!("{:?}", table.borrow_item(index).unwrap())
            })
            .unwrap();

            siv.add_layer(
                Dialog::around(TextView::new(value))
                    .title(format!("Row # {}", row))
                    .button("Close", move |s| {
                        s.call_on_name("table", |table: &mut TableView<Foo, BasicColumn>| {
                            //table.remove_item(index);
                        });
                        s.pop_layer();
                    }).button("Kill", move |s| {
                        s.call_on_name("table", |table: &mut TableView<Foo, BasicColumn>| {
                            process_kill(table.borrow_item(index).unwrap().PID.to_string().parse().unwrap());
                        });
                        s.pop_layer();
                    }).button("Pause",move |s|{
                        s.call_on_name("table", |table: &mut TableView<Foo, BasicColumn>| {
                            pause_process(table.borrow_item(index).unwrap().PID.to_string().parse().unwrap());
                        });
                    s.pop_layer();
                    }).button("Resume",move |s|{
                        s.call_on_name("table", |table: &mut TableView<Foo, BasicColumn>| {
                            resume_process(table.borrow_item(index).unwrap().PID.to_string().parse().unwrap());
                        });
                    s.pop_layer();
                    }),
                );
    });
    let regex = Regex::new(CMD.as_str()).unwrap();
    let mut items2 = Vec::new();
    for process in all_processes().unwrap() {
        if let Ok(stat) = process.unwrap().stat() {
            if regex.is_match(&stat.comm)
            {
            let process = psutil::process::Process::new(stat.pid as u32).unwrap();
            let memory_usage = process.memory_info().unwrap().rss();
            let memory_usage_mb = memory_usage as f64 / 1024.0 / 1024.0;
            items2.push(Foo {
                PID: format!("{}", stat.pid),
                State: format!("{}", stat.state),
                PPID: format!("{}", stat.ppid),
                GID: format!("{}", stat.pgrp),
                Session: format!("{}", stat.session),
                TTY: format!("{}", stat.tty_nr),
                TTYGID: format!("{}", stat.tpgid),
                Flags: format!("{}", stat.flags),
                Utime: format!("{}", stat.utime),
                Stime: format!("{}", stat.stime),
                Priority: format!("{}", stat.priority),
                Nice: format!("{}", stat.nice),
                Threads: format!("{}", stat.num_threads),
                Start: format!("{}", stat.starttime),
                Vmemory: format!("{}", stat.vsize),
                CMD: format!("{}", stat.comm),
                Memory: format!("{:.2} MB", memory_usage_mb),
                CPU: format!("{:.2}%", ((stat.utime+stat.stime) as f64/timeofprocesses)*100.0),
            });
        }
        }
    }
table.set_items(items2);
    

    let table_box =  Dialog::around(table.with_name("table").min_size((150, 120))).title("Process Table");
    let mut layout=create_layout();
    layout.add_child(table_box);
    let cb_sink = siv.cb_sink().clone();
    let duration = std::time::Duration::from_millis(5000);
    std::thread::spawn(move || {
        loop {
            let mut items = Vec::new();
            for process in all_processes().unwrap() {
                if let Ok(stat) = process.unwrap().stat() {
                    if regex.is_match(&stat.comm)
                    {
                    let process = psutil::process::Process::new(stat.pid as u32).unwrap();
                    let memory_usage = process.memory_info().unwrap().rss();
                    let memory_usage_mb = memory_usage as f64 / 1024.0 / 1024.0;
                    items.push(Foo {
                        PID: format!("{}", stat.pid),
                        State: format!("{}", stat.state),
                        PPID: format!("{}", stat.ppid),
                        GID: format!("{}", stat.pgrp),
                        Session: format!("{}", stat.session),
                        TTY: format!("{}", stat.tty_nr),
                        TTYGID: format!("{}", stat.tpgid),
                        Flags: format!("{}", stat.flags),
                        Utime: format!("{}", stat.utime),
                        Stime: format!("{}", stat.stime),
                        Priority: format!("{}", stat.priority),
                        Nice: format!("{}", stat.nice),
                        Threads: format!("{}", stat.num_threads),
                        Start: format!("{}", stat.starttime),
                        Vmemory: format!("{}", stat.vsize),
                        CMD: format!("{}", stat.comm),
                        Memory: format!("{:.2} MB", memory_usage_mb),
                        CPU: format!("{:.2}%", ((stat.utime+stat.stime) as f64/timeofprocesses)*100.0),
                    });}
                }
            }
            std::thread::sleep(duration);
            cb_sink
                .send(Box::new( |s| {
                    s.call_on_name("table", |v: &mut TableView<Foo, BasicColumn>| {
                        v.set_items(items)
                    });
                }))
                .unwrap();
        }
        cb_sink.send(Box::new(|s| s.quit())).unwrap();
    });
    siv.add_layer(layout);
    siv.load_toml(include_str!("/home/cse/Desktop/process/theme.toml")).unwrap();



    siv.run();      
    

}
fn filterby_PPID(PPID: i32)
{
    let mut timeofprocesses=0.0;
    let mut rng = rand::thread_rng();
    let mut siv = cursive::default();
    for process in all_processes().unwrap() {
        if let Ok(stat) = process.unwrap().stat() {
            timeofprocesses=timeofprocesses+((stat.utime+stat.stime) as f64);
        }
    }
    let mut found=false;
    let mut table = TableView::<Foo, BasicColumn>::new()
    .column(BasicColumn::PID, "PID", |c| c.width_percent(3))
    .column(BasicColumn::State, "State", |c| c.align(HAlign::Center).width_percent(1))
    .column(BasicColumn::PPID, "PPID", |c| {
        c.ordering(Ordering::Greater)
            .align(HAlign::Right)
            .width_percent(2)
    })
    .column(BasicColumn::GID, "GID", |c| {c.align(HAlign::Right).width_percent(2)})
    .column(BasicColumn::Session, "Session", |c| {c.align(HAlign::Right).width_percent(2)})
    .column(BasicColumn::TTY, "TTY", |c| {c.align(HAlign::Right).width_percent(1)})
    .column(BasicColumn::TTYGID, "TTY GID", |c| c.align(HAlign::Right).width_percent(1))
    .column(BasicColumn::Flags, "Flags", |c| c.align(HAlign::Center).width_percent(3))
    .column(BasicColumn::Utime, "Utime", |c| c.align(HAlign::Center).width_percent(2))
    .column(BasicColumn::Stime, "Stime", |c| c.align(HAlign::Center).width_percent(2))
    .column(BasicColumn::Priority, "Priority", |c| c.align(HAlign::Center).width_percent(1))
    .column(BasicColumn::Nice, "Nice", |c| c.align(HAlign::Center).width_percent(1))
    .column(BasicColumn::Threads, "Threads", |c| c.align(HAlign::Center).width_percent(3))
    .column(BasicColumn::Start, "Start", |c| c.align(HAlign::Center).width_percent(3))
    .column(BasicColumn::Vmemory, "Vmemory", |c| c.align(HAlign::Center).width_percent(3))
    .column(BasicColumn::CMD, "CMD", |c| c.align(HAlign::Center).width_percent(20))
    .column(BasicColumn::Memory, "Memory usage", |c| c.align(HAlign::Center).width_percent(3))
    .column(BasicColumn::CPU, "CPU", |c| c.align(HAlign::Center).width_percent(1));
    
    table.set_on_sort(|siv: &mut Cursive, column: BasicColumn, order: Ordering| {
        siv.add_layer(
            Dialog::around(TextView::new(format!("{} / {:?}", column.as_str(), order)))
                .title("Sorted by")
                .button("Close", |s| {
                    s.pop_layer();
                }),
        );
    });

    table.set_on_submit(|siv: &mut Cursive, row: usize, index: usize| {
        let value = siv
            .call_on_name("table", move |table: &mut TableView<Foo, BasicColumn>| {
                format!("{:?}", table.borrow_item(index).unwrap())
            })
            .unwrap();

            siv.add_layer(
                Dialog::around(TextView::new(value))
                    .title(format!("Row # {}", row))
                    .button("Close", move |s| {
                        s.call_on_name("table", |table: &mut TableView<Foo, BasicColumn>| {
                            //table.remove_item(index);
                        });
                        s.pop_layer();
                    }).button("Kill", move |s| {
                        s.call_on_name("table", |table: &mut TableView<Foo, BasicColumn>| {
                            process_kill(table.borrow_item(index).unwrap().PID.to_string().parse().unwrap());
                        });
                        s.pop_layer();
                    }).button("Pause",move |s|{
                        s.call_on_name("table", |table: &mut TableView<Foo, BasicColumn>| {
                            pause_process(table.borrow_item(index).unwrap().PID.to_string().parse().unwrap());
                        });
                    s.pop_layer();
                    }).button("Resume",move |s|{
                        s.call_on_name("table", |table: &mut TableView<Foo, BasicColumn>| {
                            resume_process(table.borrow_item(index).unwrap().PID.to_string().parse().unwrap());
                        });
                    s.pop_layer();
                    }),
                );
    });
    let mut items2 = Vec::new();
            for process in all_processes().unwrap() {
                if let Ok(stat) = process.unwrap().stat() {
                    if(stat.ppid==PPID)
                    {
                    let process = psutil::process::Process::new(stat.pid as u32).unwrap();
                    let memory_usage = process.memory_info().unwrap().rss();
                    let memory_usage_mb = memory_usage as f64 / 1024.0 / 1024.0;
                    items2.push(Foo {
                        PID: format!("{}", stat.pid),
                        State: format!("{}", stat.state),
                        PPID: format!("{}", stat.ppid),
                        GID: format!("{}", stat.pgrp),
                        Session: format!("{}", stat.session),
                        TTY: format!("{}", stat.tty_nr),
                        TTYGID: format!("{}", stat.tpgid),
                        Flags: format!("{}", stat.flags),
                        Utime: format!("{}", stat.utime),
                        Stime: format!("{}", stat.stime),
                        Priority: format!("{}", stat.priority),
                        Nice: format!("{}", stat.nice),
                        Threads: format!("{}", stat.num_threads),
                        Start: format!("{}", stat.starttime),
                        Vmemory: format!("{}", stat.vsize),
                        CMD: format!("{}", stat.comm),
                        Memory: format!("{:.2} MB", memory_usage_mb),
                        CPU: format!("{:.2}%", ((stat.utime+stat.stime) as f64/timeofprocesses)*100.0),
                    });
                }}
            }
    table.set_items(items2);

    let table_box =  Dialog::around(table.with_name("table").min_size((150, 120))).title("Process Table");
    let mut layout=create_layout();
    layout.add_child(table_box);
    let cb_sink = siv.cb_sink().clone();
    let duration = std::time::Duration::from_millis(5000);
    std::thread::spawn(move || {
        loop {
            let mut items = Vec::new();
            for process in all_processes().unwrap() {
                if let Ok(stat) = process.unwrap().stat() {
                    if stat.ppid==PPID{
                    let process = psutil::process::Process::new(stat.pid as u32).unwrap();
                    let memory_usage = process.memory_info().unwrap().rss();
                    let memory_usage_mb = memory_usage as f64 / 1024.0 / 1024.0;
                    items.push(Foo {
                        PID: format!("{}", stat.pid),
                        State: format!("{}", stat.state),
                        PPID: format!("{}", stat.ppid),
                        GID: format!("{}", stat.pgrp),
                        Session: format!("{}", stat.session),
                        TTY: format!("{}", stat.tty_nr),
                        TTYGID: format!("{}", stat.tpgid),
                        Flags: format!("{}", stat.flags),
                        Utime: format!("{}", stat.utime),
                        Stime: format!("{}", stat.stime),
                        Priority: format!("{}", stat.priority),
                        Nice: format!("{}", stat.nice),
                        Threads: format!("{}", stat.num_threads),
                        Start: format!("{}", stat.starttime),
                        Vmemory: format!("{}", stat.vsize),
                        CMD: format!("{}", stat.comm),
                        Memory: format!("{:.2} MB", memory_usage_mb),
                        CPU: format!("{:.2}%", ((stat.utime+stat.stime) as f64/timeofprocesses)*100.0),
                    });}
                }
            }
            std::thread::sleep(duration);
            cb_sink
                .send(Box::new( |s| {
                    s.call_on_name("table", |v: &mut TableView<Foo, BasicColumn>| {
                        v.set_items(items)
                    });
                }))
                .unwrap();
        }
        cb_sink.send(Box::new(|s| s.quit())).unwrap();
    });
    siv.add_layer(layout);
    siv.load_toml(include_str!("/home/cse/Desktop/process/theme.toml")).unwrap();



    siv.run();       
    

}
fn filterby_GID(GID: i32)
{
    let mut timeofprocesses=0.0;
    let mut rng = rand::thread_rng();
    let mut siv = cursive::default();
    for process in all_processes().unwrap() {
        if let Ok(stat) = process.unwrap().stat() {
            timeofprocesses=timeofprocesses+((stat.utime+stat.stime) as f64);
        }
    }
    let mut found=false;
    let mut table = TableView::<Foo, BasicColumn>::new()
    .column(BasicColumn::PID, "PID", |c| c.width_percent(3))
    .column(BasicColumn::State, "State", |c| c.align(HAlign::Center).width_percent(1))
    .column(BasicColumn::PPID, "PPID", |c| {
        c.ordering(Ordering::Greater)
            .align(HAlign::Right)
            .width_percent(2)
    })
    .column(BasicColumn::GID, "GID", |c| {c.align(HAlign::Right).width_percent(2)})
    .column(BasicColumn::Session, "Session", |c| {c.align(HAlign::Right).width_percent(2)})
    .column(BasicColumn::TTY, "TTY", |c| {c.align(HAlign::Right).width_percent(1)})
    .column(BasicColumn::TTYGID, "TTY GID", |c| c.align(HAlign::Right).width_percent(1))
    .column(BasicColumn::Flags, "Flags", |c| c.align(HAlign::Center).width_percent(3))
    .column(BasicColumn::Utime, "Utime", |c| c.align(HAlign::Center).width_percent(2))
    .column(BasicColumn::Stime, "Stime", |c| c.align(HAlign::Center).width_percent(2))
    .column(BasicColumn::Priority, "Priority", |c| c.align(HAlign::Center).width_percent(1))
    .column(BasicColumn::Nice, "Nice", |c| c.align(HAlign::Center).width_percent(1))
    .column(BasicColumn::Threads, "Threads", |c| c.align(HAlign::Center).width_percent(3))
    .column(BasicColumn::Start, "Start", |c| c.align(HAlign::Center).width_percent(3))
    .column(BasicColumn::Vmemory, "Vmemory", |c| c.align(HAlign::Center).width_percent(3))
    .column(BasicColumn::CMD, "CMD", |c| c.align(HAlign::Center).width_percent(20))
    .column(BasicColumn::Memory, "Memory usage", |c| c.align(HAlign::Center).width_percent(3))
    .column(BasicColumn::CPU, "CPU", |c| c.align(HAlign::Center).width_percent(1));

    table.set_on_sort(|siv: &mut Cursive, column: BasicColumn, order: Ordering| {
        siv.add_layer(
            Dialog::around(TextView::new(format!("{} / {:?}", column.as_str(), order)))
                .title("Sorted by")
                .button("Close", |s| {
                    s.pop_layer();
                }),
                
        );
    });

    table.set_on_submit(|siv: &mut Cursive, row: usize, index: usize| {
        let value = siv
            .call_on_name("table", move |table: &mut TableView<Foo, BasicColumn>| {
                format!("{:?}", table.borrow_item(index).unwrap())
            })
            .unwrap();

            siv.add_layer(
                Dialog::around(TextView::new(value))
                    .title(format!("Row # {}", row))
                    .button("Close", move |s| {
                        s.call_on_name("table", |table: &mut TableView<Foo, BasicColumn>| {
                            //table.remove_item(index);
                        });
                        s.pop_layer();
                    }).button("Kill", move |s| {
                        s.call_on_name("table", |table: &mut TableView<Foo, BasicColumn>| {
                            process_kill(table.borrow_item(index).unwrap().PID.to_string().parse().unwrap());
                        });
                        s.pop_layer();
                    }).button("Pause",move |s|{
                        s.call_on_name("table", |table: &mut TableView<Foo, BasicColumn>| {
                            pause_process(table.borrow_item(index).unwrap().PID.to_string().parse().unwrap());
                        });
                    s.pop_layer();
                    }).button("Resume",move |s|{
                        s.call_on_name("table", |table: &mut TableView<Foo, BasicColumn>| {
                            resume_process(table.borrow_item(index).unwrap().PID.to_string().parse().unwrap());
                        });
                    s.pop_layer();
                    }),
        );
    });
    let mut items2 = Vec::new();
            for process in all_processes().unwrap() {
                if let Ok(stat) = process.unwrap().stat() {
                    if stat.pgrp==GID
                    {
                    let process = psutil::process::Process::new(stat.pid as u32).unwrap();
                    let memory_usage = process.memory_info().unwrap().rss();
                    let memory_usage_mb = memory_usage as f64 / 1024.0 / 1024.0;
                    items2.push(Foo {
                        PID: format!("{}", stat.pid),
                        State: format!("{}", stat.state),
                        PPID: format!("{}", stat.ppid),
                        GID: format!("{}", stat.pgrp),
                        Session: format!("{}", stat.session),
                        TTY: format!("{}", stat.tty_nr),
                        TTYGID: format!("{}", stat.tpgid),
                        Flags: format!("{}", stat.flags),
                        Utime: format!("{}", stat.utime),
                        Stime: format!("{}", stat.stime),
                        Priority: format!("{}", stat.priority),
                        Nice: format!("{}", stat.nice),
                        Threads: format!("{}", stat.num_threads),
                        Start: format!("{}", stat.starttime),
                        Vmemory: format!("{}", stat.vsize),
                        CMD: format!("{}", stat.comm),
                        Memory: format!("{:.2} MB", memory_usage_mb),
                        CPU: format!("{:.2}%", ((stat.utime+stat.stime) as f64/timeofprocesses)*100.0),
                    });
                }}
            }
    table.set_items(items2);

    let table_box =  Dialog::around(table.with_name("table").min_size((150, 120))).title("Process Table");
    let mut layout=create_layout();
    layout.add_child(table_box);
    let cb_sink = siv.cb_sink().clone();
    let duration = std::time::Duration::from_millis(5000);
    std::thread::spawn(move || {
        loop {
            let mut items = Vec::new();
            for process in all_processes().unwrap() {
                if let Ok(stat) = process.unwrap().stat() {
                    if stat.pgrp==GID
                {
                    let process = psutil::process::Process::new(stat.pid as u32).unwrap();
                    let memory_usage = process.memory_info().unwrap().rss();
                    let memory_usage_mb = memory_usage as f64 / 1024.0 / 1024.0;
                    items.push(Foo {
                        PID: format!("{}", stat.pid),
                        State: format!("{}", stat.state),
                        PPID: format!("{}", stat.ppid),
                        GID: format!("{}", stat.pgrp),
                        Session: format!("{}", stat.session),
                        TTY: format!("{}", stat.tty_nr),
                        TTYGID: format!("{}", stat.tpgid),
                        Flags: format!("{}", stat.flags),
                        Utime: format!("{}", stat.utime),
                        Stime: format!("{}", stat.stime),
                        Priority: format!("{}", stat.priority),
                        Nice: format!("{}", stat.nice),
                        Threads: format!("{}", stat.num_threads),
                        Start: format!("{}", stat.starttime),
                        Vmemory: format!("{}", stat.vsize),
                        CMD: format!("{}", stat.comm),
                        Memory: format!("{:.2} MB", memory_usage_mb),
                        CPU: format!("{:.2}%", ((stat.utime+stat.stime) as f64/timeofprocesses)*100.0),
                    });}
                }
            }
            std::thread::sleep(duration);
            cb_sink
                .send(Box::new( |s| {
                    s.call_on_name("table", |v: &mut TableView<Foo, BasicColumn>| {
                        v.set_items(items)
                    });
                }))
                .unwrap();
        }
        cb_sink.send(Box::new(|s| s.quit())).unwrap();
    });
    siv.add_layer(layout);
    siv.load_toml(include_str!("/home/cse/Desktop/process/theme.toml")).unwrap();



    siv.run();      
    

}

fn process_kill (id: i32) ->String
{
    



    let pid = Pid::from_raw(id); 

    match kill(pid, SIGTERM) {
        Ok(_) => "Process killed".to_string(),
        Err(e) => format!("Error killing process: {}", e).to_string(),
    }

}
fn changepriority(pid: i32, delta: i32) ->String {
    unsafe {
        let which = libc::PRIO_PROCESS;
        let who = pid as libc::id_t;
        let prio = libc::getpriority(which, who);

        if prio == -1 {
            return format!("Error getting process priority: {}", std::io::Error::last_os_error()).to_string();
        }

        let new_prio = prio + delta;
        let result = libc::setpriority(which, who, new_prio);

        if result != 0 {
            return format!("Error changing process priority: {}", std::io::Error::last_os_error()).to_string();
        } else {
            return format!("Process priority changed successfully").to_string();
        }
    }
}
fn killbythreshold(threshold:f64)
{
    let mut timeofprocesses=0.0;
    for process in all_processes().unwrap() {
        if let Ok(stat) = process.unwrap().stat() {
            timeofprocesses=timeofprocesses+((stat.utime+stat.stime) as f64);
        }
    }
    for process in all_processes().unwrap() {
        
        if let Ok(stat) = process.unwrap().stat() {
            if((((stat.utime+stat.stime) as f64)/((timeofprocesses))*100.0)>threshold)
            {
                process_kill(stat.pid);
            }
            
        }
    }
    
}
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

fn pause_process(id: i32) ->String
{
    let pid = Pid::from_raw(id); 

    match kill(pid, SIGSTOP) {
        Ok(_) => format!("Process paused").to_string(),
        Err(e) => format!("Error pausing process: {}", e).to_string(),
    }
}
fn resume_process(id:i32) ->String
{
    let pid = Pid::from_raw(id); 

    match kill(pid, SIGCONT) {
        Ok(_) => format!("Process resumed").to_string(),
        Err(e) => format!("Error resuming process: {}", e).to_string(),
    }
}
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
fn takesnapshot()
{
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
#[derive(Debug)]
#[derive(Clone)]
struct ProcessNode {
    pid: i32,
    parent_pid: i32,
    name: String,
    children: Vec<ProcessNode>,
}




fn display_tree() {



    let mut siv = Cursive::default();
    let output = Command::new("pstree")
        .arg("-p")
        .output()
        .expect("failed to execute pstree command");
    
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let tree_view = ScrollView::new(TextView::new(stdout));

    let mut pidd =0;
    let mut priorityy = 0;
    let mut thresholdd =0.0;
    let mut message:String=Default::default();;
    let text_box_kill = EditView::new()
        .on_submit(
            move |s, text| {
            if let Ok(x) = text.parse(){
                
                let cloned_message = process_kill(x).clone();
                s.add_layer(Dialog::text(cloned_message).button("Ok", |s| {s.pop_layer();}));
            }}
        );
    let text_box_recursive_kill = EditView::new()
        .on_submit(
            move |s, text| {
            if let Ok(x) = text.parse(){
                recursive_kill(x);
                s.add_layer(Dialog::text("Killing").button("Ok", |s| {s.pop_layer();}));
            }}
        );
    
    let text_box_change_priority = EditView::new()
    .on_submit(
        move |s, text| {
            if let Ok(parsed) = text.parse::<String>(){
                let items: Vec<&str> = parsed.split(',').collect();
                let pidd = items[0].parse().unwrap();
                let priorityy = items[1].parse().unwrap();
                let cloned_message=changepriority(pidd, priorityy).clone();
                s.add_layer(Dialog::text(cloned_message).button("Ok", |s| {s.pop_layer();}));
            }
        }
    );


    let text_box_pause = EditView::new()
        .on_submit(
            move |s, text| {
                if let Ok(x) = text.parse(){
                    let cloned_message=pause_process(x).clone();
                    s.add_layer(Dialog::text(cloned_message).button("Ok", |s| {s.pop_layer();}));
                }}
            );
                
    let text_box_resume = EditView::new()
        .on_submit(
            move |s, text| {
                if let Ok(x) = text.parse(){
                    let cloned_message=resume_process(x).clone();
                    s.add_layer(Dialog::text(cloned_message).button("Ok", |s| {s.pop_layer();}));
                }}
            );
            
    let text_kill_bythreshhold = EditView::new()
        .on_submit(
            move |s, text| {
                if let Ok(x) = text.parse(){
                    killbythreshold(x);
                    s.add_layer(Dialog::text("Killing").button("Ok", |s| {s.pop_layer();}));
                }}
            );
    let text_box_recursive_pause = EditView::new()
        .on_submit(
            move |s, text| {
                if let Ok(x) = text.parse(){
                    recursive_pause(x);
                    s.add_layer(Dialog::text("Pausing").button("Ok", |s| {s.pop_layer();}));
                }}
            );
    let text_box_recursive_resume = EditView::new()
        .on_submit(
            move |s, text| {
                if let Ok(x) = text.parse(){
                    recursive_resume(x);
                    s.add_layer(Dialog::text("Resuming").button("Ok", |s| {s.pop_layer();}));
                }}
            );
    
    let mut button_go_home = Button::new("Home",  move |s| {
                list_processes();
                
                
                
            });
    
    
    

     
    
    
    

    
    
    

    

    let mut layout = LinearLayout::new(Orientation::Horizontal)
        .child(Dialog::around(tree_view));
    let mut layout2=LinearLayout::new(Orientation::Vertical)
        .child(TextView::new("Enter PID to kill"))
        .child(Dialog::around(text_box_kill))
        .child(TextView::new("Enter PID to recursively kill"))
        .child(Dialog::around(text_box_recursive_kill))
        .child(TextView::new("Enter PID and priority comma separated to change priority"))
        .child(Dialog::around(text_box_change_priority))
        .child(TextView::new("Enter PID to pause"))
        .child(Dialog::around(text_box_pause))
        .child(TextView::new("Enter PID to resume"))
        .child(Dialog::around(text_box_resume))
        .child(TextView::new("Enter CPU threshold to kill process exceeding it"))
        .child(Dialog::around(text_kill_bythreshhold))
        .child(TextView::new("Enter PID to recursively pause"))
        .child(Dialog::around(text_box_recursive_pause))
        .child(TextView::new("Enter PID to recursively resume"))
        .child(Dialog::around(text_box_recursive_resume));
        layout2.add_child(button_go_home);
        let layout3=Dialog::around(layout2.scrollable());


    layout.add_child(layout3);
    


    siv.add_layer(layout);
    siv.load_toml(include_str!("/home/cse/Desktop/process/theme.toml")).unwrap();
    

    siv.run();
}


fn exec()
{
    let args: Vec<String> = env::args().collect();
    println!("{:?}",args[1]);
    if(args[1]=="-ls")
    {
        list_processes();

    }
    else if(args[1]=="-p")
    {
        if(args.len()<3)
        {
            println!("Invalid number of arguments");
            return;
        }
        else
        {
            filterby_pid(args[2].parse::<i32>().unwrap());

        }
        
        
    }
    else if(args[1]=="-s")
    {
        if(args.len()<3)
        {
            println!("Invalid number of arguments");
            return;
        }
        else
        {
            filterby_state(args[2].parse::<char>().unwrap());

        }
        
    }
    else if(args[1]=="-c")
    {
        if(args.len()<3)
        {
            println!("Invalid number of arguments");
            return;
        }
        else
        {
            filterby_CMD(args[2].to_string());

        }
        
        
    }
    else if(args[1]=="-g")
    {
        if(args.len()<3)
        {
            println!("Invalid number of arguments");
            return;
        }
        else
        {
            filterby_GID(args[2].parse::<i32>().unwrap());

        }

    }
    else if(args[1]=="-pp")
    {
        if(args.len()<3)
        {
            println!("Invalid number of arguments");
            return;
        }
        else
        {
            filterby_PPID(args[2].parse::<i32>().unwrap());

        }

    }
    else if(args[1]=="-k")
    {
        if(args.len()<3)
        {
            println!("Invalid number of arguments");
            return;
        }
        else
        {
            process_kill(args[2].parse::<i32>().unwrap());

        }
        
    }
    else if(args[1]=="-kp")
    {
        if(args.len()<3)
        {
            println!("Invalid number of arguments");
            return;
        }
        else
        {
        recursive_kill(args[2].parse::<i32>().unwrap());}
    }
    else if(args[1]=="-pa")
    {
        if(args.len()<3)
        {
            println!("Invalid number of arguments");
            return;
        }
        else
        {
            pause_process(args[2].parse::<i32>().unwrap());

        }
        
    }
    else if(args[1]=="-r")
    {
        if(args.len()<3)
        {
            println!("Invalid number of arguments");
            return;
        }
        else
        {
        resume_process(args[2].parse::<i32>().unwrap());}
    }
    else if(args[1]=="-cpr")
    {
        if(args.len()<4)
        {
            println!("Invalid number of arguments");
            return;
        }
        else
        {
        changepriority(args[2].parse::<i32>().unwrap(),args[3].parse::<i32>().unwrap());}
    }
    else if(args[1]=="-t")
    {
        takesnapshot();
    }
    else if(args[1]=="-tree")
    {
        display_tree();
    }
    else if(args[1]=="-kt")
    {
        if(args.len()<3)
        {
            println!("Invalid number of arguments");
            return;
        }
        else
        {
            killbythreshold(args[2].parse::<f64>().unwrap());}
    }
    else
    {
        println!("Invalid Command");
    }

}




fn main() {
    
  
    exec();    
    
   
   

    

    
}
