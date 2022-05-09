use std::ffi::{CStr, CString};
use std::os::raw::c_char;

/// # Safety
///
/// The returned char pointer should be freed by the `cstring_free` function.
/// For detailed usage in any language, please see:
/// https://jakegoulding.com/rust-ffi-omnibus/string_return/
#[no_mangle]
pub unsafe extern "C" fn encode(input: *const c_char) -> *mut c_char {
    let input_c_str = CStr::from_ptr(input);
    let output = CString::new(base64::encode(input_c_str.to_str().unwrap())).unwrap();
    output.into_raw()
}

/// # Safety
///
/// The returned char pointer should be freed by the `cstring_free` function.
/// For detailed usage in any language, please see:
/// https://jakegoulding.com/rust-ffi-omnibus/string_return/
#[no_mangle]
pub unsafe extern "C" fn decode(input: *const c_char) -> *mut c_char {
    let input_c_str = CStr::from_ptr(input);
    let output = CString::new(base64::decode(input_c_str.to_str().unwrap()).unwrap()).unwrap();
    output.into_raw()
}

/// # Safety
///
/// This function is for freeing a pointer of the argument.
/// The pointer should be allocated with `CString` in the Rust world.
#[no_mangle]
pub unsafe extern "C" fn cstring_free(s: *mut c_char) {
    if s.is_null() {
        return;
    }

    // retake pointer to free memory
    let _ = CString::from_raw(s);
}

#[cfg(test)]
mod tests {
    use inline_c::assert_c;

    #[test]
    fn test_c_api_encode() {
        (assert_c! {
            #include "base64-cli.h"
            #include <stdio.h>
            #include <stdlib.h>

            int main() {
                const char* input = "hello";
                char* output = encode(input);
                printf("%s", output);
                cstring_free(output);
                return 0;
            }
        })
        .success()
        .stdout("aGVsbG8=");
    }

    #[test]
    fn test_c_api_decode() {
        (assert_c! {
            #include "base64-cli.h"
            #include <stdio.h>
            #include <stdlib.h>

            int main() {
                const char* input = "aGVsbG8=";
                char* output = decode(input);
                printf("%s", output);
                cstring_free(output);
                return 0;
            }
        })
        .success()
        .stdout("hello");
    }
}
