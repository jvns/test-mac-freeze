extern crate mach;
extern crate libc;
use std::io;
use std::process::Command;
use libc::{c_int, pid_t};
use mach::kern_return::KERN_SUCCESS;
use mach::port::{mach_port_name_t, MACH_PORT_NULL};
use mach::traps::mach_task_self;

pub fn main() {
    let mut command = Command::new("/usr/bin/ruby").spawn().unwrap();
    let pid = command.id() as pid_t;
    let task_id = task_for_pid(pid).expect("Failed to get task ID");
    command.kill().unwrap();
    println!("Success! Task id: {:?}", task_id);
}

pub fn task_for_pid(pid: pid_t) -> io::Result<mach_port_name_t> {
    let mut task: mach_port_name_t = MACH_PORT_NULL;
    unsafe {
        println!("Running task_for_pid...");
        let result = mach::traps::task_for_pid(mach_task_self(), pid as c_int, &mut task);
        println!("Done running task_for_pid...");
        if result != KERN_SUCCESS {
            return Err(io::Error::last_os_error());
        }
    }

    Ok(task)
}
