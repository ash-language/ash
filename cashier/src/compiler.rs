use std::{ffi::c_char, fmt};

use ash_core::prelude::*;
use llvm_sys::{
    core::{
        LLVMAddFunction, LLVMFloatType, LLVMFloatTypeInContext, LLVMFunctionType,
        LLVMInt1TypeInContext, LLVMInt32TypeInContext, LLVMVoidTypeInContext, LLVMGetParams, LLVMGetParam, LLVMSetValueName2, LLVMAppendBasicBlockInContext, LLVMPositionBuilderAtEnd, LLVMBuildRetVoid, LLVMBuildRet,
    },
    prelude::{LLVMBuilderRef, LLVMContextRef, LLVMModuleRef, LLVMTypeRef, LLVMValueRef}, LLVMValue,
};

pub struct Compiler<'a> {
    ctx: LLVMContextRef,
    builder: LLVMBuilderRef,
    module: LLVMModuleRef,
    src: &'a cash::Header,
    inst_offset: usize,
    str_offset: usize,
    data_offset: usize,
}

impl<'a> Compiler<'a> {
    pub fn new(
        src: &'a cash::Header,
        ctx: LLVMContextRef,
        builder: LLVMBuilderRef,
        module: LLVMModuleRef,
    ) -> Self {
        Self {
            ctx,
            src,
            builder,
            module,
            inst_offset: 0,
            str_offset: 0,
            data_offset: 0,
        }
    }

    pub fn compile(&mut self) {
        while let Some(inst) = self.read_inst() {
            self.compile_inst(inst);
        }
    }

    fn compile_inst(&mut self, inst: cash::Inst) -> LLVMValueRef {
        use cash::Inst;
        match inst {
            Inst::Fun {
                params_len,
                body_len,
            } => self.compile_fun(params_len as usize, body_len as usize),
            Inst::Ret => self.compile_ret(),
            Inst::Call { arg_len } => todo!(),
            Inst::Block { len } => todo!(),
            Inst::Var => todo!(),
            Inst::Sum => todo!(),
            Inst::Sub => todo!(),
            Inst::Mul => todo!(),
            Inst::Div => todo!(),
            Inst::Rem => todo!(),
            Inst::Eq => todo!(),
            Inst::Neq => todo!(),
            Inst::Gt => todo!(),
            Inst::Lt => todo!(),
            Inst::Gte => todo!(),
            Inst::Lte => todo!(),
            Inst::LogicAnd => todo!(),
            Inst::LogicOr => todo!(),
            Inst::Not => todo!(),
            Inst::Neg => todo!(),
            Inst::I32(_) => todo!(),
            Inst::F64(_) => todo!(),
            Inst::Bool(_) => todo!(),
            Inst::String => todo!(),
            Inst::VarDecl(_) => todo!(),
            Inst::Assign => todo!(),
            Inst::Loop { len } => todo!(),
            Inst::Repeat => todo!(),
            Inst::Branch(_, _) => todo!(),
            Inst::Break => todo!(),
            Inst::None => unreachable!(),
        }
    }

    fn compile_fun(&mut self, params_len: usize, body_len: usize) -> LLVMValueRef {
        let name = self.read_string();
        println!("Compiling function: {name}");

        let fun_ty = self.read_type();
        let mut param_names = Vec::with_capacity(params_len as usize);
        let mut param_types = Vec::with_capacity(params_len as usize);

        // Params
        for _ in 0..params_len {
            let (param_name, param_ty) = self.read_typed_field();
            param_names.push(param_name);
            param_types.push(param_ty);
        }

        let fun = unsafe {
            let fun_ty = LLVMFunctionType(fun_ty, param_types.as_mut_ptr(), params_len as u32, 0);
            let fun = LLVMAddFunction(self.module, name.llvm_str(), fun_ty);
            
            for (i, name) in param_names.iter().enumerate() {
                let param = LLVMGetParam(fun, i as u32);
                // Names can't contain null bytes hence len - 1
                LLVMSetValueName2(param, name.llvm_str(), name.len()-1)
            }

            let block = LLVMAppendBasicBlockInContext(self.ctx, fun, RawStr("entry".as_bytes()).llvm_str());
            LLVMPositionBuilderAtEnd(self.builder, block);

            fun
        };
        

        for _ in 0..body_len {
            let stmt = self.read_inst().expect("expected function statement");
            self.compile_inst(stmt);
        }

        fun
    }

    fn compile_ret(&mut self) -> LLVMValueRef {
        let value = self.read_inst().expect("expected ret value");
        if matches!(value, cash::Inst::None) {
            unsafe {
                return LLVMBuildRetVoid(self.builder);
            }
        }

        let value = self.compile_inst(value);
        unsafe {
            return LLVMBuildRet(self.builder, value);
        }
    }

    fn lower_type(&mut self, ty: cash::Ty) -> LLVMTypeRef {
        unsafe {
            let ty = match ty {
                cash::Ty::String => todo!(),
                cash::Ty::I32 => LLVMInt32TypeInContext(self.ctx),
                cash::Ty::F64 => LLVMFloatTypeInContext(self.ctx),
                cash::Ty::Bool => LLVMInt1TypeInContext(self.ctx),
                cash::Ty::Void => LLVMVoidTypeInContext(self.ctx),
            };

            ty.into()
        }
    }

    fn read_type(&mut self) -> LLVMTypeRef {
        let cash::Extra::Type(ty) = self.read_data();
        self.lower_type(ty)
    }

    fn read_typed_field(&mut self) -> (RawStr<'a>, LLVMTypeRef) {
        (self.read_string(), self.read_type())
    }

    fn read_inst(&mut self) -> Option<cash::Inst> {
        let inst = self.src.instructions.get(self.inst_offset).cloned();
        self.inst_offset += 1;

        inst
    }

    fn read_string(&mut self) -> RawStr<'a> {
        let mut offset = 0;
        for b in self.src.strings.iter().skip(self.str_offset) {
            offset += 1;
            if *b == '\0' as u8 {
                break;
            }
        }

        let bytes = &self.src.strings[self.str_offset..(self.str_offset + offset)];
        self.str_offset += offset;
        RawStr(bytes)
    }

    fn read_data(&mut self) -> cash::Extra {
        let data = self.src.extra.get(self.data_offset).cloned();
        self.data_offset += 1;
        data.unwrap()
    }
}

pub struct RawStr<'a>(pub &'a [u8]);

impl<'a> RawStr<'a> {
    pub fn as_str(&self) -> &'a str {
        std::str::from_utf8(self.0).unwrap()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub unsafe fn llvm_str(&self) -> *const c_char {
        self.0.as_ptr() as *const _
    }
}

impl<'a> fmt::Display for RawStr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
