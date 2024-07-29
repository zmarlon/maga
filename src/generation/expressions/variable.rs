use crate::generation::expressions::{CodeGenExpr, ExpressionCodeGen};
use crate::generation::{CodeGenContext, CodeGenError, Module};
use crate::parser::{Function, VariableExpression};
use llvm_sys::LLVMBasicBlock;

impl ExpressionCodeGen for VariableExpression {
    fn generate(
        &self,
        context: &mut CodeGenContext,
        module: &mut Module,
        function: &Function,
        block: LLVMBasicBlock,
    ) -> Result<CodeGenExpr, CodeGenError> {
        Ok(*context
            .variables
            .get(&self.name)
            .ok_or(CodeGenError::InvalidVariable(self.name.clone()))?)
    }
}
