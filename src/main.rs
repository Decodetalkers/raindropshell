use std::{
    io::{
        self, Read, Write
    }, 
    process::{
        self,
        Command,
    }, 
    sync::mpsc,
    thread
};
mod tool;
use tool::ascii_to_char;
fn child_process(command: String,tx: mpsc::Sender<bool>){
    let mut child = Command::new(command.as_str())
        .stdin(process::Stdio::piped())
        .stdout(process::Stdio::piped())
        .spawn()
        .expect("error");

    let mut stream = child.stdout.take().expect("error");
    let mut inputstream = child.stdin.take().unwrap();
    
    //let (tx2,rx2):(mpsc::Sender<bool>,mpsc::Receiver<bool>) = mpsc::channel();
    //let tx3 = tx2.clone();
    thread::Builder::new()
        .name("input".into())
        .spawn(move||{
            loop{
                let mut input = String::new();
                //println!("end2");
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");
                println!("buffer {}",input);
                match inputstream.write_all(input.as_bytes()){
                    Ok(_)=>{},
                    Err(_)=>{
                        break;
                    },
                }

            }
        }).expect("error");

    thread::Builder::new()
        .name(command)
        .spawn(move||{
            //如果后面没有需要家的信息，首先会等进程结束
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
            //println!("end");
            if tx.send(true).is_ok(){};
            //thread::sleep(Duration::from_millis(100));
            io::stdout().write(b"ssss").expect("sss");
            io::copy(&mut io::stdin(), &mut io::stdout()).expect("sss");
            //if tx2.send(false).is_ok(){};
        }).expect("error");
}
fn main() {
    loop{
        let (tx,rx) = mpsc::channel();
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        guess.pop();
        child_process(guess.to_string(),tx);
        if let Ok(test) = rx.recv(){
            if test{
                continue;
            }
        }
    }
}
