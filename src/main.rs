use std::{io::{self, BufRead, Read, Write}, process::{
        self,
        Command,
    }, thread};
mod tool;
use tool::ascii_to_char;
fn child_process(command: String){
    let mut child = Command::new(command.clone())
        .stdin(process::Stdio::piped())
        .stdout(process::Stdio::piped())
        .spawn()
        .expect("error");

    let mut stream = child.stdout.take().expect("error");
    let mut inputstream = child.stdin.take().unwrap();
    
    thread::Builder::new()
        .name("input".into())
        .spawn(move||{
            loop{
                let stdin = io::stdin();
                let mut input = String::new();
                println!("Input");
                for line in stdin.lock().lines() {
                    let line = line.expect("error");
                    input = line;
                    println!("input");
                    break;
                }
                match inputstream.write_all(input.as_bytes()){
                    Ok(_)=>{},
                    Err(_)=>{
                        break;
                    },
                }
            }
        }).expect("error");

    thread::Builder::new()
        .name(command.into())
        .spawn(move||{
            loop {
                let mut buf =[0];
                match stream.read(&mut buf) {
                    Err(err) => {
                        println!("{}]Err happened{}",line!(),err);
                        break;
                    }
                    Ok(got) => {
                        if got == 0 {
                            println!("\n bye");
                            break;
                        } else if got == 1 {
                            let index = ascii_to_char(buf[0]);
                            print!("{}",index);

                        }else{
                            print!("error");
                            break
                        }
                    }
                }
            }
        }).expect("error");

    thread::Builder::new()
        .name("save".into())
        .spawn(move||{
            child.wait().unwrap();
            drop(child);
        }).expect("error");
}
fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.expect("error");
        child_process(line);
    }
}
