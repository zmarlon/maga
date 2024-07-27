use crate::generation::type_registry::TypeRegistry;
use crate::generation::{CodeGenError, Context, Module};
use crate::parser::Function;
use llvm_sys::core::{LLVMAddFunction, LLVMFunctionType};
use llvm_sys::prelude::LLVMTypeRef;
use std::ffi::CString;

pub fn generate_function(
    context: &Context,
    type_registry: &TypeRegistry,
    module: &mut Module,
    function: &Function,
) -> Result<(), CodeGenError> {
    let function_type = make_function_type(type_registry, function)?;
    let name = CString::new(function.name.as_str())?;

    unsafe { LLVMAddFunction(**module, name.as_ptr(), function_type) };

    Ok(())
}

fn make_function_type(
    type_registry: &TypeRegistry,
    function: &Function,
) -> Result<LLVMTypeRef, CodeGenError> {
    let mut args = function
        .params
        .iter()
        .map(|arg| {
            Ok(type_registry
                .get(&arg.r#type)
                .ok_or(CodeGenError::InvalidType(arg.r#type.name.clone()))?
                .type_ref)
        })
        .collect::<Result<Vec<_>, CodeGenError>>()?;

    let return_type = type_registry
        .get(&function.return_type)
        .ok_or(CodeGenError::InvalidType(function.return_type.name.clone()))?
        .type_ref;

    let function_type =
        unsafe { LLVMFunctionType(return_type, args.as_mut_ptr(), args.len() as _, 0) };
    if function_type.is_null() {
        Err(CodeGenError::LLVMError)
    } else {
        Ok(function_type)
    }
}
