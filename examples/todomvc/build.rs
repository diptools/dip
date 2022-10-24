use std::process::Command;

fn main() {
    // quick and dirty way to compile tailwind css on each run
    let status = Command::new("npm")
        .args(["run", "todomvc:css"])
        .status()
        .expect("failed to execute process");
    println!("{}", status);
}
