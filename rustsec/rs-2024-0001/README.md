## RUSTSEC-2024-0001
Run `cargo typepulse`:  
```
Error (BrokenBitPatterns:): Potential broken bit patterns issue in `say`
-> src/lib.rs:86:1: 146:2
pub fn say<W>(input: &[u8], max_width: usize, writer: &mut W) -> Result<()>
where
    W: Write,
{
    // Final output is stored here
    let mut write_buffer = SmallVec::<[u8; BUFSIZE]>::new();

    // Let textwrap work its magic
    let wrapped = fill(unsafe { str::from_utf8_unchecked(input) }, max_width);
...
```