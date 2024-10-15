// quiz2.rs
//
// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Let's build a little machine in the form of a function. As input, we're going
// to give a list of strings and commands. These commands determine what action
// is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
// The exact form of this will be:
// - The input is going to be a Vector of a 2-length tuple,
//   the first element is the string, the second one is the command.
// - The output element is going to be a Vector of strings.
//
// No hints this time!


pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

pub mod my_module{
    // 使用 super::Command 来引用上级模块中的 Command 枚举
    use super::Command;
    pub fn transformer(input: Vec<(String, super::Command)>) -> Vec<String> {
        let mut output: Vec<String> = vec![];
        for (string, command) in input {
            match command {
                super::Command::Uppercase => output.push(string.to_uppercase()),
                super::Command::Trim => output.push(string.trim().to_string()),
                super::Command::Append(times) => {
                    let mut new_string = string.clone();
                    for _ in 0..times {
                        new_string.push_str("bar");
                    }
                    output.push(new_string);
                }
            }
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use crate::my_module;
    // 使用完整的路径引用 transformer 函数和 Command 枚举
    fn transformer(input: Vec<(String, super::Command)>) -> Vec<String> {
        my_module::transformer(input)
    }

    use super::Command; // 仍然需要使用 use 来引用上级模块中的 Command 枚举

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello".to_string(), Command::Uppercase),
            (" all roads lead to rome! ".to_string(), Command::Trim),
            ("foo".to_string(), Command::Append(1)),
            ("bar".to_string(), Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}
