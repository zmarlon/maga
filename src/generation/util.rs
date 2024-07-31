use lazy_static::lazy_static;
use llvm_sys::core::LLVMBuildAdd;
use llvm_sys::prelude::{LLVMBuilderRef, LLVMValueRef};
use std::collections::HashMap;
use std::ffi::c_char;

fn add_int(
    builder: LLVMBuilderRef,
    lhs: LLVMValueRef,
    rhs: LLVMValueRef,
    name: *const c_char,
) -> LLVMValueRef {
    unsafe { LLVMBuildAdd(builder, lhs, rhs, name) }
}

lazy_static! {
    static ref ADD_MAP: HashMap<String, fn(LLVMBuilderRef, LLVMValueRef, LLVMValueRef, *const c_char) -> LLVMValueRef> = {
        let mut map = HashMap::new();

        map.insert("u8".to_owned(), add_int);
        map.insert("i8".to_owned(), add_int);

        map
    };
}
