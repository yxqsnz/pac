use std::io::Result;
use std::process::{self, ExitStatus};
use std::{
    env,
    process::{Command, Stdio},
};
fn exit(e: ExitStatus) {
    process::exit(e.code().unwrap_or(0));
}
fn main() -> Result<()> {
    let mut args = env::args();
    args.next();
    match args.next() {
        Some(it) => match it.as_str() {
            "add" => {
                exit(
                    Command::new("pacman")
                        .arg("-S")
                        .args(args)
                        .stdin(Stdio::inherit())
                        .stdout(Stdio::inherit())
                        .stderr(Stdio::inherit())
                        .spawn()?
                        .wait()?,
                );
            }
            "del" => {
                exit(
                    Command::new("pacman")
                        .arg("-Rcnsu")
                        .args(args)
                        .stdin(Stdio::inherit())
                        .stdout(Stdio::inherit())
                        .stderr(Stdio::inherit())
                        .spawn()?
                        .wait()?,
                );
            }
            "up" => {
                exit(
                    Command::new("pacman")
                        .arg("-Sy")
                        .args(args)
                        .stdin(Stdio::inherit())
                        .stdout(Stdio::inherit())
                        .stderr(Stdio::inherit())
                        .spawn()?
                        .wait()?,
                );
            }
            "os-up" => {
                exit(
                    Command::new("pacman")
                        .arg("-Syu")
                        .args(args)
                        .stdin(Stdio::inherit())
                        .stdout(Stdio::inherit())
                        .stderr(Stdio::inherit())
                        .spawn()?
                        .wait()?,
                );
            }
            "find" => {
                exit(
                    Command::new("pacman")
                        .arg("-Ss")
                        .args(args)
                        .stdin(Stdio::inherit())
                        .stdout(Stdio::inherit())
                        .stderr(Stdio::inherit())
                        .spawn()?
                        .wait()?,
                );
            }            it => eprintln!("unknown action {it}"),
        },
        None => println!("usage: pac <add,del,up,os-up,find>"),
    }
    Ok(())
}
