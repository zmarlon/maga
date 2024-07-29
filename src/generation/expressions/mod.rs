mod constant;
mod variable;

use crate::generation::type_registry::TypeDef;
use crate::generation::{CodeGenContext, CodeGenError, Module};
use crate::parser::Function;
use llvm_sys::prelude::LLVMValueRef;
use llvm_sys::LLVMBasicBlock;

pub trait ExpressionCodeGen {
    fn generate(
        &self,
        context: &mut CodeGenContext,
        module: &mut Module,
        function: &Function,
        block: LLVMBasicBlock,
    ) -> Result<CodeGenExpr, CodeGenError>;
}

#[derive(Copy, Clone, Debug)]
pub struct CodeGenExpr {
    val: LLVMValueRef,
    r#type: TypeDef,
}
