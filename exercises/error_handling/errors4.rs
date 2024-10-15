// errors4.rs
//
// Execute `rustlings hint errors4` or use the `hint` watch subcommand for a
// hint.



#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        if value <= 0 {
            if value < 0 {
                Err(CreationError::Negative) // 如果值是负数，返回 `Negative` 错误
            } else {
                Err(CreationError::Zero) // 如果值是零，返回 `Zero` 错误
            }
        } else {
            Ok(PositiveNonzeroInteger(value as u64)) // 如果值是正数且非零，则成功创建实例
        }
    }
}

#[test]
fn test_creation() {
    assert!(PositiveNonzeroInteger::new(10).is_ok());
    assert_eq!(
        Err(CreationError::Negative),
        PositiveNonzeroInteger::new(-10)
    );
    assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
}
