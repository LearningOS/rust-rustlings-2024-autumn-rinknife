use std::env;
use std::time::SystemTime;

fn main() {
    // 获取当前时间戳
    
    let current_timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    // 检查环境变量 `TEST_FOO`
    match env::var("TEST_FOO") {
        Ok(value) => {
            // 尝试将 `TEST_FOO` 的值解析为 u64
            match value.parse::<u64>() {
                Ok(timestamp) => {
                    // 确保时间戳在当前时间戳的前10秒内
                    if current_timestamp >= timestamp && current_timestamp < timestamp + 10 {
                        // 如果时间戳有效，使用 `println!` 输出它
                        println!("cargo:rustc-env=TEST_FOO={}", timestamp);
                    } else {
                        // 如果时间戳不在范围内，则生成错误
                        panic!("TEST_FOO is not within the expected range.");
                    }
                }
                Err(_) => {
                    // 如果解析失败，则生成错误
                    panic!("TEST_FOO is not a valid u64.");
                }
            }
        }
        Err(_) => {
            // 如果环境变量不存在，则生成错误
            panic!("TEST_FOO environment variable is not set.");
        }
    }
}
