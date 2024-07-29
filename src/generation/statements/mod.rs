use crate::generation::{CodeGenContext, CodeGenError, Module};
use crate::parser::Function;
use llvm_sys::LLVMBasicBlock;

mod assignment;
mod r#return;

pub trait StatementCodeGen {
    fn generate(
        &self,
        context: &mut CodeGenContext,
        module: &mut Module,
        function: &Function,
        block: LLVMBasicBlock,
    ) -> Result<(), CodeGenError>;
}
