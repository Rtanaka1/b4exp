## RUSTSEC-2021-0019
Run `cargo typepulse`:  
```
Error (BrokenBitPatterns:): Potential broken bit patterns issue in `xproto::<impl base::Reply<ffi::xproto::xcb_get_atom_name_reply_t>>::name`
-> /home/rustsec/rs-2021-0019/target/x86_64-unknown-linux-gnu/debug/build/xcb-a0f9ee689706d3a1/out/xproto.rs:4591:5: 4600:6
pub fn name(&self) -> &str {
        unsafe {
            let field = self.ptr;
            let len = xcb_get_atom_name_name_length(field) as usize;
            let data = xcb_get_atom_name_name(field);
            let slice = std::slice::from_raw_parts(data as *const u8, len);
            // should we check what comes from X?
            std::str::from_utf8_unchecked(&slice)
        }
    }

Error (BrokenBitPatterns:): Potential broken bit patterns issue in `xproto::<impl base::Reply<ffi::xproto::xcb_list_fonts_with_info_reply_t>>::name`
-> /home/rustsec/rs-2021-0019/target/x86_64-unknown-linux-gnu/debug/build/xcb-a0f9ee689706d3a1/out/xproto.rs:8293:5: 8302:6
pub fn name(&self) -> &str {
        unsafe {
            let field = self.ptr;
            let len = xcb_list_fonts_with_info_name_length(field) as usize;
            let data = xcb_list_fonts_with_info_name(field);
            let slice = std::slice::from_raw_parts(data as *const u8, len);
            // should we check what comes from X?
            std::str::from_utf8_unchecked(&slice)
        }
    }
```