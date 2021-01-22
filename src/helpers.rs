use std::ffi::CString;

pub fn c_str_with_size(size: usize) -> CString {
    let mut buffer = Vec::with_capacity(size as usize + 1);
    buffer.extend([b' '].iter().cycle().take(size as usize));
    return unsafe { CString::from_vec_unchecked(buffer) };
}
