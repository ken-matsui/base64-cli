use std::ffi::{CStr, CString};
use std::mem;
use std::os::raw::c_char;

/// # Safety
///
/// This public function might dereference a raw pointer.
#[no_mangle]
pub unsafe extern "C" fn encode(input: *const c_char) -> *const c_char {
    let input_c_str = CStr::from_ptr(input);
    let output = CString::new(base64::encode(input_c_str.to_str().unwrap())).unwrap();
    let ptr = output.as_ptr();
    mem::forget(output);
    ptr
}

/// # Safety
///
/// This public function might dereference a raw pointer.
#[no_mangle]
pub unsafe extern "C" fn decode(input: *const c_char) -> *const c_char {
    let input_c_str = CStr::from_ptr(input);
    let output = CString::new(base64::decode(input_c_str.to_str().unwrap()).unwrap()).unwrap();
    let ptr = output.as_ptr();
    mem::forget(output);
    ptr
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
                const char* output = encode(input);
                printf("%s", output);
                free((char*)output);
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
                const char* output = decode(input);
                printf("%s", output);
                free((char*)output);
                return 0;
            }
        })
        .success()
        .stdout("hello");
    }
}
