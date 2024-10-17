#![allow(temporary_cstring_as_ptr)]
#![allow(dead_code)]
#![allow(unused_imports)]

mod parts;
mod stages;

use crate::stages::{ir_compile::generate_ir,  linking::link_buffer, linking::sin};
use std::f64::consts::PI;
use std::fs::File;
use std::io::{Write, Read, self, BufRead};
use llvm_sys as llvm;
use llvm::core::*;
use llvm::prelude::*;
use llvm::ir_reader::LLVMParseIRInContext;
use llvm::target::*;
use llvm::target_machine::*;
use std::ffi::{CString, CStr};
use std::ptr;

use libc::{c_void, mmap, PROT_READ, PROT_WRITE, PROT_EXEC, MAP_PRIVATE, MAP_ANONYMOUS, MAP_FAILED};

extern "C" {
    static __object_buffer: u8;  // Declare the external symbol for the start of the reserved block, size is 16KB
}

// fn main(){
//     let mut file = File::open("prototype/example.o").unwrap();
//     let mut temp_buffer = Vec::new();
//     println!("Loaded file object file");
//     file.read_to_end(&mut temp_buffer).unwrap();
    
//     let buffer: &mut [u8] = &mut temp_buffer;

//     let buffer_size = buffer.len();

//     unsafe{
//         let object_address: *const u8 = &__object_buffer;

//         let fja: fn(f64) -> f64 = link_buffer(buffer, object_address as *mut u8);
//         println!("Linked object file");
//         std::ptr::copy_nonoverlapping(buffer.as_ptr(), object_address as *mut u8, buffer_size);
//         let result = fja(-PI/2.0);
//         println!("Result of the function: {}", result);
//     }
// }

fn main(){
    let function = String::from("sin(x)");
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
            eprintln!("Error parsing IR: {:?}", error);
            LLVMDisposeMessage(error);
            return;
        }

        LLVM_InitializeAllTargetInfos();
        LLVM_InitializeAllTargets();
        LLVM_InitializeAllTargetMCs();
        LLVM_InitializeAllAsmPrinters();
        LLVM_InitializeAllAsmParsers();

        let triple = LLVMGetDefaultTargetTriple();
        let mut target: LLVMTargetRef = ptr::null_mut();

        if LLVMGetTargetFromTriple(triple, &mut target, &mut error) != 0 {
            eprintln!("Error getting target: {:?}", error);
            LLVMDisposeMessage(error);
            return;
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
            eprintln!("Error emitting machine code to buffer: {:?}", error_message);
            LLVMDisposeMessage(error);
            return;
        }

        let buffer_start = LLVMGetBufferStart(memory_buffer) as *mut u8;
        let buffer_size = LLVMGetBufferSize(memory_buffer);

        let buffer_data = std::slice::from_raw_parts_mut(buffer_start, buffer_size as usize);

        let object_address: *const u8 = &__object_buffer;

        let fja: fn(f64) -> f64 = link_buffer(buffer_data, object_address as *mut u8);
        std::ptr::copy_nonoverlapping(buffer_data.as_ptr(), object_address as *mut u8, buffer_size);
        let result = fja(-PI/2.0);
        println!("Result of the function: {}", result);

        LLVMDisposeMemoryBuffer(memory_buffer);
        LLVMDisposeModule(module);
        LLVMDisposeTargetMachine(target_machine);
        LLVMContextDispose(context);
        LLVMDisposeMessage(triple);
    }
}

#[cfg(test)]
mod tests {
    mod unit_tree;
}