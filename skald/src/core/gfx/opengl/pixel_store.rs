/// 1, 2, 4, or 8. Default: 4
pub fn set_unpack_alignment(param: i32) {
    unsafe {
        gl::PixelStorei(gl::UNPACK_ALIGNMENT, param)
    }
}

pub fn get_unpack_alignment() -> i32 {
    unsafe {
        let mut val: i32 = 0;

        gl::GetIntegerv(gl::UNPACK_ALIGNMENT, &mut val);

        val
    }
}
