//! This holds the libcore basic requirements for things like libc routines
//! GAMOZO LABS WROTE THIS CODE! GO FOLLOW HIM ON TWITCH :) @gamozo
///
/// libc `memcpy` implementation in Rust, without the C convention and mangling
/// allowing for better internal inlining
///
/// # Parameters
///
/// * `dest` - pointer to memory to copy to
/// * `src`  - pointer to memory to copy from
/// * `n`    - number of bytes to copy
#[no_mangle]
#[cfg(target_arch = "x86_64")]
pub unsafe extern fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    asm!("rep movsb",
        inout("rcx") n    => _,
        inout("rdi") dest => _,
        inout("rsi") src  => _);

    dest
}

/// libc `memmove` implementation in Rust
///
/// # Parameters
///
/// * `dest` - pointer to memory to copy to
/// * `src`  - pointer to memory to copy from
/// * `n`    - number of bytes to copy
#[no_mangle]
pub unsafe extern fn memmove(dest: *mut u8, src: *const u8, mut n: usize)
    -> *mut u8 {
        if (dest as usize) > (src as usize) &&
            (src as usize).wrapping_add(n) > (dest as usize) {
            
            let overhang = dest as usize - src as usize;

            if overhang < 64 {
                while n != 0 && (dest as usize).wrapping_add(n) & 0x7 != 0 {
                    n = n.wrapping_sub(1);
                    *dest.offset(n as isize) = *src.offset(n as isize);
                }

                while n >= 8 {
                        n = n.wrapping_sub(8);

                        let val = core::ptr::read_unaligned(
                            src.offset(n as isize) as *const u64);

                        core::ptr::write(dest.offset(n as isize) as *mut u64, val);
                }

                while n != 0 {
                    n = n.wrapping_sub(1);
                    *dest.offset(n as isize) = *src.offset(n as isize);
                }

                return dest;
            }

            while n >= overhang {
                n = n.wrapping_sub(overhang);
                let src  = src.offset(n as isize);
                let dest = dest.offset(n as isize);
                memcpy(dest, src, overhang);
            }

            if n == 0 {
                return dest;
            }
    }
    memcpy(dest, src, n);

    dest
}

/// libc `memset` implementation in Rust
///
/// # Parameters
/// * `s` - pointer to memory to set
/// * `c` - character to set `n` bytes in `s` to
/// * `n` - number of bytes to set
#[no_mangle]
#[cfg(target_arch = "x86_64")]
pub unsafe extern fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8 {
    asm!("rep stosb",
        inout("rcx") n    => _,
        inout("rdi") s    => _,
        in("eax")    c as u32);

    s
}

/// libc `memcmp` implementation in Rust
///
/// # Parameters
///
/// * `s1` - pointer to memory to compare with s2
/// * `s2` - pointer to memory to compare with s1
/// * `n`  - number of bytes to compare
#[no_mangle]
pub unsafe extern fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    let mut ii = 0;

    while ii < n {
        let a = *s1.offset(ii as isize);
        let b = *s2.offset(ii as isize);
        if a != b {
            return (a as i32).wrapping_sub(b as i32);
        }
        ii = ii.wrapping_add(1);
    }

    0
}