## RUSTSEC-2022-0092
Run `cargo typepulse`:  
```
Error (BrokenBitPatterns:): Potential broken bit patterns issue in `<Raw as serde::Serialize>::serialize`
-> rmp-serde/src/lib.rs:182:5: 194:6
fn serialize<S>(&self, se: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer
    {
        let s = match self.s {
            Ok(ref s) => s.as_str(),
            // FIXME: this is invalid. It should use a newtype hack instead.
            // https://github.com/3Hren/msgpack-rust/issues/305
            Err((ref b, ..)) => unsafe { mem::transmute(&b[..]) },
        };

        se.serialize_str(s)
    }

Error (BrokenBitPatterns:): Potential broken bit patterns issue in `<RawRef<'a> as serde::Serialize>::serialize`
-> rmp-serde/src/lib.rs:327:5: 339:6
fn serialize<S>(&self, se: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = match self.s {
            Ok(ref s) => s,
            // FIXME: this is invalid. It should use a newtype hack instead.
            // https://github.com/3Hren/msgpack-rust/issues/305
            Err((ref b, ..)) => unsafe { mem::transmute(b) },
        };

        se.serialize_str(s)
    }
```