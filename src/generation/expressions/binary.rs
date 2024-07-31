use crate::generation::expressions::{CodeGenExpr, ExpressionCodeGen, ExpressionCodeGenParams};
use crate::generation::CodeGenError;
use crate::parser::{BinaryExpression, Operator};
use llvm_sys::core::{LLVMBuildAdd, LLVMBuildICmp, LLVMBuildMul, LLVMBuildSub};
use llvm_sys::LLVMIntPredicate;

impl ExpressionCodeGen<'_> for BinaryExpression {
    fn generate(&self, params: &ExpressionCodeGenParams) -> Result<CodeGenExpr, CodeGenError> {
        let lhs = self.lhs.generate(params)?;
        let rhs = self.rhs.generate(params)?;

        if lhs.r#type != rhs.r#type {
            return Err(CodeGenError::TypesNotEqual(lhs.r#type, rhs.r#type));
        }

        unsafe {
            match &self.operator {
                Operator::Add => {
                    LLVMBuildAdd(
                        params.builder,
                        lhs.val,
                        rhs.val,
                        b"add_temp\0".as_ptr().cast(),
                    );
                }
                Operator::Sub => {
                    LLVMBuildSub(
                        params.builder,
                        lhs.val,
                        rhs.val,
                        b"sub_temp\0".as_ptr().cast(),
                    );
                }
                Operator::Mul => {
                    LLVMBuildMul(
                        params.builder,
                        lhs.val,
                        rhs.val,
                        b"mul_temp\0".as_ptr().cast(),
                    );
                }
                Operator::Less => {
                    LLVMBuildICmp(
                        params.builder,
                        LLVMIntPredicate::LLVMIntSLE,
                        lhs.val,
                        rhs.val,
                        b"le_temp\0".as_ptr().cast(),
                    );
                }
                token => return Err(CodeGenError::UnsupportedOperator),
            }
        }

        todo!()
    }
}
