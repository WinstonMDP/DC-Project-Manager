use std::env;
use std::process::Stdio;
use std::thread;

fn fn_to_thread(init_command_variable_value: &str) -> impl FnOnce() + '_ {
    move || {
        env::set_var("COMMAND", init_command_variable_value);
        let before_output = action_ddb::output("src/before.action");
        println!("{before_output}");
        // TODO: send before_output to mongodb
        let after_output = action_ddb::output("src/after.action");
        assert_eq!("", after_output);
    }
}

fn fn_to_thread_2(cmd: &str) -> impl FnOnce() + '_ {
    move || {
        println!(
            "{}",
            String::from_utf8(
                execute::shell(format!("{}{}", "docker run --rm archlinux bash {}", cmd))
                    .stderr(Stdio::inherit())
                    .output()
                    .unwrap()
                    .stdout,
            )
            .expect("hey")
        )
    }
}

fn main() {
    let handle_1 = thread::spawn(fn_to_thread_2("echo 'Hey'"));
    let handle_2 = thread::spawn(fn_to_thread_2("echo 'Ya'"));
    let handle_3 = thread::spawn(fn_to_thread_2("echo 'No'"));
    handle_1.join().unwrap();
    handle_2.join().unwrap();
    handle_3.join().unwrap();
    let mut a = Vec::new();
    a.push(1);
}
