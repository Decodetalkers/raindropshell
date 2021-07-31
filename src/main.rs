use std::{
    io::Write,
    process::{self, Command},
    sync::mpsc,
    thread,
};
mod tool;
use tool::{
    //ascii_to_char,
    spawn_stdin_channel,
    Input,
};
fn child_process(
    commandin: Input,
    tx: mpsc::Sender<()>,
    rv: mpsc::Receiver<String>,
    rx3: mpsc::Receiver<()>,
) {
    let command = commandin.command;
    let args = commandin.args;
    let childa = Command::new(command.as_str())
        .args(args)
        .stdin(process::Stdio::piped())
        //    .stdout(process::Stdio::piped())
        .spawn();
    //.expect("error");
    let mut child = match childa {
        Ok(child) => child,
        Err(_) => {
            println!("no such command");
            if tx.send(()).is_ok() {};
            return;
        }
    };

    //let mut stream = child.stdout.take().expect("error");
    let mut inputstream = child.stdin.take().unwrap();

    //let (tx2,rx2):(mpsc::Sender<bool>,mpsc::Receiver<bool>) = mpsc::channel();
    //let tx3 = tx2.clone();
    thread::Builder::new()
        .name("input".into())
        .spawn(move || {
            loop {
                //let mut input = String::new();
                let input: String = loop {
                    if let Ok(key) = rv.try_recv() {
                        break key;
                    }
                };
                match inputstream.write_all(input.as_bytes()) {
                    Ok(_) => {}
                    Err(_) => {
                        break;
                    }
                }
            }
        })
        .expect("error");

    //thread::Builder::new()
    //    .name(command)
    //    .spawn(move || {
    //        //如果后面没有需要家的信息，首先会等进程结束
    //        loop {
    //            let mut buf = [0];
    //            match stream.read(&mut buf) {
    //                Err(err) => {
    //                    println!("{}]Err happened{}", line!(), err);
    //                    break;
    //                }
    //                Ok(got) => {
    //                    if got == 0 {
    //                        println!("\n bye");
    //                        break;
    //                    } else if got == 1 {
    //                        let index = ascii_to_char(buf[0]);
    //                        print!("{}", index);
    //                    } else {
    //                        print!("error");
    //                        break;
    //                    }
    //                }
    //            }
    //        }
    //    })
    //    .expect("error");
    //ctrlc::set_handler(move ||{
    //    child.kill();
    //}).expect("error");
    //child.wait().unwrap();
    //drop(child);
    //println!("end");
    //if tx.send(true).is_ok() {};

    thread::Builder::new()
        .name("save".into())
        .spawn(move || {
            //child.wait().unwrap();
            loop {
                if let Ok(()) = rx3.try_recv() {
                    println!("ss");
                    child.kill().expect("ss");
                }
                match child.try_wait() {
                    Ok(Some(_)) => break,
                    Ok(None) => {}
                    Err(_) => {
                        println!("ggg");
                        println!("error");
                    }
                }
            }
            drop(child);
            //println!("end");
            if tx.send(()).is_ok() {};
            //thread::sleep(Duration::from_millis(100));
            //io::stdout().write(b"ssss").expect("sss");
            //io::copy(&mut io::stdin(), &mut io::stdout()).expect("sss");
            //if tx2.send(false).is_ok(){};
        })
        .expect("error");
}
fn main() {
    let (tx0, rx0): (mpsc::Sender<()>, mpsc::Receiver<()>) = mpsc::channel();
    ctrlc::set_handler(move || tx0.send(()).expect("couldnot send")).expect("Err Ctrl-C");

    let stdin_channel = spawn_stdin_channel();
    loop {
        print!("~");
        std::io::stdout().flush().unwrap();
        let (tx, rx) = mpsc::channel();
        if let Ok(()) = rx0.try_recv() {
            break;
        }
        let mut guess: String = loop {
            if let Ok(key) = stdin_channel.try_recv() {
                break key;
            }
        };
        //io::stdin()
        //    .read_line(&mut guess)
        //    .expect("Failed to read line");
        guess.pop();
        let guess2 = Input::new(guess);
        //println!("{}",guess);
        let (tx2, rx2) = mpsc::channel();
        let (tx3, rx3): (mpsc::Sender<()>, mpsc::Receiver<()>) = mpsc::channel();
        child_process(guess2, tx, rx2, rx3);
        loop {
            //用try就不会阻塞了妈的
            //如果command不成立，就break
            if let Ok(()) = rx.try_recv() {
                break;
            }
            //把信号传送到子进程中
            if let Ok(key) = stdin_channel.try_recv() {
                if tx2.send(key).is_ok() {}
            }
            if let Ok(()) = rx0.try_recv() {
                if tx3.send(()).is_ok() {}
            }
        }
    }
}
