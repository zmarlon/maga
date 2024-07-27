use crate::generation::Context;
use crate::parser::Type;
use llvm_sys::core::{
    LLVMDoubleTypeInContext, LLVMFloatTypeInContext, LLVMInt16TypeInContext,
    LLVMInt32TypeInContext, LLVMInt64TypeInContext, LLVMInt8TypeInContext, LLVMPointerType,
    LLVMVoidTypeInContext,
};
use llvm_sys::prelude::LLVMTypeRef;
use std::collections::HashMap;

pub struct TypeRegistry {
    types: HashMap<String, TypeDef>,
}

impl TypeRegistry {
    pub fn new(context: &Context) -> Self {
        let mut types = HashMap::new();
        let context = context.context;

        types.insert(
            "u8".to_owned(),
            TypeDef {
                type_ref: unsafe { LLVMInt8TypeInContext(context) },
                size: 8,
            },
        );
        types.insert(
            "i8".to_owned(),
            TypeDef {
                type_ref: unsafe { LLVMInt8TypeInContext(context) },
                size: 8,
            },
        );

        types.insert(
            "u16".to_owned(),
            TypeDef {
                type_ref: unsafe { LLVMInt16TypeInContext(context) },
                size: 16,
            },
        );
        types.insert(
            "i16".to_owned(),
            TypeDef {
                type_ref: unsafe { LLVMInt16TypeInContext(context) },
                size: 16,
            },
        );

        types.insert(
            "u32".to_owned(),
            TypeDef {
                type_ref: unsafe { LLVMInt32TypeInContext(context) },
                size: 32,
            },
        );
        types.insert(
            "i32".to_owned(),
            TypeDef {
                type_ref: unsafe { LLVMInt32TypeInContext(context) },
                size: 32,
            },
        );

        types.insert(
            "u64".to_owned(),
            TypeDef {
                type_ref: unsafe { LLVMInt64TypeInContext(context) },
                size: 64,
            },
        );
        types.insert(
            "i64".to_owned(),
            TypeDef {
                type_ref: unsafe { LLVMInt64TypeInContext(context) },
                size: 64,
            },
        );

        types.insert(
            "f32".to_owned(),
            TypeDef {
                type_ref: unsafe { LLVMFloatTypeInContext(context) },
                size: 32,
            },
        );
        types.insert(
            "f64".to_owned(),
            TypeDef {
                type_ref: unsafe { LLVMDoubleTypeInContext(context) },
                size: 64,
            },
        );

        types.insert(
            "()".to_owned(),
            TypeDef {
                type_ref: unsafe { LLVMVoidTypeInContext(context) },
                size: 0,
            },
        );

        types.insert(
            "bool".to_owned(),
            TypeDef {
                type_ref: unsafe { LLVMInt8TypeInContext(context) },
                size: 8,
            },
        );

        Self { types }
    }

    pub fn get(&self, r#type: &Type) -> Option<TypeDef> {
        self.types.get(&r#type.name).map(|def| {
            if r#type.is_pointer {
                TypeDef {
                    type_ref: unsafe { LLVMPointerType(def.type_ref, def.size as _) },
                    size: 64,
                }
            } else {
                *def
            }
        })
    }
}

#[derive(Copy, Clone, Debug)]
pub struct TypeDef {
    pub type_ref: LLVMTypeRef,
    pub size: usize,
}
