## RUSTSEC-2020-0050
Run `cargo typepulse`:  
```
Error (BrokenLayout:): Potential broken layout issue in `vec_copy::VecCopy::<V>::as_slice_as`
-> src/vec_copy.rs:500:5: 503:6
pub fn as_slice_as<T: Any>(&self) -> Option<&[T]> {
        let ptr = self.check_ref::<T>()?.data.as_ptr() as *const T;
        Some(unsafe { slice::from_raw_parts(ptr, self.len()) })
    }

Error (BrokenLayout:): Potential broken layout issue in `vec_copy::VecCopy::<V>::as_mut_slice_as`
-> src/vec_copy.rs:508:5: 511:6
pub fn as_mut_slice_as<T: Any>(&mut self) -> Option<&mut [T]> {
        let ptr = self.check_mut::<T>()?.data.as_mut_ptr() as *mut T;
        Some(unsafe { slice::from_raw_parts_mut(ptr, self.len()) })
    }

Error (BrokenLayout:): Potential broken layout issue in `vec_copy::VecCopy::<V>::get_as`
-> src/vec_copy.rs:515:5: 519:6
pub fn get_as<T: CopyElem>(&self, i: usize) -> Option<T> {
        assert!(i < self.len());
        let ptr = self.check_ref::<T>()?.data.as_ptr() as *const T;
        Some(unsafe { *ptr.add(i) })
    }

Error (BrokenLayout:): Potential broken layout issue in `vec_copy::VecCopy::<V>::get_ref_as`
-> src/vec_copy.rs:523:5: 527:6
pub fn get_ref_as<T: Any>(&self, i: usize) -> Option<&T> {
        assert!(i < self.len());
        let ptr = self.check_ref::<T>()?.data.as_ptr() as *const T;
        Some(unsafe { &*ptr.add(i) })
    }

Error (BrokenLayout:): Potential broken layout issue in `vec_copy::VecCopy::<V>::get_mut_as`
-> src/vec_copy.rs:531:5: 535:6
pub fn get_mut_as<T: Any>(&mut self, i: usize) -> Option<&mut T> {
        assert!(i < self.len());
        let ptr = self.check_mut::<T>()?.data.as_mut_ptr() as *mut T;
        Some(unsafe { &mut *ptr.add(i) })
    }
```
Vulnerable functions in `vec_copy.rs`.