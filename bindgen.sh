#! /bin/sh
# Run this script to regenerate src/nfs/ffi.rs
bindgen --whitelist-function nfssvc --whitelist-type nfsstatsv1 --whitelist-var 'NFS.*' --with-derive-default bindgen.h > src/nfs/ffi.rs
