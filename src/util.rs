use crate::*;

/// Returns a pointer to `s` if `s` is a valid c string,
/// otherwise copies to `s` to `buf`, removes nul bytes and adds the final nul byte.
#[inline(always)]
pub(crate) fn cstr_buf<S: AsRef<[u8]>>(s: Option<S>, buf: &mut Vec<u8>) -> *mut libc::c_char {
    cstr_buf_impl(s.as_ref().map(|s| s.as_ref()), buf)
}

fn cstr_buf_impl(s: Option<&[u8]>, buf: &mut Vec<u8>) -> *mut libc::c_char {
    match s {
        Some(s) => {
            let nulb =
                unsafe { libc::memchr(s.as_ptr() as *const libc::c_void, 0, s.len()) as usize };
            // check if the only nul byte is at the end
            (if nulb as usize == s.as_ptr() as usize + s.len() - 1 {
                s.as_ptr()
            } else {
                buf.clear();
                buf.extend(s.iter().filter(|&&b| b != 0).chain(core::iter::once(&0u8)));
                buf.as_mut_ptr()
            }) as *mut libc::c_char
        }
        None => ptr::null_mut(),
    }
}

/// Converts a byte slice to a c string without checking for nul characters in the string.
///
/// # Safety
/// The function does not check for null bytes
#[inline]
pub unsafe fn cstr_unchecked<S: AsRef<[u8]>>(s: Option<S>) -> *const libc::c_char {
    match s {
        Some(s) => s.as_ref().as_ptr() as *const libc::c_char,
        None => ptr::null(),
    }
}
