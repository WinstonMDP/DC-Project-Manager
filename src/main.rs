use execute::shell;
use std::io::Error;
use std::process::Output;
use std::{fs, thread};

enum Action {
    Before,
    After,
}

fn action_closure<'a>(action: Action) -> impl FnOnce() -> Result<Output, Error> + 'a {
    let filepath = "src/".to_owned()
        + match action {
            Action::Before => "before",
            Action::After => "after",
        }
        + ".action";
    move || {
        shell(format!(
            "echo 161281 | su -c 'docker run --rm archlinux bash -c \"{}\"' MDPDockerUser",
            fs::read_to_string(filepath).unwrap()
        ))
        .output()
    }
}

fn main() {
    let handle_1 = thread::spawn(action_closure(Action::Before));
    let handle_2 = thread::spawn(action_closure(Action::Before));
    let handle_3 = thread::spawn(action_closure(Action::After));
    handle_1.join().unwrap().unwrap();
    handle_2.join().unwrap().unwrap();
    handle_3.join().unwrap().unwrap();
}
