use std::{io::{self, Write}, println};
use std::fs;

struct Session {
    filename: String, // filename
    buffer:   String, // buffer that will be saved to file
}

fn append() -> String {
    let mut buf = String::new();

    let mut cline = String::new();

    loop {
        io::stdout().flush().expect("fuck");

        io::stdin()
            .read_line(&mut cline)
            .expect("Could not read line.");

        cline = String::from(cline.trim());

        if cline == "..d" {
            return buf.trim().to_string();
        }

        buf.push_str(cline.as_str());
        buf.push('\n');

        cline = String::new();
    }
}

fn list_buf(buf: &Vec<&str>) {
    let j = buf.len() + 1;
    for i in 1..j {
        println!("{}  {}", i, buf[i - 1]);
    }
}

fn main_loop() {

    let mut sessions: Vec<Session> = Vec::new();
    let mut current_session = 0;
    let mut buf: String = String::new();

    let mut com_code: u8 = 0;
    let mut command = String::new();
    let mut temp: String = String::new();

    loop {

        print!("% ");
        io::stdout().flush().expect("fuck");

        io::stdin()
            .read_line(&mut command)
            .expect("Could not read line.");

        let com_args: Vec<&str> = command.split_whitespace().collect();

        match com_args[0] {
            "a" => {
                buf.push_str(append().as_str());
                com_code = 1;
            },
            "i" => {
                temp = append();
                let mut idx: i32 = 0;
                let mut nl_num: i32 = 1;

                let mut out: String = buf.clone();
                for c in buf.chars() {
                    if c == '\n' {
                        nl_num += 1;
                        if nl_num == com_args[1].parse::<i32>().unwrap() {
                            out.insert(idx as usize, '\n');
                            for (i, c) in temp.chars().enumerate() {
                                out.insert(idx as usize + i as usize + 1, c);
                            }
                        }
                    }
                    idx += 1;
                }

                buf = out.clone();

                com_code = 1;
            },
            "n"=> {
                let filename = String::from(com_args[1]);
                let data = fs::read_to_string(filename.as_str()).expect("Unable to read file");
                sessions.push(Session { filename: filename, buffer: data });
                buf = sessions[current_session].buffer.clone();
                com_code = 1;
            },
            "l" => {
                list_buf(&buf.lines().collect());
                com_code = 1;
            },
            "s" => {
                fs::write(sessions[current_session].filename.as_str(),
                    sessions[current_session].buffer.as_str())
                    .expect("Unable to write file");
                com_code = 1;
            }
            "x" => {
                println!("exit");
                com_code = 2;
            },
            _ => com_code = 0,
        }

        if com_code == 2 {
            break;
        } else if com_code == 0 {
            println!("error.");
        }

        command = String::new();
        sessions[current_session].buffer = buf.clone();
    }
}

fn main() {
    println!("JOTCAT v0.1");
    main_loop();
}
