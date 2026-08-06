## RUSTSEC-2019-0028
Run `cargo typepulse`:  
```
Error (BrokenBitPatterns:): Potential broken bit patterns issue in `endian_scalar::emplace_scalar`
-> src/endian_scalar.rs:152:1: 159:2
pub fn emplace_scalar<T: EndianScalar>(s: &mut [u8], x: T) {
    let sz = size_of::<T>();
    let mut_ptr = (&mut s[..sz]).as_mut_ptr() as *mut T;
    let val = x.to_little_endian();
    unsafe {
        *mut_ptr = val;
    }
}

Error (BrokenBitPatterns:): Potential broken bit patterns issue in `endian_scalar::read_scalar`
-> src/endian_scalar.rs:172:1: 179:2
pub fn read_scalar<T: EndianScalar>(s: &[u8]) -> T {
    let sz = size_of::<T>();

    let p = (&s[..sz]).as_ptr() as *const T;
    let x = unsafe { *p };

    x.from_little_endian()
}

Error (BrokenBitPatterns:): Potential broken bit patterns issue in `vector::follow_cast_ref`
-> src/vector.rs:93:1: 98:2
pub fn follow_cast_ref<'a, T: Sized + 'a>(buf: &'a [u8], loc: usize) -> &'a T {
    let sz = size_of::<T>();
    let buf = &buf[loc..loc + sz];
    let ptr = buf.as_ptr() as *const T;
    unsafe { &*ptr }
}

Error (BrokenBitPatterns:): Potential broken bit patterns issue in `vector::follow_slice_helper`
-> src/vector.rs:111:1: 119:2
fn follow_slice_helper<T>(buf: &[u8], loc: usize) -> &[T] {
    let sz = size_of::<T>();
    debug_assert!(sz > 0);
    let len = read_scalar::<UOffsetT>(&buf[loc..loc + SIZE_UOFFSET]) as usize;
    let data_buf = &buf[loc + SIZE_UOFFSET..loc + SIZE_UOFFSET + len * sz];
    let ptr = data_buf.as_ptr() as *const T;
    let s: &[T] = unsafe { from_raw_parts(ptr, len) };
    s
}

Error (BrokenBitPatterns:): Potential broken bit patterns issue in `vector::Vector::<'a, T>::safe_slice`
-> src/vector.rs:59:5: 69:6
pub fn safe_slice(self) -> &'a [T] {
        let buf = self.0;
        let loc = self.1;
        let sz = size_of::<T>();
        debug_assert!(sz > 0);
        let len = read_scalar::<UOffsetT>(&buf[loc..loc + SIZE_UOFFSET]) as usize;
        let data_buf = &buf[loc + SIZE_UOFFSET..loc + SIZE_UOFFSET + len * sz];
        let ptr = data_buf.as_ptr() as *const T;
        let s: &'a [T] = unsafe { from_raw_parts(ptr, len) };
        s
    }
```