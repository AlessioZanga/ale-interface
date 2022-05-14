use crate::ffi::{
    std::{string as std_string, vector as std_vector},
    utils::{
        std_string_buffer, std_string_capacity, std_string_from, std_string_length, std_vector_u32_buffer,
        std_vector_u32_capacity, std_vector_u32_length,
    },
};

/// Map `std::vector<uint32_t>` to `Vec<u32>`.
#[inline]
pub fn from_std_vector_u32(mut buffer: std_vector) -> Vec<u32> {
    unsafe {
        // Get raw pointer.
        let (buffer, length, capacity) = (
            std_vector_u32_buffer(&mut buffer),
            std_vector_u32_length(&buffer),
            std_vector_u32_capacity(&buffer),
        );
        // Map raw pointer to associated struct.
        Vec::from_raw_parts(buffer, length, capacity)
    }
}

/// Map `&str` to `std::string`.
#[inline]
pub fn into_std_string(str: &str) -> std_string {
    unsafe { std_string_from(str.as_ptr(), str.len()) }
}

/// Map `std::string` to `&str`.
#[inline]
pub fn from_std_string(mut buffer: std_string) -> String {
    unsafe {
        // Get raw pointer.
        let (buffer, length, capacity) = (
            std_string_buffer(&mut buffer),
            std_string_length(&buffer),
            std_string_capacity(&buffer),
        );
        // Map raw pointer to associated struct.
        String::from_raw_parts(buffer, length, capacity)
    }
}
