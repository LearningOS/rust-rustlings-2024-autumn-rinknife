// errors5.rs
//
// This program uses an altered version of the code from errors4.
//
// This exercise uses some concepts that we won't get to until later in the
// course, like `Box` and the `From` trait. It's not important to understand
// them in detail right now, but you can read ahead if you like. For now, think
// of the `Box<dyn ???>` type as an "I want anything that does ???" type, which,
// given Rust's usual standards for runtime safety, should strike you as
// somewhat lenient!
//
// In short, this particular use case for boxes is for when you want to own a
// value and you care only that it is a type which implements a particular
// trait. To do so, The Box is declared as of type Box<dyn Trait> where Trait is
// the trait the compiler looks for on any value used in that context. For this
// exercise, that context is the potential errors which can be returned in a
// Result.
//
// What can we use to describe both errors? In other words, is there a trait
// which both errors implement?
//
// Execute `rustlings hint errors5` or use the `hint` watch subcommand for a
// hint.


use std::error;
use std::fmt;
use std::num::ParseIntError;

// TODO: update the return type of `main()` to make this compile.
fn main() -> Result<(), Box<dyn ???>> {
    let pretend_user_input = "42";
    let x: i64 = pretend_user_input.parse()?;
    println!("output={:?}", PositiveNonzeroInteger::new(x)?);
    Ok(())
}

// Don't change anything below this line.

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        // `new` 是一个关联函数，它尝试创建一个新的 `PositiveNonzeroInteger` 实例。
        // 它接收一个 `i64` 类型的参数 `value`，并返回一个 `Result` 类型。
        // `Result` 类型可以是 `Ok(PositiveNonzeroInteger)` 或者 `Err(CreationError)`。

        if value <= 0 {
            // 首先检查 `value` 是否小于或等于零。如果不满足我们的条件（正数且非零），
            // 则需要返回一个错误。

            if value < 0 {
                // 如果 `value` 小于零，则说明它是一个负数。
                // 在这种情况下，我们返回一个 `Err(CreationError::Negative)`，
                // 表示创建失败的原因是传入了负数。
                Err(CreationError::Negative)
            } else {
                // 如果 `value` 等于零，则说明它不是一个正数。
                // 我们返回一个 `Err(CreationError::Zero)`，
                // 表示创建失败的原因是传入了零。
                Err(CreationError::Zero)
            }
        } else {
            // 如果 `value` 大于零，则它是一个正数且非零，满足我们的条件。
            // 我们可以安全地将 `value` 强制转换为 `u64` 类型，并创建一个新的 `PositiveNonzeroInteger` 实例。
            // 然后我们将其包裹在 `Ok` 中返回，表示创建成功。
            Ok(PositiveNonzeroInteger(value as u64))
        }
    }
}


// This is required so that `CreationError` can implement `error::Error`.
impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            CreationError::Negative => "number is negative",
            CreationError::Zero => "number is zero",
        };
        f.write_str(description)
    }
}

impl error::Error for CreationError {}
