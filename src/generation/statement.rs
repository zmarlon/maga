use crate::generation::statements::StatementCodeGen;
use crate::generation::{CodeGenContext, CodeGenError, Module};
use crate::parser::{Function, Statement};
use llvm_sys::LLVMBasicBlock;

impl StatementCodeGen for Statement {
    fn generate(
        &self,
        context: &mut CodeGenContext,
        module: &mut Module,
        function: &Function,
        block: LLVMBasicBlock,
    ) -> Result<(), CodeGenError> {
        match self {
            Statement::Return(stat) => {}
            Statement::Declaration(stat) => {}
        }

        Ok(())
    }
}
