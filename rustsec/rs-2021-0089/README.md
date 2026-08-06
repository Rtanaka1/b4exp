## RUSTSEC-2021-0089
Run `cargo typepulse` in current package:  
```
Error (BrokenBitPatterns:): Potential broken bit patterns issue in `VendorInfo::as_string`
-> src/lib.rs:720:5: 732:6
pub fn as_string<'a>(&'a self) -> &'a str {
        let brand_string_start = self as *const VendorInfo as *const u8;
        unsafe {
            // Safety: VendorInfo is laid out with repr(C) and exactly
            // 12 byte long without any padding.
            let slice: &'a [u8] =
                slice::from_raw_parts(brand_string_start, size_of::<VendorInfo>());
            // Safety: The field is specified to be ASCII, and the only safe
            // way to construct VendorInfo is from real CPUID data or the
            // Default implementation.
            str::from_utf8_unchecked(slice)
        }
    }
```