// tests6.rs
//
// In this example we take a shallow dive into the Rust standard library's
// unsafe functions. Fix all the question marks and todos to make the test
// pass.
//
// Execute `rustlings hint tests6` or use the `hint` watch subcommand for a
// hint.


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
  
    //let mut ret: Box<Foo> = unsafe { ??? };
    //todo!("The rest of the code goes here")

    let mut ret: Box<Foo> = unsafe { Box::from_raw(ptr) };
    ret.b = Some("hello".to_owned());
    ret
    
}

/*
Box<T> 是一种堆分配的智能指针，用于拥有一个类型为 T 的值，并在其不再需要时负责释放内存。
       Box 提供了 into_raw() 方法和 from_raw() 函数，用于在 Box 和裸指针之间进行转换。

    a. Box::into_raw()
       into_raw() 方法将 Box<T> 转换为一个裸指针 *mut T，释放了对堆上内存的所有权，而不会运行 T 的析构函数。
       这个函数的主要用途是将 Rust 的所有权转移给 C 或其他不受 Rust 生命周期管理的代码。在这种情况下，负责
       释放内存的代码必须手动调用适当的析构函数。    
       注意：使用 into_raw() 后，必须小心处理裸指针，确保在使用完毕后正确释放内存，以避免内存泄漏或悬垂指针。

    b. Box::from_raw()
       from_raw() 是一个函数，接受一个裸指针 *mut T，并将其转换为一个 Box<T>。
       这个函数用于接管由 into_raw() 转换的裸指针，并将其重新封装为 Box，以便进行后续的 Rust 内存管理。
       注意：使用 from_raw() 时，需要注意确保裸指针是有效的、未被释放或重复释放的，以避免出现未定义的行为。

       需要注意的是，使用 into_raw() 和 from_raw() 需要小心操作，因为它们涉及到裸指针和 Rust 的内存管理。
       在使用这些函数时，务必遵循安全性规则，确保正确管理内存以避免悬垂指针、空指针解引用或重复释放等问题。

*/

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_success() {
        let data = Box::new(Foo { a: 1, b: None });

        // 注：ptr_1 在 Box::into_raw(data) 移动所有权后依然有效，
        // 因转换后 ptr_1 最终为 usize 类型，可复制
        let ptr_1 = &data.a as *const u128 as usize;
   
        // 注意: Box::into_raw(data) 会移动走 data 的所有权
        // SAFETY: We pass an owned box of `Foo`.
        let ret = unsafe { raw_pointer_to_box(Box::into_raw(data)) };

        let ptr_2 = &ret.a as *const u128 as usize;

        assert!(ptr_1 == ptr_2);
        assert!(ret.b == Some("hello".to_owned()));
    }
}
