## RUSTSEC-2021-0120
Run `cargo typepulse`:  
```
Error (UninitExposure:): Potential uninit exposure issue in `encode`
-> src/lib.rs:74:1: 81:2
pub fn encode<T: Abomonation, W: Write>(typed: &T, write: &mut W) -> IOResult<()> {
    unsafe {
        let slice = std::slice::from_raw_parts(mem::transmute(typed), mem::size_of::<T>());
        write.write_all(slice)?;
        typed.entomb(write)?;
    }
    Ok(())
}

Error (BrokenBitPatterns:): Potential broken bit patterns issue in `decode`
-> src/lib.rs:125:1: 139:2
pub fn decode<T: Abomonation>(bytes: &mut [u8]) -> Option<(&T, &mut [u8])> {
    unsafe {
    if bytes.len() < mem::size_of::<T>() { None }
    else {
        let (split1, split2) = bytes.split_at_mut(mem::size_of::<T>());
        let result: &mut T = mem::transmute(split1.get_unchecked_mut(0));
        if let Some(remaining) = result.exhume(split2) {
            Some((result, remaining))
        }
        else {
            None
        }
    }
    }
}
```