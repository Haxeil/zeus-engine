use gl;

pub unsafe fn gl_clear_errors() {
    while gl::GetError() != gl::NO_ERROR {}
}

pub unsafe fn gl_check_error(func_name: &str, file_name: &str, line: u32) {
    loop {
        let error = gl::GetError();
        if error == 0 {
            break;
        }
        println!(
            "open_gl Error: {}, func: {}, file_name: {}, line: {}",
            error, func_name, file_name, line
        );
        debug_assert!(false);
    }
}

#[macro_export]
macro_rules! log_gl_error {
    ($func:expr) => {
        unsafe {
            gl_clear_errors();
            $func;
            gl_check_error(stringify!($func), file!(), line!())
        }
    };

    ($stmt:stmt) => {
        unsafe { gl_clear_errors() };
        let name = stringify!($stmt);
        $stmt
        unsafe { gl_check_error(name, file!(), line!()) };
    };
}
