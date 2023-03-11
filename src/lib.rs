use execute::shell;
use std::fs;

pub fn output(filepath: &str) -> String {
    String::from_utf8(
        shell(fs::read_to_string(filepath).unwrap())
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string_out_of_after_output() {
        let filepath = "tests/after_output_test.action";
        fs::write(filepath, "").unwrap();
    }

    #[test]
    fn right_output() {
        let filepath = "tests/action.action";
        fs::write(filepath, "echo 'Hey'").unwrap();
        assert_eq!("Hey\n", output(filepath));
    }
}
