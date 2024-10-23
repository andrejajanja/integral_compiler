#![allow(temporary_cstring_as_ptr)]
mod components;
mod stages;

extern crate libc;

use crate::components::terminal_decoration::Color;
use crate::components::auxilary_functions::parse_input_file;
use crate::stages::{
    ir_compile::generate_ir,
    linking::link_buffer
};
use std::{
    ffi::{CString, CStr},
    ptr,
    thread,
    sync::mpsc
};
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

fn calculate_integral(fja: fn (f64) -> f64, r_start: f64, r_end: f64, samples: u64) -> f64 {
    let mut x = r_start;
    let dx = (r_end-r_start)/(samples as f64);
    let mut sum = fja(r_start);

    for _ in 0..samples-1{
        x+=dx;
        sum += fja(x);
    }

    sum*dx
}

fn main(){
    let parameters = parse_input_file("app_config.toml");

    let llvm_ir = generate_ir(&parameters.function);

    let ir_c_string = CString::new(llvm_ir.clone()).unwrap();
    let fja;

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

        fja= link_buffer(buffer_data, object_address as *mut u8);
        std::ptr::copy_nonoverlapping(buffer_data.as_ptr(), object_address as *mut u8, buffer_size);

        LLVMDisposeMemoryBuffer(memory_buffer);
        LLVMDisposeModule(module);
        LLVMDisposeTargetMachine(target_machine);
        LLVMContextDispose(context);
        LLVMDisposeMessage(triple);
    }

    let result = calculate_integral(fja, parameters.range_start, parameters.range_end, parameters.samples);

    // let thread_n = 4 as u64;
    // let chunk_size = (parameters.range_end-parameters.range_start)/(thread_n as f64);
    // let chunk_samples = parameters.samples/thread_n;

    // let (tx, rx) = mpsc::channel();

    // for i in 0..thread_n {
    //     let tx = tx.clone();
    //     let r_start_thread = parameters.range_start + i as f64 * chunk_size;
    //     let r_end_thread = parameters.range_start + (i + 1) as f64 * chunk_size;
    //     thread::spawn(move || {
    //         let temp_result = calculate_integral(fja, r_start_thread, r_end_thread, chunk_samples);
    //         tx.send(temp_result).expect("Failed to send result");
    //     });
    // }

    // // Collect results from the threads
    // let mut result = 0.0;
    // for _ in 0..thread_n {
    //     result += rx.recv().expect("Failed to receive result");
    // }

    print!("\n\t{:.2}\n\t∫ {} dx = {:.10}\n\t{:.2}\n\n", parameters.range_end, &parameters.function, result, parameters.range_start);
}

#[cfg(test)]
mod tests {
    mod unit_parsing;
}