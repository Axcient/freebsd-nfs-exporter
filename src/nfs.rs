use std::{
    io::{Error, Result},
    mem,
    os::raw::c_void
};

mod ffi;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NfsStat {
    pub getattr: u64,
    pub setattr: u64,
    pub lookup: u64,
    pub readlink: u64,
    pub read: u64,
    pub write: u64,
    pub create: u64,
    pub remove: u64,
    pub rename: u64,
    pub link: u64,
    pub symlink: u64,
    pub mkdir: u64,
    pub rmdir: u64,
    pub readdir: u64,
    pub readdirplus: u64,
    pub access: u64,
    pub mknod: u64,
    pub fsstat: u64,
    pub fsinfo: u64,
    pub pathconf: u64,
    pub commit: u64,
}

pub fn collect() -> Result<NfsStat> {
    let mut raw = ffi::nfsstatsv1::default();
    raw.vers = ffi::NFSSTATS_V1 as i32;
	//let mut raw = mem::MaybeUninit::<ffi::nfsstatsv1>::uninit();
    let flag = ffi::NFSSVC_GETSTATS | ffi::NFSSVC_NEWSTRUCT;
	let raw = unsafe {
        let r = ffi::nfssvc(flag as i32, &mut raw as *mut  _ as *mut c_void);
        //let r = ffi::nfssvc(flag, raw.as_mut_ptr() as *mut c_void);
        if r != 0 {
            return Err(Error::last_os_error());
        }
        raw
        //raw.assume_init()
    };
    //dbg!(&raw);
	Ok(NfsStat{
        getattr: raw.srvrpccnt[ffi::NFSV4OP_GETATTR as usize],
        setattr: raw.srvrpccnt[ffi::NFSV4OP_SETATTR as usize],
        lookup: raw.srvrpccnt[ffi::NFSV4OP_LOOKUP as usize],
        readlink: raw.srvrpccnt[ffi::NFSV4OP_READLINK as usize],
        read: raw.srvrpccnt[ffi::NFSV4OP_READ as usize],
        write: raw.srvrpccnt[ffi::NFSV4OP_WRITE as usize],
        create: raw.srvrpccnt[ffi::NFSV4OP_CREATE as usize],
        remove: raw.srvrpccnt[ffi::NFSV4OP_REMOVE as usize],
        rename: raw.srvrpccnt[ffi::NFSV4OP_RENAME as usize],
        link: raw.srvrpccnt[ffi::NFSV4OP_LINK as usize],
        symlink: raw.srvrpccnt[ffi::NFSV4OP_SYMLINK as usize],
        mkdir: raw.srvrpccnt[ffi::NFSV4OP_MKDIR as usize],
        rmdir: raw.srvrpccnt[ffi::NFSV4OP_RMDIR as usize],
        readdir: raw.srvrpccnt[ffi::NFSV4OP_READDIR as usize],
        readdirplus: raw.srvrpccnt[ffi::NFSV4OP_READDIRPLUS as usize],
        access: raw.srvrpccnt[ffi::NFSV4OP_ACCESS as usize],
        mknod: raw.srvrpccnt[ffi::NFSV4OP_MKNOD as usize],
        fsstat: raw.srvrpccnt[ffi::NFSV4OP_FSSTAT as usize],
        fsinfo: raw.srvrpccnt[ffi::NFSV4OP_FSINFO as usize],
        pathconf: raw.srvrpccnt[ffi::NFSV4OP_PATHCONF as usize],
        commit: raw.srvrpccnt[ffi::NFSV4OP_COMMIT as usize],
    })
}
