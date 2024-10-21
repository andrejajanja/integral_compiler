#![allow(temporary_cstring_as_ptr)]
mod components;
mod stages;

use crate::components::terminal_decoration::Color;
use crate::stages::{
    ir_compile::generate_ir,
    linking::link_buffer
};
use std::ffi::{CString, CStr};
use std::ptr;
use std::fs::read_to_string;
use llvm_sys::{
    core::*,
    prelude::*,
    ir_reader::LLVMParseIRInContext,
    target::*,
    target_machine::*
};

extern "C" {
    static __code_buffer: u8;  // Start of the reserved block, size is 16KB
}

fn main(){
    let function = read_to_string("fun.txt").expect("Failed to read function from a file");
    let llvm_ir = generate_ir(&function);

    let ir_c_string = CString::new(llvm_ir.clone()).unwrap();

    unsafe {
        let context = LLVMContextCreate();
        let buffer = LLVMCreateMemoryBufferWithMemoryRangeCopy(
            ir_c_string.as_ptr(),
            llvm_ir.len(), 
            CString::new("LLVM IR").unwrap().as_ptr()
        );

        let mut module: LLVMModuleRef = ptr::null_mut();
        let mut error: *mut i8 = ptr::null_mut();
        if LLVMParseIRInContext(context, buffer, &mut module, &mut error) != 0 {
            unrecoverable_error!("LLVM Error | Error occured while parsing the IR string", *error);
        }

        LLVM_InitializeAllTargetInfos();
        LLVM_InitializeAllTargets();
        LLVM_InitializeAllTargetMCs();
        LLVM_InitializeAllAsmPrinters();
        LLVM_InitializeAllAsmParsers();

        let triple = LLVMGetDefaultTargetTriple();
        let mut target: LLVMTargetRef = ptr::null_mut();

        if LLVMGetTargetFromTriple(triple, &mut target, &mut error) != 0 {
            unrecoverable_error!("LLVM Error | Error getting target information", *error);
        }

        let target_machine = LLVMCreateTargetMachine(
            target,
            triple,
            CString::new("generic").unwrap().as_ptr(),
            CString::new("").unwrap().as_ptr(),
            LLVMCodeGenOptLevel::LLVMCodeGenLevelAggressive,
            LLVMRelocMode::LLVMRelocDefault,
            LLVMCodeModel::LLVMCodeModelDefault
        );

        let mut memory_buffer: LLVMMemoryBufferRef = ptr::null_mut();
        if LLVMTargetMachineEmitToMemoryBuffer(
            target_machine,
            module,
            LLVMCodeGenFileType::LLVMObjectFile,
            &mut error,
            &mut memory_buffer
        ) != 0{
            let error_message = CStr::from_ptr(error).to_string_lossy().into_owned();
            unrecoverable_error!("LLVM Error | Error emitting machine code to buffer", error_message);
        }

        let buffer_start = LLVMGetBufferStart(memory_buffer) as *mut u8;
        let buffer_size = LLVMGetBufferSize(memory_buffer);

        let buffer_data = std::slice::from_raw_parts_mut(buffer_start, buffer_size as usize);

        let object_address: *const u8 = &__code_buffer;

        let fja= link_buffer(buffer_data, object_address as *mut u8);
        std::ptr::copy_nonoverlapping(buffer_data.as_ptr(), object_address as *mut u8, buffer_size);
        let result = fja(1.0);
        print!("\nResult of the function: {}\n\n", result);

        LLVMDisposeMemoryBuffer(memory_buffer);
        LLVMDisposeModule(module);
        LLVMDisposeTargetMachine(target_machine);
        LLVMContextDispose(context);
        LLVMDisposeMessage(triple);
    }
}

#[cfg(test)]
mod tests {
    mod unit_parsing;
}