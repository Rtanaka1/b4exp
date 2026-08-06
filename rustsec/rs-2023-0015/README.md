## RUSTSEC-2023-0015
Run `cargo typepulse` in current package:  
```
Error (BrokenBitPatterns:): Potential broken bit patterns issue in `<ascii_string::AsciiString as std::convert::Into<std::vec::Vec<u8>>>::into`
-> src/ascii_string.rs:435:5: 448:6
fn into(self) -> Vec<u8> {
        unsafe {
            let v = Vec::from_raw_parts(
                self.vec.as_ptr() as *mut u8,
                self.vec.len(),
                self.vec.capacity(),
            );

            // We forget `self` to avoid freeing it at the end of the scope.
            // Otherwise, the returned `Vec` would point to freed memory.
            mem::forget(self);
            v
        }
    }
```