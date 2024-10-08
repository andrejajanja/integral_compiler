#![allow(temporary_cstring_as_ptr)]
#![allow(dead_code)]
#![allow(unused_imports)]

mod parts;
mod stages;

use crate::stages::{ir_compile::generate_ir,  linking::link_functions};
use std::fs::File;
use std::io::{Write, Read, self, BufRead};
use llvm_sys as llvm;
use llvm::core::*;
use llvm::prelude::*;
use llvm::ir_reader::LLVMParseIRInContext;
use llvm::target::*;
use llvm::target_machine::*;
use stages::linking::find_text_section;
use std::ffi::{CString, CStr};
use std::ptr;

use libc::{c_void, mmap, PROT_READ, PROT_WRITE, PROT_EXEC, MAP_PRIVATE, MAP_ANONYMOUS};

fn main(){
    let page_size = unsafe { libc::sysconf(libc::_SC_PAGESIZE) } as usize;
    // let function = String::from("sin(7.56*x)*e^(x+1)-tg(x-8)");
    // let (_llvm_ir, called_funcs) = generate_ir(&function);

    let mut file = File::open("example.o").unwrap();
    let mut temp_buffer = Vec::new();
    file.read_to_end(&mut temp_buffer).unwrap();
    
    let buffer: &[u8] = &temp_buffer;

    find_text_section(buffer);
    
    return;

    let buffer_size = buffer.len();

    unsafe{
        let aligned_size = (buffer_size + page_size - 1) & !(page_size - 1);
        // let func_memory = mmap(
        //     (sin as *mut u8).sub(0x40000) as *mut _,
        //     aligned_size,
        //     PROT_READ | PROT_WRITE | PROT_EXEC,
        //     MAP_PRIVATE | MAP_ANONYMOUS,
        //     -1,
        //     0,
        // );

        // if func_memory.is_null() {
        //     panic!("ALLOCATED MEMORY FOR A FUNCTION IS NULL");
        // }

        // std::ptr::copy_nonoverlapping(buffer.as_ptr(), func_memory as *mut u8, buffer_size);
        // link_functions(func_memory as *mut u8, buffer_size, called_funcs);
    }
}

// fn main(){
//     let function = String::from("sin(7.56*x)*e^(x+1)-tg(x-8)");
//     let (llvm_ir, called_funcs) = generate_ir(&function);
    
//     let ir_c_string = CString::new(llvm_ir.clone()).unwrap();

//     unsafe {
//         let context = LLVMContextCreate();
//         let buffer = LLVMCreateMemoryBufferWithMemoryRangeCopy(
//             ir_c_string.as_ptr(),
//             llvm_ir.len(),            
//             CString::new("LLVM IR").unwrap().as_ptr()
//         );

//         let mut module: LLVMModuleRef = ptr::null_mut();
//         let mut error: *mut i8 = ptr::null_mut();
//         if LLVMParseIRInContext(context, buffer, &mut module, &mut error) != 0 {
//             eprintln!("Error parsing IR: {:?}", error);
//             LLVMDisposeMessage(error);
//             return;
//         }

//         LLVM_InitializeAllTargetInfos();
//         LLVM_InitializeAllTargets();
//         LLVM_InitializeAllTargetMCs();
//         LLVM_InitializeAllAsmPrinters();
//         LLVM_InitializeAllAsmParsers();

//         let triple = LLVMGetDefaultTargetTriple();
//         let mut target: LLVMTargetRef = ptr::null_mut();

//         if LLVMGetTargetFromTriple(triple, &mut target, &mut error) != 0 {
//             eprintln!("Error getting target: {:?}", error);
//             LLVMDisposeMessage(error);
//             return;
//         }

//         let target_machine = LLVMCreateTargetMachine(
//             target,
//             triple,
//             CString::new("generic").unwrap().as_ptr(),
//             CString::new("").unwrap().as_ptr(),
//             LLVMCodeGenOptLevel::LLVMCodeGenLevelAggressive,
//             LLVMRelocMode::LLVMRelocDefault,
//             LLVMCodeModel::LLVMCodeModelDefault
//         );

//         let mut memory_buffer: LLVMMemoryBufferRef = ptr::null_mut();
//         if LLVMTargetMachineEmitToMemoryBuffer(
//             target_machine,
//             module,
//             LLVMCodeGenFileType::LLVMObjectFile,
//             &mut error,
//             &mut memory_buffer
//         ) != 0{
//             let error_message = CStr::from_ptr(error).to_string_lossy().into_owned();
//             eprintln!("Error emitting machine code to buffer: {:?}", error_message);
//             LLVMDisposeMessage(error);
//             return;
//         }

//         let buffer_start = LLVMGetBufferStart(memory_buffer) as *const u8;
//         let buffer_size = LLVMGetBufferSize(memory_buffer);

//         let buffer_data = std::slice::from_raw_parts(buffer_start, buffer_size as usize);
//         let mut file = File::create("example.o").unwrap();
//         file.write(&buffer_data).unwrap();
//         file.flush().unwrap();

//         let func_memory = allocate_executable_memory(buffer_size);
//         std::ptr::copy_nonoverlapping(buffer_start, func_memory as *mut u8, buffer_size);

//         let fja: CompiledFunc = std::mem::transmute(func_memory);

//         let result = fja(0.57);
//         println!("Result of the function: {}", result);


//         LLVMDisposeMemoryBuffer(memory_buffer);
//         LLVMDisposeModule(module);
//         LLVMDisposeTargetMachine(target_machine);
//         LLVMContextDispose(context);
//         LLVMDisposeMessage(triple);
//     }
// }

//toy main:
// fn main(){
//     let start = 5.0;
//     let end = 10.0;
//     let steps = 100000;
//     //let function = String::from("sin(7.56*x)*e^(x+1)-tg(x-8)");
//     //let function = String::from("sin(x)*e^(x)+cos(x)*ln(x)");
//     let function = String::from("sin(x)");
//     //let function = String::from("3*x+7");
//     //let function = String::from("sin(x)+7*e^(atg(x+11))");    

//     let ir_code = format!(r#"@.fstr = private unnamed_addr constant [4 x i8] c"%f\0A\00"
// declare i32 @printf(i8*, ...)
// declare i64 @llvm.readcyclecounter() nounwind readnone

// {}

// define i32 @main() {{
//     ;SUM that represents the integral
//     %integral = alloca double
//     store double 0.000000e0, double* %integral

//     ;shiftable argument the function
//     %arg = alloca double
//     store double {:.6e}, double* %arg

//     ;step of the aproximation
//     %width = alloca double
//     store double {:.6e}, double* %width

//     ;loop counter
//     %cnt = alloca i64
//     store i64 0, i64* %cnt

//     ;timer start
//     %start = call i64 @llvm.readcyclecounter()
//     br label %for_loop

// for_loop:
//     %temp_cnt = load i64, i64* %cnt
//     %temp_arg = load double, double* %arg
//     %ld_integral = load double, double* %integral
//     %ld_width = load double, double* %width, !readonly !0

//     %krajnji_uslov = icmp slt i64 %temp_cnt, {}
//     br i1 %krajnji_uslov, label %loop_body, label %loop_end

// loop_body:
//     %result = call double @fja(double %temp_arg)

//     ;+= the result of the function
//     %new_integral = fadd double %ld_integral, %result
//     store double %new_integral, double* %integral

//     ;shifting argument of a function for one width
//     %new_arg = fadd double %temp_arg, %ld_width
//     store double %new_arg, double* %arg

//     ;incrementing the loop counter
//     %new_cnt = add i64 %temp_cnt, 1
//     store i64 %new_cnt, i64* %cnt

//     br label %for_loop

// loop_end:
//     %temp_integral = load double, double* %integral
//     %temp_width = load double, double* %width, !readonly !0

//     ;calculating the final aproximated version of the integral
//     %new_temp_integral = fmul double %temp_integral, %temp_width

//     %end = call i64 @llvm.readcyclecounter()

//     %std = sitofp i64 %start to double
//     %endd = sitofp i64 %end to double
//     %elapsed_cycles = fsub double %endd, %std
//     %elapsed_time = fdiv double %elapsed_cycles, 2.0e9

//     %1 = getelementptr [4 x i8],[4 x i8]* @.fstr, i64 0, i64 0
//     %2 = call i32 (i8*, ...) @printf(i8* %1, double %new_temp_integral)
//     %3 = getelementptr [4 x i8],[4 x i8]* @.fstr, i64 0, i64 0
//     %4 = call i32 (i8*, ...) @printf(i8* %3, double %elapsed_time)
//     ret i32 0
// }}
// !0 = !{{!"readonly"}}"#, generate_ir(&function), start, (end-start)/(steps as f64), steps);

//     fs::write("/home/andreja/Documents/rust_projects/integral_aprox/IR_code.ll", ir_code).expect("neka greska");

//     let _kompilacija = Command::new("bash")
//         .arg("/home/andreja/Documents/rust_projects/integral_aprox/compile_parts.sh")
//         .output()
//         .expect("Greska u kompilaciji");
    
//     let output = Command::new("/home/andreja/Documents/rust_projects/integral_aprox/executable")
//         .output()
//         .expect("Greska u egzekuciji");

//     let stdout_str = String::from_utf8_lossy(&output.stdout);
//     let stdout_line: Vec<&str> = stdout_str.lines().collect();

//     print!("\n\n  {}\n\n ∫ {} = ", end, function);

//     if let Some(integral_value) = stdout_line.get(0) {
//         print!("{}\n\n{}\t\t\n\n", integral_value, start);
//     }

//     print!("Aproximated with {steps} steps");

//     if let Some(time) = stdout_line.get(1) {
//         println!(", in {}s.\n\n", time);
//     }
// }


// whole main, DO SOME WORK ON USER FUCKING EXPERIENCE
// fn main() {
//     let provided: Vec<String> = env::args().collect();
//     if provided.len() != 1 {
//         if provided[1] == "--help"{
//             print_help();
//             exit(0);
//         }
//     }
//     let (mut function, start, end, steps) = parse_inputs();
//     function = function.replace(" ", "");

//     let ir_code = generate_ir(&function);
//     println!("{}", ir_code);
 
//     print!("\n\n  {}\n\n ∫ 3*x + 7 dx  =  89.0\t\t(With {steps} steps)\n\n{}\n\n", end, start);
// }

#[cfg(test)]
mod tests {
    mod unit_tree;
}