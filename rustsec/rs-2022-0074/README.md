## RUSTSEC-2022-0074
Run `cargo typepulse`:  
```
Error (UninitExposure:): Potential uninit exposure issue in `<Table as std::convert::AsRef<TableSlice<'a>>>::as_ref`
-> src/lib.rs:512:5: 519:6
fn as_ref(&self) -> &TableSlice<'a> {
        unsafe {
            // All this is a bit hacky. Let's try to find something else
            let s = &mut *((self as *const Table) as *mut Table);
            s.rows.shrink_to_fit();
            &*(self as *const Table as *const TableSlice<'a>)
        }
    }
```