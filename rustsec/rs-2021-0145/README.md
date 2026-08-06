## RUSTSEC-2021-0145
Run `cargo typepulse`:  
```
Error (BrokenLayout:): Potential broken layout issue in `vul_msys_tty_on`
-> src/lib.rs:122:1: 137:2
fn vul_msys_tty_on(fd: u32) -> bool {
    use core::{mem, slice};

    unsafe {
        let size = mem::size_of::<f_name_info>();
        let mut name_info_bytes = [0u8; 48 + mem::size_of::<u16>()];

        let name_info: &f_name_info = &*(name_info_bytes.as_ptr() as *const f_name_info);
        let s = slice::from_raw_parts(
            name_info.name.as_ptr(),
            name_info.name_len as usize / 2,
        );
    }

    true
}
```
Since the original buggy function relied on window system `cfg(windows)`, we manually create a unix version with same function body `vul_msys_tty_on`.