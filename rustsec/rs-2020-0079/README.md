## RUSTSEC-2020-0079
Run `cargo typepulse`:  
```
Error (UninitExposure:): Potential uninit exposure issue in `<sockaddr::SockAddr as std::convert::From<std::net::SocketAddrV6>>::from`
-> src/sockaddr.rs:107:5: 114:6
fn from(addr: SocketAddrV6) -> SockAddr {
        unsafe {
            SockAddr::from_raw_parts(
                &addr as *const _ as *const _,
                mem::size_of::<SocketAddrV6>() as socklen_t,
            )
        }
    }

Error (UninitExposure:): Potential uninit exposure issue in `<sockaddr::SockAddr as std::convert::From<std::net::SocketAddrV4>>::from`
-> src/sockaddr.rs:96:5: 103:6
fn from(addr: SocketAddrV4) -> SockAddr {
        unsafe {
            SockAddr::from_raw_parts(
                &addr as *const _ as *const _,
                mem::size_of::<SocketAddrV4>() as socklen_t,
            )
        }
    }
```