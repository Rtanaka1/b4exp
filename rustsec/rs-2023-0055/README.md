## RUSTSEC-2023-0055
Run `cargo typepulse --features "parse"` in directory of `lexical-util`:  
```
Error (BrokenBitPatterns:): Potential broken bit patterns issue in `noskip::Bytes::<'a, __>::read_unchecked`
-> lexical-util/src/noskip.rs:111:5: 118:6
pub fn read_unchecked<V>(&self) -> V {
        debug_assert!(Self::IS_CONTIGUOUS);
        debug_assert!(self.as_slice().len() >= mem::size_of::<V>());

        let slc = self.as_slice();
        // SAFETY: safe as long as the slice has at least count elements.
        unsafe { ptr::read_unaligned::<V>(slc.as_ptr() as *const _) }
    }
```