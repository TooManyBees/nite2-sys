# nite2-sys: NiTE2 Bindings

Rust bindings for NiTE2

The reference vendor files were built with bindgen using this command, with a few additional edits:

`bindgen vendor/NiteCAPI.h -o ~src/lib.rs --whitelist-function nite.* --opaque-type Oni.* --whitelist-type Nite.* --whitelist-var NITE_.* -- -x c++ -I../OpenNI2.2/Include`

# Compilation

NiTE2 usually expects to be dynamically linked, and requires env variables
to indicate where the libraries are.

When building on Windows, the build script checks the presence of the env vars
`NITE2_LIB` and `NITE2_LIB64` (per the NiTE2forms, it checks
`NITE2_REDIST` and `NITE2_REDIST64`, which should be the location of
`libNiTE2.dylib` or `libNiTE2.so` on OSX or Linux.

(A Windows NiTE2 installation should also have the `NITE2_REDIST(64)` env
var set, but it's not the location needed to correctly link.)

# Runtime considerations

# Runtime considerations

For OSX, add `NITE2_REDIST(64)` (the location of `libNiTE2.dylib`)
to your `DYLD_LIBRARY_PATH` env var.

For Linux, add `NITE2_REDIST(64)` (the location of `libNiTE2.so`)
to your `LD_LIBRARY_PATH` env var.

For Windows, add `NITE2_REDIST(64)` to your `PATH`.

Otherwise to avoid using shared locations, copy `NiTE2.dll`,
`libNiTE2.dylib`, or `libNiTE2.so` to the executable's directory.

# LICENSE

These bindings are distributed under the MIT license, which I don't exactly
know what it means, but was recommended and idgaf.
