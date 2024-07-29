use crate::generation::expressions::{CodeGenExpr, ExpressionCodeGen};
use crate::generation::{CodeGenContext, CodeGenError, Module};
use crate::parser::{ConstantExpression, ConstantExpressionValue, Function, Type};
use llvm_sys::core::LLVMConstInt;
use llvm_sys::LLVMBasicBlock;

impl ExpressionCodeGen for ConstantExpression {
    fn generate(
        &self,
        context: &mut CodeGenContext,
        module: &mut Module,
        function: &Function,
        block: LLVMBasicBlock,
    ) -> Result<CodeGenExpr, CodeGenError> {
        unsafe {
            match self.value {
                ConstantExpressionValue::Int(val) => {
                    let ty = context
                        .type_registry
                        .get(&Type {
                            is_pointer: false,
                            name: "i64".to_owned(),
                        })
                        .unwrap();
                    Ok(CodeGenExpr {
                        val: LLVMConstInt(ty.type_ref, val as _, 0),
                        r#type: ty,
                    })
                }
                ConstantExpressionValue::UInt(val) => {
                    let ty = context
                        .type_registry
                        .get(&Type {
                            is_pointer: false,
                            name: "i64".to_owned(),
                        })
                        .unwrap();
                    Ok(CodeGenExpr {
                        val: LLVMConstInt(ty.type_ref, val as _, 0),
                        r#type: ty,
                    })
                }
            }
        }
    }
}
