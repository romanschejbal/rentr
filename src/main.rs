//! A utility for running arbitrary commands when files change.
//! Basically a Rust version of [entr](https://github.com/eradman/entr).
//!
//! # Usage example
//! ```
//! echo "my-folder" | rentr -c -r echo "my-folder has changed"
//! ```
use notify::{
    event::{Event, EventKind, ModifyKind},
    RecommendedWatcher, RecursiveMode, Result, Watcher,
};
use std::io::{self, Read};
use std::process::{Child, Command, Stdio};
use std::sync::{mpsc::channel, Mutex};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "rentr", about = "An example of StructOpt usage.")]
struct Opt {
    /// Clear the screen before running commands.
    #[structopt(short, long)]
    clear: bool,

    /// Reload a persistent child process.
    #[structopt(short, long)]
    reset: bool,

    /// Command to run.
    command: String,

    /// Optional number of arguments for provided command.
    args: Vec<String>,
}

/// Main program
fn main() -> Result<()> {
    let opt = Opt::from_args();

    if opt.clear {
        clear_screen();
    }
    let command_process = Mutex::new(run_command(&opt.command, &opt.args));

    let (tx, rx) = channel();

    let mut watcher: RecommendedWatcher = Watcher::new_immediate(move |res| {
        tx.send(res).unwrap();
    })?;

    let paths_to_watch = read_paths_to_watch();
    paths_to_watch.iter().for_each(|path| {
        watcher
            .watch(path, RecursiveMode::Recursive)
            .expect(&format!("Couldn't set a watch for {}", path));
    });

    loop {
        match rx.recv() {
            Ok(Ok(Event {
                kind: EventKind::Modify(ModifyKind::Data(_)),
                ..
            })) => {
                let mut proc_result = command_process.lock().unwrap();
                if let Ok(proc) = &mut *proc_result {
                    let _ = proc.kill();
                }
                if opt.clear {
                    clear_screen();
                }
                *proc_result = run_command(&opt.command, &opt.args);
            }
            Err(error) => println!("Error: {:?}", error),
            _ => (),
        }
    }
}

/// Reads stdin for paths to watch
fn read_paths_to_watch() -> Vec<String> {
    let mut buffer = String::new();
    let _ = io::stdin().read_to_string(&mut buffer);
    buffer
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

/// Runs the "clear" command
fn clear_screen() {
    let _ = Command::new("clear")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output();
}

/// Runs the provided command
fn run_command(command: &String, args: &Vec<String>) -> std::io::Result<Child> {
    let mut cmd = Command::new(&command);
    cmd.stdout(Stdio::inherit());
    cmd.stderr(Stdio::inherit());
    args.iter().for_each(|arg| {
        cmd.arg(arg);
    });
    cmd.spawn()
}
