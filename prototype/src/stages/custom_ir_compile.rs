use crate::stages::{
    binary_compile::generate_binary_from_ir,
    linking::{link_buffer, FunctionType}
};

use std::ptr::NonNull;

extern "C" {
    static __code_buffer: u8;  // Start of the reserved block, size is 16KB
}

pub fn generate_custom_function(ir_code: String) -> FunctionType{
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