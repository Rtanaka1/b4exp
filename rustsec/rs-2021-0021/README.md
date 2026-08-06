## RUSTSEC-2021-0021
Run `cargo typepulse`:  
```
Error (UninitExposure:): Potential uninit exposure issue in `Addr::new`
-> src/lib.rs:63:5: 77:6
fn new(addr: SocketAddr) -> Self {
        let (addr, len): (*const sockaddr, socklen_t) = match &addr {
            SocketAddr::V4(addr) => {
                let tmp = addr as *const _ as *const libc::sockaddr;
                let tmp_deref = unsafe { &*tmp };
                (tmp, mem::size_of_val(addr) as _)
            },
            SocketAddr::V6(addr) => {
                let tmp = addr as *const _ as *const libc::sockaddr;
                let tmp_deref = unsafe { &*tmp };
                (tmp, mem::size_of_val(addr) as _)
            },
        };
        unsafe { Self::from_raw_parts(addr, len) }
    }
```
Based on Rust's safety rules, we insert raw pointer dereference: `unsafe { &*tmp }`.