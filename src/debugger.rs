mod error;

use std::collections::{HashMap, HashSet};
use std::io::{self, prelude::*};

use itertools::Itertools;

use crate::cpu::Cpu;
use crate::emu_log;
use crate::hw::memory::{Bus, BusWidth};

use self::error::{DebugError, DebugResult};

#[derive(PartialEq)]
enum DebuggerState {
    Running,
    Paused,
}

fn next_cmd(
    _dbgr: &mut Debugger,
    cpu: &mut Cpu,
    _arg_iterator: &mut Iterator<Item = &str>,
) -> DebugResult<()> {
    emu_log!("{:04x?}", cpu.regs);
    emu_log!(
        "Flags: z: {}, c: {}, h: {}, n: {}",
        cpu.regs.get_flag_z(),
        cpu.regs.get_flag_c(),
        cpu.regs.get_flag_h(),
        cpu.regs.get_flag_n()
    );
    Ok(())
}

fn registers_cmd(
    _: &mut Debugger,
    cpu: &mut Cpu,
    _: &mut Iterator<Item = &str>,
) -> DebugResult<()> {
    println!("{:04x?}", cpu.regs);
    println!(
        "Flags: z: {}, c: {}, h: {}, n: {}",
        cpu.regs.get_flag_z(),
        cpu.regs.get_flag_c(),
        cpu.regs.get_flag_h(),
        cpu.regs.get_flag_n()
    );
    Ok(())
}

fn parse_val<S: Into<String>>(string: S) -> DebugResult<BusWidth> {
    let addr_str = string.into();
    if addr_str.starts_with("0x") {
        let trimmed_str = addr_str.trim_start_matches("0x");
        u16::from_str_radix(trimmed_str, 16).map_err(|_| DebugError)
    } else {
        addr_str.parse::<u16>().map_err(|_| DebugError)
    }
}

fn print_cmd(
    _dbgr: &mut Debugger,
    cpu: &mut Cpu,
    args: &mut Iterator<Item = &str>,
) -> DebugResult<()> {
    let start_addr = parse_val(args.next().ok_or(DebugError)?)?;
    let len = parse_val(args.next().unwrap_or("1"))?;

    let mut line_output = Vec::<String>::new();
    for chunk_iter in &(start_addr..=start_addr + len).chunks(0x10) {
        let mut peekable_iter = chunk_iter.peekable();
        let line_start_addr = peekable_iter.peek().ok_or(DebugError)?;
        print!("{:04x}: ", line_start_addr);
        peekable_iter
            .map(|addr| format!("{:02x}", cpu.read8(addr)))
            .for_each(|s| line_output.push(s));
        println!("{}", line_output.join(" "));
        line_output.clear();
    }
    Ok(())
}

fn continue_cmd(
    dbgr: &mut Debugger,
    _: &mut Cpu,
    _: &mut Iterator<Item = &str>,
) -> DebugResult<()> {
    dbgr.state = DebuggerState::Running;
    Ok(())
}

fn break_cmd(
    dbgr: &mut Debugger,
    _: &mut Cpu,
    args: &mut Iterator<Item = &str>,
) -> DebugResult<()> {
    let addr = parse_val(args.next().ok_or(DebugError)?)?;

    dbgr.breakpoints.insert(addr);
    println!("Added breakpoint to 0x{:04x}", addr);
    Ok(())
}

struct Cmd {
    command: &'static str,
    func: fn(&mut Debugger, &mut Cpu, &mut Iterator<Item = &str>) -> DebugResult<()>,
    goto_next_cmd: bool,
}

enum Cmds {
    NextCmd,
    PrintCmd,
    ContinueCmd,
    BreakCmd,
    RegistersCmd,
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
    Cmd {
        command: "r",
        func: registers_cmd,
        goto_next_cmd: false,
    },
    Cmd {
        command: "registers",
        func: registers_cmd,
        goto_next_cmd: false,
    },
];

pub struct Debugger {
    state: DebuggerState,
    breakpoints: HashSet<BusWidth>,
    commands: HashMap<&'static str, &'static Cmd>,
    last_cmd: &'static Cmd,
}

impl Debugger {
    pub fn new() -> Debugger {
        let mut commands = HashMap::new();
        for cmd in CMD_LIST {
            commands.insert(cmd.command, cmd);
        }
        let last_cmd = commands["r"];
        Debugger {
            state: DebuggerState::Paused,
            breakpoints: HashSet::new(),
            commands: commands,
            last_cmd: last_cmd,
        }
    }

    fn parse_input<'a, I>(&self, cmd_iter: I) -> Option<&'static Cmd>
    where
        I: IntoIterator<Item = &'a str>,
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
            io::stdin()
                .read_line(&mut input)
                .expect("Could not read line");
            if input == "\n" {
                if self.last_cmd.goto_next_cmd {
                    exit_loop = true;
                }
                (self.last_cmd.func)(self, cpu, &mut std::iter::empty());
            }
            let mut cmd_iter = input.split_whitespace();
            match self.parse_input(cmd_iter.clone()) {
                Some(cmd) => {
                    if cmd.goto_next_cmd {
                        exit_loop = true;
                    }
                    cmd_iter.next().unwrap(); // skip initial command
                    (cmd.func)(self, cpu, &mut cmd_iter);
                    self.last_cmd = cmd;
                }
                None => println!("Could not parse command"),
            }
            input.clear();
        }
    }
}
