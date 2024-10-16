// tests6.rs
//
// In this example we take a shallow dive into the Rust standard library's
// unsafe functions. Fix all the question marks and todos to make the test
// pass.
//
// Execute `rustlings hint tests6` or use the `hint` watch subcommand for a
// hint.


// tests6.rs

struct Foo {
    a: u128,
    b: Option<String>,
}

/// # Safety
///
/// The `ptr` must contain an owned box of `Foo`.
unsafe fn raw_pointer_to_box(ptr: *mut Foo) -> Box<Foo> {
    // SAFETY: The `ptr` contains an owned box of `Foo` by contract. We
    // simply reconstruct the box from that pointer.
    // 使用Box::from_raw来从裸指针重建Box
    let mut ret: Box<Foo> = unsafe { Box::from_raw(ptr) };
    
    // 设置b字段为"hello"
    ret.b = Some("hello".to_owned());
    
    // 返回重建的Box
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_success() {
        let data = Box::new(Foo { a: 1, b: None });

        let ptr_1 = &data.a as *const u128 as usize;
        // SAFETY: We pass an owned box of `Foo`.
        // 使用Box::into_raw将Box转换为裸指针，然后传递给raw_pointer_to_box
        let ret = unsafe { raw_pointer_to_box(Box::into_raw(data)) };

        let ptr_2 = &ret.a as *const u128 as usize;

        // 检查原始指针和新Box的指针是否相同
        assert_eq!(ptr_1, ptr_2);
        // 检查b字段是否被设置为"hello"
        assert_eq!(ret.b, Some("hello".to_owned()));
    }
}
