mod binary;
mod constant;
mod variable;

use crate::generation::type_registry::TypeDef;
use crate::generation::{CodeGenContext, CodeGenError, Module};
use crate::parser::{Expression, Function};
use llvm_sys::prelude::{LLVMBasicBlockRef, LLVMBuilderRef, LLVMValueRef};

pub trait ExpressionCodeGen<'a> {
    fn generate(&self, params: &ExpressionCodeGenParams<'a>) -> Result<CodeGenExpr, CodeGenError>;
}

pub struct ExpressionCodeGenParams<'a> {
    context: &'a mut CodeGenContext,
    module: &'a mut Module,
    function: &'a Function,
    block: LLVMBasicBlockRef,
    builder: LLVMBuilderRef,
}

#[derive(Clone, Debug)]
pub struct CodeGenExpr {
    val: LLVMValueRef,
    r#type: TypeDef,
}

impl ExpressionCodeGen<'_> for Expression {
    fn generate(&self, params: &ExpressionCodeGenParams) -> Result<CodeGenExpr, CodeGenError> {
        match self {
            Expression::Binary(exp) => exp.generate(params),
            Expression::Unary(_) => {
                todo!()
            }
            Expression::Constant(exp) => exp.generate(params),
            Expression::Variable(exp) => exp.generate(params),
            Expression::Call(exp) => {
                todo!()
            }
        }
    }
}
