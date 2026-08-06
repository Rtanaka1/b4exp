## RUSTSEC-2019-0035
Run `cargo typepulse` in `rand_core`:  
```
Error (BrokenLayout:): Potential broken layout issue in `<block::BlockRng64<R> as RngCore>::fill_bytes`
-> rand_core/src/block.rs:422:5: 457:6
fn fill_bytes(&mut self, dest: &mut [u8]) {
        let mut filled = 0;
        self.half_used = false;

        // Continue filling from the current set of results
        if self.index < self.results.as_ref().len() {
            let (consumed_u64, filled_u8) =
                fill_via_u64_chunks(&self.results.as_ref()[self.index..],
                                    dest);

            self.index += consumed_u64;
            filled += filled_u8;
        }

        let len_remainder =
            (dest.len() - filled) % (self.results.as_ref().len() * 8);
        let end_direct = dest.len() - len_remainder;

        while filled < end_direct {
            let dest_u64: &mut R::Results = unsafe {
                ::core::mem::transmute(dest[filled..].as_mut_ptr())
            };
```