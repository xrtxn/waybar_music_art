use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

fn main() {
    let stdout = Command::new("playerctl")
        .args(("--follow -p audacious metadata mpris:artUrl").split(" "))
        .stdout(Stdio::piped())
        .spawn()
        .unwrap()
        .stdout
        .ok_or_else(|| "Could not capture standard output.")
        .unwrap();

    let reader = BufReader::new(stdout);

    reader
        .lines()
        .filter_map(|line| line.ok())
        .for_each(|line| {
            println!("{}", line.trim_end().replace("file://", ""));
        });
}
