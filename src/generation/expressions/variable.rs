use crate::generation::expressions::{CodeGenExpr, ExpressionCodeGen, ExpressionCodeGenParams};
use crate::generation::CodeGenError;
use crate::parser::VariableExpression;

impl ExpressionCodeGen<'_> for VariableExpression {
    fn generate(&self, params: &ExpressionCodeGenParams) -> Result<CodeGenExpr, CodeGenError> {
        Ok(params
            .context
            .variables
            .get(&self.name)
            .ok_or(CodeGenError::InvalidVariable(self.name.clone()))?
            .clone())
    }
}
