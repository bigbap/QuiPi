macro_rules! c_str {
    ($literal: expr) => {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(format!("{}\0", $literal).as_bytes())
    }
}

pub(crate) use c_str;
