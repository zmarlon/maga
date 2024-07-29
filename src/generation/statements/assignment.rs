use crate::generation::statements::StatementCodeGen;
use crate::generation::{CodeGenContext, CodeGenError, Module};
use crate::parser::{AssignmentStatement, Function};
use llvm_sys::LLVMBasicBlock;

impl StatementCodeGen for AssignmentStatement {
    fn generate(
        &self,
        context: &mut CodeGenContext,
        module: &mut Module,
        function: &Function,
        block: LLVMBasicBlock,
    ) -> Result<(), CodeGenError> {
        todo!()
    }
}
