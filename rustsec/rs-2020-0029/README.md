## RUSTSEC-2020-0029
Run `cargo typepulse`:  
```
Error (UninitExposure:): Potential uninit exposure issue in `internal::pixel::ComponentBytes::as_bytes`
-> src/internal/pixel.rs:20:5: 25:6
fn as_bytes(&self) -> &[u8] {
        let slice = self.as_slice();
        unsafe {
            core::slice::from_raw_parts(slice.as_ptr() as *const _, slice.len() * core::mem::size_of::<T>())
        }
    }

Error (UninitExposure:): Potential uninit exposure issue in `internal::pixel::ComponentBytes::as_bytes_mut`
-> src/internal/pixel.rs:29:5: 34:6
fn as_bytes_mut(&mut self) -> &mut [u8] {
        let slice = self.as_mut_slice();
        unsafe {
            core::slice::from_raw_parts_mut(slice.as_mut_ptr() as *mut _, slice.len() * core::mem::size_of::<T>())
        }
    }
Error (BrokenBitPatterns:): Potential broken bit patterns issue in `internal::pixel::ComponentBytes::as_bytes_mut`
-> src/internal/pixel.rs:29:5: 34:6
fn as_bytes_mut(&mut self) -> &mut [u8] {
        let slice = self.as_mut_slice();
        unsafe {
            core::slice::from_raw_parts_mut(slice.as_mut_ptr() as *mut _, slice.len() * core::mem::size_of::<T>())
        }
    }
```