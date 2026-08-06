## RUSTSEC-2021-0035
Run `cargo typepulse` in `quinn`:  
```
Error (UninitExposure:): Potential uninit exposure issue in `platform::imp::prepare_msg`
-> quinn/src/platform/unix.rs:335:1: 407:2
fn prepare_msg(
    transmit: &Transmit,
    hdr: &mut libc::msghdr,
    iov: &mut libc::iovec,
    ctrl: &mut cmsg::Aligned<[u8; CMSG_LEN]>,
) {
    iov.iov_base = transmit.contents.as_ptr() as *const _ as *mut _;
    iov.iov_len = transmit.contents.len();

    let (name, namelen) = match transmit.destination {
        SocketAddr::V4(ref addr) => {
            let tmp: *mut libc::c_void = addr as *const _ as _;
            let tmp_deref = unsafe { &mut *tmp };
            (tmp, mem::size_of::<libc::sockaddr_in>())
        },
        SocketAddr::V6(ref addr) => {
            let tmp: *mut libc::c_void = addr as *const _ as _;
            let tmp_deref = unsafe { &mut *tmp };
            (tmp, mem::size_of::<libc::sockaddr_in6>())
        },
    };
```
The original code leaves raw pointer only. However, we rely on Rust's safety rules, which only guarantee references, so we insert raw pointer dereference here (`unsafe { &mut *tmp }`).