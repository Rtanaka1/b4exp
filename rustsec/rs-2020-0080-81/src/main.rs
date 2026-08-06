use std::mem;
use std::net::SocketAddr;
use std::os::raw::c_int;

#[cfg(target_os = "windows")]
use winapi::shared::ws2def::{SOCKADDR, SOCKADDR_IN, SOCKADDR_IN6_LH};

#[cfg(target_os = "linux")]
use libc::{sockaddr as SOCKADDR, sockaddr_in as SOCKADDR_IN, sockaddr_in6 as SOCKADDR_IN6, sockaddr_storage, socklen_t};

// raw_ptr_deref; RUSTSEC-2020-0081
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

// windows; RUSTSEC-2020-0080
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

fn main() {
    println!("Testing adjusted code!!");
}
