use futures_lite::future;
use std::process;

fn main() {
    return future::block_on(async {
        let cmd = process::Command::new("git")
            .args(["push", "origin", "main"])
            .stderr(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stdin(std::process::Stdio::null())
            .output()
            .expect("\x1B[41;1m\x1B[37;1m ERROR: \x1B[0m\x1B[0m failed to run command");

        println!("{}", String::from_utf8_lossy(&cmd.stderr));
        println!("{}", String::from_utf8_lossy(&cmd.stdout));
    });
}
