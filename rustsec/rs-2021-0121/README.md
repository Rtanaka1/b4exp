## RUSTSEC-2021-0121
Run `cargo typepulse`:  
```
Error (BrokenLayout:): Potential broken layout issue in `streamcipher::chacha20::xor_si512_inplace`
-> src/streamcipher/chacha20.rs:12:1: 22:2
fn xor_si512_inplace(a: &mut [u8], b: &[u32; Chacha20::STATE_LEN]) {
    // NOTE: 看起来编译器会对这种单独的函数做优化，我们不再需要手动写 AVX2/AVX512 的代码咯。
    use core::slice;

    unsafe {
        let d1 = slice::from_raw_parts_mut(a.as_mut_ptr() as *mut u32, Chacha20::STATE_LEN);
        for i in 0..Chacha20::STATE_LEN {
            d1[i] ^= b[i];
        }
    }
}
...
```