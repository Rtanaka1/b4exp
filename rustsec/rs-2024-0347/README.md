## RUSTSEC-2024-0347
Run `cargo typepulse`:  
```
Error (UninitExposure:): Potential uninit exposure issue in `ule::VarULE::as_byte_slice`
-> src/ule/mod.rs:352:5: 354:6
fn as_byte_slice(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self as *const Self as *const u8, mem::size_of_val(self)) }
    }

Error (UninitExposure:): Potential uninit exposure issue in `flexzerovec::slice::FlexZeroSlice::as_bytes`
-> src/flexzerovec/slice.rs:166:5: 171:6
pub fn as_bytes(&self) -> &[u8] {
        // Safety: See comments in `from_byte_slice_unchecked`
        unsafe {
            core::slice::from_raw_parts(self as *const Self as *const u8, self.data.len() + 1)
        }
    }
```