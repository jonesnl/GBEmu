use std::io::{self, prelude::*};
use std::str::SplitWhitespace;
use std::collections::{HashSet, HashMap};

use crate::hw::memory::BusWidth;
use crate::cpu::Cpu;

#[derive(PartialEq)]
enum DebuggerState {
    Running,
    Paused,
}

type DebugFn = fn(&mut Debugger, &mut Cpu, SplitWhitespace);
    
fn next_cmd(_dbgr: &mut Debugger, _: &mut Cpu, _: SplitWhitespace) {
    println!("next cmd");
}

fn print_cmd(_dbgr: &mut Debugger, cpu: &mut Cpu, _cmd_args: SplitWhitespace) {
    println!("{:04x?}", cpu.regs);
    println!("Flags: z: {}, c: {}, h: {}, n: {}", cpu.regs.get_flag_z(),
               cpu.regs.get_flag_c(), cpu.regs.get_flag_h(), cpu.regs.get_flag_n());
    println!("print cmd");
}

fn continue_cmd(dbgr: &mut Debugger, _: &mut Cpu, _: SplitWhitespace) {
    dbgr.state = DebuggerState::Running;
}

fn break_cmd(dbgr: &mut Debugger, _: &mut Cpu, mut args: SplitWhitespace) {
    let addr_str = match args.next() {
        Some(x) => x.trim_start_matches("0x"),
        None => {
            println!("Not enough arguments");
            return;
        },
    };

    let addr = match u16::from_str_radix(addr_str, 16) {
        Ok(addr) => addr,
        Err(err) => {
            println!("{}", err);
            return;
        },
    };

    dbgr.breakpoints.insert(addr);
    println!("Added breakpoint to 0x{:04x}", addr);
}

struct Cmd {
    command: &'static str,
    func: DebugFn,
    goto_next_cmd: bool,
}

const CMD_LIST: &'static [Cmd] = &[
    Cmd {
        command: "n",
        func: next_cmd,
        goto_next_cmd: true,
    },
    Cmd {
        command: "next",
        func: next_cmd,
        goto_next_cmd: true,
    },
    Cmd {
        command: "p",
        func: print_cmd,
        goto_next_cmd: false,
    },
    Cmd {
        command: "print",
        func: print_cmd,
        goto_next_cmd: false,
    },
    Cmd {
        command: "c",
        func: continue_cmd,
        goto_next_cmd: true,
    },
    Cmd {
        command: "continue",
        func: continue_cmd,
        goto_next_cmd: true,
    },
    Cmd {
        command: "b",
        func: break_cmd,
        goto_next_cmd: false,
    },
    Cmd {
        command: "break",
        func: break_cmd,
        goto_next_cmd: false,
    },
];

pub struct Debugger {
    state: DebuggerState,
    breakpoints: HashSet<BusWidth>,
    commands: HashMap<&'static str, &'static Cmd>,
}

impl Debugger {
    pub fn new() -> Debugger {
        let mut commands = HashMap::new();
        for cmd in CMD_LIST {
            commands.insert(cmd.command, cmd);
        }
        Debugger {
            state: DebuggerState::Paused,
            breakpoints: HashSet::new(),
            commands: commands,
        }
    }

    fn parse_input<'a, I>(&self, cmd_iter: I) -> Option<&Cmd>
    where
        I: IntoIterator<Item = &'a str>
    {
        let cmd = cmd_iter.into_iter().next()?;
        Some(self.commands.get(&cmd)?)
    }

    pub fn tick(&mut self, cpu: &mut Cpu) {
        if self.state == DebuggerState::Running {
            // XXX Add ability to break out of running state
            if self.breakpoints.contains(&cpu.regs.get_pc()) {
                self.state = DebuggerState::Paused;
            } else {
                return;
            }
        }

        let mut input = String::new();
        let mut exit_loop = false;
        while !exit_loop {
            print!("gbdb=> ");
            io::stdout().flush().expect("Could not flush stdout");
            io::stdin().read_line(&mut input).expect("Could not read line");
            let mut cmd_iter = input.split_whitespace();
            match self.parse_input(cmd_iter.clone()) {
                Some(cmd) => {
                    if cmd.goto_next_cmd {
                        exit_loop = true;
                    }
                    cmd_iter.next().unwrap(); // skip initial command
                    (cmd.func)(self, cpu, cmd_iter);
                },
                None => println!("Could not parse command"),
            }
            input.clear();
        }
    }
}
