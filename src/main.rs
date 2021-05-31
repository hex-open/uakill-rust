mod bindings {
    windows::include_bindings!();
}

use bindings::{
    Windows::Win32::System::Diagnostics::ToolHelp::{
        CreateToolhelp32Snapshot, Process32First, Process32Next, PROCESSENTRY32, TH32CS_SNAPPROCESS,
    },
    Windows::Win32::System::SystemServices::{CHAR, THREAD_SUSPEND_RESUME},
    Windows::Win32::System::Threading::{
        OpenProcess, OpenThread, SuspendThread, TerminateProcess, PROCESS_TERMINATE,
    },
    Windows::Win32::System::WindowsProgramming::CloseHandle,
};
use std::collections::HashMap;
use std::mem::size_of;
use std::process::Command;
fn get_all_process() -> HashMap<String, u32> {
    let mut proc_map = HashMap::new();
    let blank: [CHAR; 260] = [std::default::Default::default(); 260];
    let h_process_snap = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) };
    // println!("{:?}", h_process_snap);
    let mut proc_entry = PROCESSENTRY32::default();
    proc_entry.dwSize = size_of::<PROCESSENTRY32>() as u32;
    if !unsafe { Process32First(h_process_snap, &mut proc_entry) }.as_bool() {
        panic!("Failed To Get the Process SnapShot");
    }
    loop {
        let name_array: [u8; 260] =
            unsafe { *((&proc_entry.szExeFile).as_ptr() as *const [u8; 260]) };
        let proc_name = String::from_utf8(name_array.to_vec().clone()).unwrap();
        let proc_name = proc_name.trim_matches(char::from(0));
        let proc_id = proc_entry.th32ProcessID;
        // println!("{:?}--{:?}", proc_name, proc_id);
        proc_map.insert(String::from(proc_name), proc_id);
        proc_entry.szExeFile = blank;
        if !unsafe { Process32Next(h_process_snap, &mut proc_entry) }.as_bool() {
            break;
        }
    }
    unsafe { CloseHandle(h_process_snap) };
    proc_map
}

fn suspend_thread(pid: u32) {
    let pt_sr = unsafe { OpenThread(THREAD_SUSPEND_RESUME, false, pid) };
    unsafe { SuspendThread(pt_sr) };
    unsafe { CloseHandle(pt_sr) };
}

fn kill_process(pid: u32) {
    let pt_t = unsafe { OpenProcess(PROCESS_TERMINATE, true, pid) };
    unsafe { TerminateProcess(pt_t, 0) };
    unsafe { CloseHandle(pt_t) };
}
fn main() {
    let proc_list = [
        "UniAccessAgentDaemon.exe",
        "HutiehuaApp.exe",
        "Tinaiat.exe",
        "LvaNac.exe",
        "UniSensitive.exe",
        "UniAccessAgent.exe",
        "UniAccessAgentTray.exe",
    ];

    let in_sub = std::env::args().count() > 1;

    let mut times = 0;
    loop {
        let proc_map = get_all_process();
        let mut pid_count = 0;
        proc_list.iter().for_each(|proc_name| {
            let proc_id = proc_map.get(*proc_name).unwrap_or(&0);
            pid_count += proc_id
        });
        if in_sub {
            proc_list.iter().for_each(|proc_name| {
                let proc_id = proc_map.get(*proc_name).unwrap_or(&0);
                // println!("Suspend thread: {:?}[{:?}]", proc_name, proc_id);
                suspend_thread(*proc_id);
            });
            proc_list.iter().for_each(|proc_name| {
                let proc_id = proc_map.get(*proc_name).unwrap_or(&0);
                println!("Kill process: {:?}[{:?}]", proc_name, proc_id);
                kill_process(*proc_id);
            });
        } else {
            if pid_count == 0 {
                println!("UniAccess is killed.");
                break;
            }
        }
        if !in_sub {
            times += 1;
            let cmd = std::env::args().nth(0).unwrap();
            println!("Try kill {}...", times);
            Command::new(cmd).args(&["sub"]).status().unwrap();
        } else {
            println!("Kill done...");
            std::process::exit(0);
        }
    }

    println!("Press Enter to Exit...");
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
}
