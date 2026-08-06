## RUSTSEC-2022-0052
Run `cargo typepulse`:  
```
Error (UninitExposure:): Potential uninit exposure issue in `<OsSocketAddr as std::convert::From<std::net::SocketAddr>>::from`
-> src/lib.rs:268:5: 281:6
fn from(addr: SocketAddr) -> Self {
        OsSocketAddr {
            sa6: unsafe {
                match addr {
                    SocketAddr::V4(addr) => {
                        let mut sa6 = std::mem::zeroed();
                        *(&mut sa6 as *mut _ as *mut _) = addr;
                        sa6
                    }
                    SocketAddr::V6(addr) => *(&addr as *const _ as *const _),
                }
            },
        }
    }
```