// macros1.rs
//
// Execute `rustlings hint macros1` or use the `hint` watch subcommand for a
// hint.
// 定义一个名为 `my_macro` 的宏
macro_rules! my_macro {
    // 宏定义的开始部分，这里我们使用 `()` 来匹配空参数列表
    () => {
        // 宏展开时要执行的代码块
        // 这里的代码块将会在调用宏时被插入到代码中
        println!("Check out my macro!");
    };
}

// 主函数，程序的入口点
fn main() {
    // 调用上面定义的宏 `my_macro`
    // 当这个宏被调用时，它将会展开为 `println!("Check out my macro!");`
    // 这行代码将在控制台上打印出宏定义中的字符串
    my_macro!();
}
