// tests5.rs
//
// An `unsafe` in Rust serves as a contract.

// When `unsafe` is marked on an item declaration, such as a function,
// a trait or so on, it declares a contract alongside it. However,
// the content of the contract cannot be expressed only by a single keyword.
// Hence, its your responsibility to manually state it in the `# Safety`
// section of your documentation comment on the item.

// When `unsafe` is marked on a code block enclosed by curly braces,
// it declares an observance of some contract, such as the validity of some
// pointer parameter, the ownership of some memory address. However, like
// the text above, you still need to state how the contract is observed in
// the comment on the code block.

// NOTE: All the comments are for the readability and the maintainability of
// your code, while the Rust compiler hands its trust of soundness of your
// code to yourself! If you cannot prove the memory safety and soundness of
// your own code, take a step back and use safe code instead!
//
// 在 Rust 中，unsafe 关键字充当了一种契约。

// 当 unsafe 被标记在函数、trait 等声明上时，它不仅声明了该项与之相关的契约。
// 然而，仅靠一个关键字并不能完全表达契约的内容。因此，你需要在该项的文档注释中的 
// # Safety 部分手动说明该契约的具体内容。

// 当 unsafe 被标记在用大括号括起来的代码块上时，
// 它声明了某些契约的遵循，例如某个指针参数的有效性或某个内存地址的所有权。
// 然而，正如上文所述，你仍然需要在代码块的注释中说明如何遵守该契约。

// 注意：所有的注释都是为了提升代码的可读性和可维护性，
// 而 Rust 编译器则将代码的健全性（soundness）交由你自己负责！
// 如果你无法证明代码在内存安全性和健全性方面是正确的，那就后退一步，使用安全代码代替吧！
// Execute `rustlings hint tests5` or use the `hint` watch subcommand for a
// hint.



/// # Safety
///
/// The `address` must contain a mutable reference to a valid `u32` value.
unsafe fn modify_by_address(address: usize) {
    // TODO: Fill your safety notice of the code block below to match your
    // code's behavior and the contract of this function. You may use the
    // comment of the test below as your format reference.
    // 待办事项：填写下面代码块的安全注释，以匹配代码的行为和该函数的契约。
    // 你可以参考测试中的注释格式。
    unsafe {
        let ptr = address as *mut u32;
        *ptr = 0xAABBCCDD;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let mut t: u32 = 0x12345678;
        // SAFETY: The address is guaranteed to be valid and contains
        // a unique reference to a `u32` local variable.
        unsafe { modify_by_address(&mut t as *mut u32 as usize) };
        assert!(t == 0xAABBCCDD);
    }
}
