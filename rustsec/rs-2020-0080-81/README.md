## RUSTSEC-2020-0080/81
Run `cargo typepulse`:  
```
Error (UninitExposure:): Potential uninit exposure issue in `socket_addr`
-> src/main.rs:12:1: 26:2
fn socket_addr(addr: &SocketAddr) -> (&libc::sockaddr, libc::socklen_t) {
    // `RUSTSEC-2020-0081`
    use mem::size_of_val;

    match addr {
        SocketAddr::V4(ref addr) => {
            let tmp = addr as *const _ as *const libc::sockaddr;
            (unsafe { &*tmp }, size_of_val(addr) as libc::socklen_t)
        },
        SocketAddr::V6(ref addr) => {
            let tmp = addr as *const _ as *const libc::sockaddr;
            (unsafe { &*tmp }, size_of_val(addr) as libc::socklen_t)
        },
    }
}

Error (UninitExposure:): Potential uninit exposure issue in `socket_addr_to_ptrs`
-> src/main.rs:29:1: 41:2
fn socket_addr_to_ptrs(addr: &SocketAddr) -> (&SOCKADDR, c_int) {
    // `RUSTSEC-2020-0080`
    match *addr {
        SocketAddr::V4(ref a) => {
            let tmp = a as *const _ as *const SOCKADDR;
            (unsafe { &*tmp }, mem::size_of::<SOCKADDR_IN>() as c_int)
        },
        SocketAddr::V6(ref a) => {
            let tmp = a as *const _ as *const SOCKADDR;
            (unsafe { &*tmp }, mem::size_of::<SOCKADDR_IN6>() as c_int)
        },
    }
}
```
Buggy functions of `RUSTSEC-2020-0080/81` all relied on windows, so we create a unix version for them in a same crate.