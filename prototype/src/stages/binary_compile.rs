#![allow(temporary_cstring_as_ptr)]
#![allow(dead_code)]
extern crate libc;

use crate::{
    unrecoverable_error,
    components::terminal_decoration::Color,
    components::taylor_optimizer::optimize_postfix_using_taylor,
    stages::function_lexing::{lex_function, convert_infix_to_postfix},
    stages::taylor_ir_compile::generate_ir_from_taylor_sequence,
    stages::linking::{link_buffer, FunctionType},
};
use std::{
    ffi::{CString, CStr},
    ptr, ptr::NonNull
};
use llvm_sys::{
    core::*,
    prelude::*,
    ir_reader::LLVMParseIRInContext,
    target::*,
    target_machine::*
};

pub fn generate_binary_from_ir(llvm_ir: String) -> (Vec<u8>, usize){
    let llvm_ir_len = llvm_ir.len();
    let ir_c_string = CString::new(llvm_ir).unwrap();
    let buffer_data: Vec<u8>;
    let buffer_len: usize;

    unsafe {
        let context = LLVMContextCreate();
        let buffer = LLVMCreateMemoryBufferWithMemoryRangeCopy(
            ir_c_string.as_ptr(),
            llvm_ir_len, 
            CString::new("LLVM IR").unwrap().as_ptr()
        );

        let mut module: LLVMModuleRef = ptr::null_mut();
        let mut error: *mut i8 = ptr::null_mut();
        if LLVMParseIRInContext(context, buffer, &mut module, &mut error) != 0 {
            let c_str = CStr::from_ptr(error); // Interpret as CStr
            let temp_str = match c_str.to_str() {
                Ok(rust_str) => rust_str,
                Err(e) => &format!("Failed to convert to &str: {}", e),
            };

            unrecoverable_error!("LLVM Error | Error occured while lexing and parsing the IR string", temp_str);
        }

        let result = LLVM_InitializeNativeTarget();
        if result != 0 {
            unrecoverable_error!("LLVM Error | Initialization", "Failed to initialize native target.");
        }

        let result = LLVM_InitializeNativeAsmPrinter();
        if result != 0 {
            unrecoverable_error!("LLVM Error | Initialization", "Failed to initialize native assembler printer.");
        }

        let triple = LLVMGetDefaultTargetTriple();
        let mut target: LLVMTargetRef = ptr::null_mut();

        if LLVMGetTargetFromTriple(triple, &mut target, &mut error) != 0 {
            let c_str = CStr::from_ptr(error); // Interpret as CStr
            let temp_str = match c_str.to_str() {
                Ok(rust_str) => rust_str,
                Err(e) => &format!("Failed to convert to &str: {}", e),
            };
            unrecoverable_error!("LLVM Error | Error getting target information", temp_str);
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
        buffer_len = LLVMGetBufferSize(memory_buffer);

        buffer_data = std::slice::from_raw_parts_mut(buffer_start, buffer_len).to_vec();

        LLVMDisposeMemoryBuffer(memory_buffer);
        LLVMDisposeModule(module);
        LLVMDisposeTargetMachine(target_machine);
        LLVMContextDispose(context);
        LLVMDisposeMessage(triple);
    }

    (buffer_data, buffer_len)
}

pub fn save_generated_binary_to_file(llvm_ir: String, obj_file: String) {
    let llvm_ir_len = llvm_ir.len();
    let ir_c_string = CString::new(llvm_ir).unwrap();

    unsafe {
        let context = LLVMContextCreate();
        let buffer = LLVMCreateMemoryBufferWithMemoryRangeCopy(
            ir_c_string.as_ptr(),
            llvm_ir_len, 
            CString::new("LLVM IR").unwrap().as_ptr()
        );

        let mut module: LLVMModuleRef = ptr::null_mut();
        let mut error: *mut i8 = ptr::null_mut();
        if LLVMParseIRInContext(context, buffer, &mut module, &mut error) != 0 {
            let error_message = CStr::from_ptr(error).to_string_lossy().into_owned();
            unrecoverable_error!("LLVM Error | Error occured while lexing and parsing the IR string", error_message);
        }

        let result = LLVM_InitializeNativeTarget();
        if result != 0 {
            unrecoverable_error!("LLVM Error | Initialization", "Failed to initialize native target.");
        }

        let result = LLVM_InitializeNativeAsmPrinter();
        if result != 0 {
            unrecoverable_error!("LLVM Error | Initialization", "Failed to initialize native assembler printer.");
        }

        let triple = LLVMGetDefaultTargetTriple();
        let mut target: LLVMTargetRef = ptr::null_mut();

        if LLVMGetTargetFromTriple(triple, &mut target, &mut error) != 0 {
            let error_message = CStr::from_ptr(error).to_string_lossy().into_owned();
            unrecoverable_error!("LLVM Error | Error getting target information", error_message);
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

        if LLVMTargetMachineEmitToFile(
            target_machine,
            module,
            CString::new(obj_file).unwrap().as_ptr(),
            LLVMCodeGenFileType::LLVMObjectFile,
            &mut error
        ) != 0{
            let error_message = CStr::from_ptr(error).to_string_lossy().into_owned();
            unrecoverable_error!("LLVM Error | Error emitting machine code to buffer", error_message);
        }

        LLVMDisposeModule(module);
        LLVMDisposeTargetMachine(target_machine);
        LLVMContextDispose(context);
        LLVMDisposeMessage(triple);
    }
}

extern "C" {
    static __code_buffer: u8;  // Start of the reserved block, size is 16KB
}

pub fn generate_function(function: &str, precision_center:f64, max_power: usize) -> FunctionType{
    let mut sequence = lex_function(function);
    convert_infix_to_postfix(&mut sequence);
    optimize_postfix_using_taylor(&mut sequence, precision_center, max_power);
    let ir_code = generate_ir_from_taylor_sequence(&sequence);

    println!("{}", ir_code);

    let (mut buffer_data, buffer_len) = generate_binary_from_ir(ir_code);

    unsafe {
        let object_space: *const u8 = &__code_buffer;

        let temp = link_buffer(
            &mut buffer_data,
            NonNull::new_unchecked(object_space as *mut u8)
        );

        std::ptr::copy_nonoverlapping(buffer_data.as_ptr(), object_space as *mut u8, buffer_len);

        temp
    }
}
