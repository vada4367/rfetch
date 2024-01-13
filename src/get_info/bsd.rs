#![allow(dead_code)]

use crate::get_info::*;

pub fn device(
    sys_format: &SystemFormat,
    info_space: size_t,
) -> CSTR {
    let mut spaces = [0x20 as c_char; LEN_STRING + 100];
    let spaces_str = &mut spaces[..info_space + 1];
    spaces_str[info_space] = 0 as c_char;

    let file_ptr = unsafe {
        popen(c_str("/sbin/sysctl -n hw.product\0"), c_str("r\0"))
    };

    if file_ptr.is_null() {
        return c_str("popen_error\0");
    }
    let output = [0; LEN_STRING + 100];

    let result = [0; LEN_STRING];
    unsafe {
        fgets(
            output.as_ptr() as *mut c_char,
            (LEN_STRING + 100) as i32,
            file_ptr,
        );
        sprintf(
            result.as_ptr() as *mut c_char,
            c_str("host %s%s\0"),
            spaces_str.as_ptr() as CSTR,
            c_str(&output),
        );
    }

    // DELETE ALL \n
    let mut p = result.as_ptr() as CSTR;
    loop {
        // 0x0a IS \n
        p = unsafe { strchr(p, 0x0a as c_int) };
        if p == core::ptr::null() {
            break;
        }
        unsafe { strcpy(p as *mut c_char, p.add(1)) };
    }

    c_str(&result)
}

pub fn uptime(
    sys_format: &SystemFormat,
    info_space: size_t,
) -> CSTR {
    todo!();
}

pub fn memory(
    sys_format: &SystemFormat,
    info_space: size_t,
) -> CSTR {
    let mut spaces = [0x20 as c_char; LEN_STRING + 100];
    let spaces_str = &mut spaces[..info_space + 1];
    spaces_str[info_space] = 0 as c_char;

    let mut usage = unsafe { MaybeUninit::<rusage>::uninit().assume_init() };
    let result = [0; LEN_STRING];
    unsafe {
        getrusage(0, &mut usage as *mut rusage);
        let used = usage.ru_maxrss * 1024;
        sprintf(
            result.as_ptr() as *mut c_char,
            c_str("memory %s%d\0"),
            spaces_str.as_ptr() as CSTR,
            used,
        );
    }

    c_str(&result)
}

pub fn pkgs(sys_format: &SystemFormat, info_space: size_t) -> CSTR {
    todo!();
}

pub fn get_os_name() -> &'static str {
    let result = [0; LEN_STRING];

    let mut name =
        unsafe { MaybeUninit::<utsname>::uninit().assume_init() };

    unsafe {
        uname(&mut name);

        sprintf(
            result.as_ptr() as *mut c_char,
            c_str("%s\0"),
            c_str(&name.sysname),
        );
    }

    unsafe {
        core::str::from_utf8_unchecked(slice::from_raw_parts(
            c_str(&result) as *const u8,
            strlen(c_str(&result)),
        ))
    }
}

