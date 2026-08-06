## RUSTSEC-2020-0165
Run `cargo typepulse` in current package:  

First, we create a vulnerable version of `read_scanlines` since the old version (`v0.8.17`) of crate relied on `mozjpeg-sys`:
```
Error during execution of `cargo metadata`:     Updating crates.io index
error: failed to select a version for the requirement `mozjpeg-sys = "^0.10.5"`
candidate versions found which didn't match: 2.2.2, 2.2.1, 2.2.0, ...
location searched: crates.io index
required by package `mozjpeg v0.8.17 (/home/rustsec/mozjpeg-0.8.17)`
perhaps a crate was updated and forgotten to be re-vendored?
```

Solving the dependencies issue, the following function (`src/decompress.rs`) is the one that can reproduce the issue:
```rust
pub fn vul_read_scanlines<T: Copy + 'static>(&mut self) -> Vec<T> {
    let num_components = self.color_space().num_components();
    assert_eq!(num_components, mem::size_of::<T>());
    let width = self.width();
    let height = self.height();
    let mut image_dst: Vec<T> = Vec::with_capacity(self.height() * width);
    unsafe {
        // image_dst.extend_uninit(height * width);

        // while self.read_more_chunks() {
            // let start_line = self.dec.cinfo.output_scanline as usize;
            let rest: &mut [T] = &mut image_dst[..];
            let rows = (&mut rest.as_mut_ptr()) as *mut *mut T;
            let rows_read = ffi::jpeg_read_scanlines(&mut self.dec.cinfo, rows as *mut *mut u8, 1) as usize;
        // }
    }

    image_dst
}
```

Since the mutable type will be passed to `ffi::jpeg_read_scanlines`, where the FFI function is out of our scope. We instrument our detector to show that TypePulse can detect the creation of invalid type (see the following log), ensuring the capability of our work.

```
|INFO | [typepulse-progress] Mismatched Scope Bug: Waiting type to be mutated as invalid type...
```