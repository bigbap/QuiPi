use std::ffi::CString;

pub fn create_empty_cstring_with_len(len: usize) -> CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);

    buffer.extend([b' '].iter().cycle().take(len));

    unsafe { CString::from_vec_unchecked(buffer) }
}

macro_rules! c_str {
    ($literal: expr) => {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(format!("{}\0", $literal).as_bytes())
    }
}

pub(crate) use c_str;