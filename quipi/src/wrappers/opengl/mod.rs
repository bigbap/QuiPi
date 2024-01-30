use super::sdl2::window::QuiPiWindow;

pub mod buffer;
pub mod capabilities;
pub mod draw;
pub mod functions;
pub mod pixel_store;
pub mod textures;
pub mod shader;

pub struct MyOpenGL {}

impl MyOpenGL {
    pub fn init(
        window_api: &QuiPiWindow
    ) -> Result<Self, Box<dyn std::error::Error>> {
        gl::load_with(
            |name| window_api.video_subsystem.gl_get_proc_address(name) as *const _
        );

        let mut flags: gl::types::GLint = 0;
        unsafe {
            gl::GetIntegerv(gl::CONTEXT_FLAGS, &mut flags);

            if flags & (gl::CONTEXT_FLAG_DEBUG_BIT as i32) > 0 {
                println!("opengl debug enabled");

                gl::Enable(gl::DEBUG_OUTPUT);
                gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);

                let ptr: extern "system" fn(
                    gl::types::GLenum,
                    gl::types::GLenum,
                    gl::types::GLuint,
                    gl::types::GLenum,
                    gl::types::GLsizei,
                    *const i8,
                    *mut gl::types::GLvoid
                ) = debug_callback;
                gl::DebugMessageCallback(Some(ptr), std::ptr::null());
                gl::DebugMessageControl(
                    gl::DONT_CARE,
                    gl::DONT_CARE,
                    gl::DONT_CARE,
                    0,
                    std::ptr::null(),
                    gl::TRUE
                );
            }
        }

        Ok(Self {})
    }
}

extern "system" fn debug_callback(
    source: gl::types::GLenum,
    kind: gl::types::GLenum,
    id: gl::types::GLuint,
    severity: gl::types::GLenum,
    _length: gl::types::GLsizei,
    message: *const i8,
    _user_params: *mut gl::types::GLvoid
) {
    if id == 131169 || id == 131185 || id == 131218 || id == 131204 {
        return;
    }

    println!("______________________");
    println!("Debug message ({id}): {:?}", message);

    match source {
        gl::DEBUG_SOURCE_API => println!("Source: API"),
        gl::DEBUG_SOURCE_WINDOW_SYSTEM => println!("Source: Window System"),
        gl::DEBUG_SOURCE_SHADER_COMPILER => println!("Source: Shader Compiler"),
        gl::DEBUG_SOURCE_THIRD_PARTY => println!("Source: Third Party"),
        gl::DEBUG_SOURCE_APPLICATION => println!("Source: Application"),
        gl::DEBUG_SOURCE_OTHER => println!("Source: Other"),
        _ => ()
    }

    match kind {
        gl::DEBUG_TYPE_ERROR => println!("Type: Error"),
        gl::DEBUG_TYPE_DEPRECATED_BEHAVIOR => println!("Type: Deprecated Behavior"),
        gl::DEBUG_TYPE_UNDEFINED_BEHAVIOR => println!("Type: Undefined Behavior"),
        gl::DEBUG_TYPE_PORTABILITY => println!("Type: Portability"),
        gl::DEBUG_TYPE_PERFORMANCE => println!("Type: Performance"),
        gl::DEBUG_TYPE_MARKER => println!("Type: Marker"),
        gl::DEBUG_TYPE_PUSH_GROUP => println!("Type: Push Group"),
        gl::DEBUG_TYPE_POP_GROUP => println!("Type: Pop Group"),
        gl::DEBUG_TYPE_OTHER => println!("Type: Other"),
        _ => ()
    }

    match severity {
        gl::DEBUG_SEVERITY_HIGH => println!("Severity: high"),
        gl::DEBUG_SEVERITY_MEDIUM => println!("Severity: medium"),
        gl::DEBUG_SEVERITY_LOW => println!("Severity: low"),
        gl::DEBUG_SEVERITY_NOTIFICATION => println!("Severity: notification"),
        _ => ()
    }
}
