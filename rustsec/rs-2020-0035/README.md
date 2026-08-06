## RUSTSEC-2020-0035
Run `cargo typepulse`:  
```
Error (BrokenLayout:): Potential broken layout issue in `value::Value::<V>::load_or_default`
-> src/value.rs:13:5: 26:6
pub fn load_or_default(ident: Ident, default: V, storage: Rc<dyn ChunkStorage>) -> Value<V> {
        let (mut chunk, created_new) = storage.load_or_create_chunk(ident, ::std::mem::size_of::<V>());

        if created_new {
            unsafe {
                ::std::ptr::write(chunk.as_mut_ptr() as *mut V, default);
            }
        }

        Value {
            chunk,
            _marker: PhantomData,
        }
    }

Error (BrokenLayout:): Potential broken layout issue in `<value::Value<V> as std::ops::DerefMut>::deref_mut`
-> src/value.rs:38:5: 40:6
fn deref_mut(&mut self) -> &mut V {
        unsafe { (self.chunk.as_mut_ptr() as *mut V).as_mut().unwrap() }
    }

Error (BrokenLayout:): Potential broken layout issue in `<value::Value<V> as std::ops::Deref>::deref`
-> src/value.rs:32:5: 34:6
fn deref(&self) -> &V {
        unsafe { (self.chunk.as_ptr() as *const V).as_ref().unwrap() }
    }
```
Vulnerable functions in `src/values.rs`.