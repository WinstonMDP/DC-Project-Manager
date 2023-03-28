use execute::shell;
use std::{fs, thread};

fn fn_to_thread(filepath: &str) -> impl FnOnce() + '_ {
    move || {
        println!(
            "{}",
            String::from_utf8(
                shell(format!(
                    "echo 161281 | su -c 'docker run --rm archlinux bash -c \"{}\"' MDPDockerUser",
                    fs::read_to_string(filepath).unwrap()
                ))
                .output()
                .unwrap()
                .stdout,
            )
            .unwrap()
        )
    }
}

fn main() {
    let handle_1 = thread::spawn(fn_to_thread("src/before.action"));
    let handle_2 = thread::spawn(fn_to_thread("src/before.action"));
    //let handle_3 = thread::spawn(fn_to_thread("echo 'No'"));
    handle_1.join().unwrap();
    handle_2.join().unwrap();
    //handle_3.join().unwrap();
}
