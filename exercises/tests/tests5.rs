// tests5.rs
//
// An `unsafe` in Rust serves as a contract.
//
// When `unsafe` is marked on an item declaration, such as a function,
// a trait or so on, it declares a contract alongside it. However,
// the content of the contract cannot be expressed only by a single keyword.
// Hence, its your responsibility to manually state it in the `# Safety`
// section of your documentation comment on the item.
//
// When `unsafe` is marked on a code block enclosed by curly braces,
// it declares an observance of some contract, such as the validity of some
// pointer parameter, the ownership of some memory address. However, like
// the text above, you still need to state how the contract is observed in
// the comment on the code block.
//
// NOTE: All the comments are for the readability and the maintainability of
// your code, while the Rust compiler hands its trust of soundness of your
// code to yourself! If you cannot prove the memory safety and soundness of
// your own code, take a step back and use safe code instead!
//
// Execute `rustlings hint tests5` or use the `hint` watch subcommand for a
// hint.


// tests5.rs

/// # Safety
///
/// The `address` must be a valid pointer to a `u32` value that is mutable.
/// The caller must ensure that no other references to the same `u32` value exist
/// for the duration of this function call to prevent data races.
unsafe fn modify_by_address(address: *mut u32) {
    // # Safety
    //
    // We have received a raw pointer to a `u32` from the caller and have
    // been promised that it is valid and mutable. We must trust that the
    // caller has not provided an invalid or immutable pointer.
    unsafe {
        // Dereference the pointer to modify the value it points to.
        // We assume that the pointer is valid and points to a `u32` value.
        *address = 0xAABBCCDD;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let mut t: u32 = 0x12345678;
        // SAFETY: The address is guaranteed to be valid and mutable as it is
        // a unique reference to a `u32` local variable. There are no other
        // references to this variable, ensuring exclusive access.
        unsafe { modify_by_address(&mut t as *mut u32) };
        assert_eq!(t, 0xAABBCCDD);
    }
}
