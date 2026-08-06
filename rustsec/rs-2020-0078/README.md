## RUSTSEC-2020-0078
Run `cargo typepulse`:  
```
Error (UninitExposure:): Potential uninit exposure issue in `socket::addr2raw`
-> src/socket.rs:87:1: 100:2
fn addr2raw(addr: &SocketAddr) -> (&c::sockaddr, c::socklen_t) {
    unsafe {
        match *addr {
            SocketAddr::V4(ref a) => {
                let tmp = a as *const _ as *const c::sockaddr;
                (&*tmp, mem::size_of_val(a) as c::socklen_t)
            }
            SocketAddr::V6(ref a) => {
                let tmp = a as *const _ as *const c::sockaddr;
                (&*tmp, mem::size_of_val(a) as c::socklen_t)
            }
        }
    }
}
```