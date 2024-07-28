mod function;
mod type_registry;

use crate::generation::function::generate_function;
use crate::generation::type_registry::TypeRegistry;
use crate::parser::{Element, Parser};
use llvm_sys::core::{LLVMContextCreate, LLVMDumpModule, LLVMModuleCreateWithNameInContext};
use llvm_sys::prelude::*;
use std::ffi::{CString, NulError};
use std::ops::Deref;
use thiserror::Error;

pub struct Context {
    context: LLVMContextRef,
}

impl Deref for Context {
    type Target = LLVMContextRef;

    fn deref(&self) -> &Self::Target {
        &self.context
    }
}

#[derive(Debug, Error)]
pub enum CodeGenError {
    #[error("LLVM error occured")]
    LLVMError,

    #[error("Nul error occured:")]
    NulError(#[from] NulError),

    #[error("Invalid type: {0}")]
    InvalidType(String),
}

impl Context {
    pub fn new() -> Result<Self, CodeGenError> {
        let context = unsafe { LLVMContextCreate() };

        if context.is_null() {
            Err(CodeGenError::LLVMError)
        } else {
            Ok(Self { context })
        }
    }

    pub fn create_module(&mut self, name: &str) -> Result<Module, CodeGenError> {
        let name_cstr = CString::new(name)?;

        let module = unsafe { LLVMModuleCreateWithNameInContext(name_cstr.as_ptr(), self.context) };
        if module.is_null() {
            Err(CodeGenError::LLVMError)
        } else {
            Ok(Module { module })
        }
    }
}

pub struct Module {
    module: LLVMModuleRef,
}

impl Deref for Module {
    type Target = LLVMModuleRef;

    fn deref(&self) -> &Self::Target {
        &self.module
    }
}

pub struct CompileContext {
    context: Context,
    type_registry: TypeRegistry,
}

impl CompileContext {
    pub fn new() -> Result<Self, CodeGenError> {
        let context = Context::new()?;
        let type_registry = TypeRegistry::new(&context);

        Ok(Self {
            context,
            type_registry,
        })
    }

    pub fn generate(&mut self, module: &mut Module, parser: &Parser) -> Result<(), CodeGenError> {
        let source_file = parser.root();

        for element in &source_file.0 {
            match element {
                Element::Function(function) => {
                    generate_function(&self.context, &self.type_registry, module, function)?;
                }
                _ => {}
            }
        }

        unsafe {
            LLVMDumpModule(**module);
        }

        Ok(())
    }

    pub fn context(&self) -> &Context {
        &self.context
    }

    pub fn context_mut(&mut self) -> &mut Context {
        &mut self.context
    }
}
