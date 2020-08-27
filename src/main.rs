// vim: tw=80

use env_logger::{
    Builder,
    Env,
};
use prometheus_exporter::{
    FinishedUpdate,
    PrometheusExporter,
    prometheus::register_int_gauge,
};
use std::{
    convert::TryInto,
    net::SocketAddr
};

mod nfs;

fn main() {
    // Setup logger with default level info so we can see the messages from
    // prometheus_exporter.
    Builder::from_env(Env::default().default_filter_or("info")).init();

    // Parse address used to bind exporter to.
    let addr_raw = "0.0.0.0:9185";
    let addr: SocketAddr = addr_raw.parse().expect("can not parse listen addr");

    // Start exporter.
    let (request_receiver, finished_sender) = PrometheusExporter::run_and_notify(addr);

    // Create metrics
    // Even though these are gauge, we use the Gauge API since the kernel
    // reports their current values.
    let getattr = register_int_gauge!("getattr", "Count of server GETATTR RPCs")
        .expect("can not create gauge");
    let setattr = register_int_gauge!("setattr", "Count of server SETATTR RPCs")
        .expect("can not create gauge");
    let lookup = register_int_gauge!("lookup", "Count of server LOOKUP RPCs")
        .expect("can not create gauge");
    let readlink = register_int_gauge!("readlink", "Count of server READLINK RPCs")
        .expect("can not create gauge");
    let read = register_int_gauge!("read", "Count of server READ RPCs")
        .expect("can not create gauge");
    let write = register_int_gauge!("write", "Count of server WRITE RPCs")
        .expect("can not create gauge");
    let create = register_int_gauge!("create", "Count of server CREATE RPCs")
        .expect("can not create gauge");
    let remove = register_int_gauge!("remove", "Count of server REMOVE RPCs")
        .expect("can not create gauge");
    let rename = register_int_gauge!("rename", "Count of server RENAME RPCs")
        .expect("can not create gauge");
    let link = register_int_gauge!("link", "Count of server LINK RPCs")
        .expect("can not create gauge");
    let symlink = register_int_gauge!("symlink", "Count of server SYMLINK RPCs")
        .expect("can not create gauge");
    let mkdir = register_int_gauge!("mkdir", "Count of server MKDIR RPCs")
        .expect("can not create gauge");
    let rmdir = register_int_gauge!("rmdir", "Count of server RMDIR RPCs")
        .expect("can not create gauge");
    let readdir = register_int_gauge!("readdir", "Count of server READDIR RPCs")
        .expect("can not create gauge");
    let readdirplus = register_int_gauge!("readdirplus", "Count of server READDIRPLUS RPCs")
        .expect("can not create gauge");
    let access = register_int_gauge!("access", "Count of server ACCESS RPCs")
        .expect("can not create gauge");
    let mknod = register_int_gauge!("mknod", "Count of server MKNOD RPCs")
        .expect("can not create gauge");
    let fsstat = register_int_gauge!("fsstat", "Count of server FSSTAT RPCs")
        .expect("can not create gauge");
    let fsinfo = register_int_gauge!("fsinfo", "Count of server FSINFO RPCs")
        .expect("can not create gauge");
    let pathconf = register_int_gauge!("pathconf", "Count of server PATHCONF RPCs")
        .expect("can not create gauge");
    let commit = register_int_gauge!("commit", "Count of server COMMIT RPCs")
        .expect("can not create gauge");

    loop {
        // Will block until exporter receives http request.
        request_receiver.recv().unwrap();

        // Update metric with random value.
        let nfs_stat = nfs::collect().unwrap();
        getattr.set(nfs_stat.getattr.try_into().unwrap());
        setattr.set(nfs_stat.setattr.try_into().unwrap());
        lookup.set(nfs_stat.lookup.try_into().unwrap());
        readlink.set(nfs_stat.readlink.try_into().unwrap());
        read.set(nfs_stat.read.try_into().unwrap());
        write.set(nfs_stat.write.try_into().unwrap());
        create.set(nfs_stat.create.try_into().unwrap());
        remove.set(nfs_stat.remove.try_into().unwrap());
        rename.set(nfs_stat.rename.try_into().unwrap());
        link.set(nfs_stat.link.try_into().unwrap());
        symlink.set(nfs_stat.symlink.try_into().unwrap());
        mkdir.set(nfs_stat.mkdir.try_into().unwrap());
        rmdir.set(nfs_stat.rmdir.try_into().unwrap());
        readdir.set(nfs_stat.readdir.try_into().unwrap());
        readdirplus.set(nfs_stat.readdirplus.try_into().unwrap());
        access.set(nfs_stat.access.try_into().unwrap());
        mknod.set(nfs_stat.mknod.try_into().unwrap());
        fsstat.set(nfs_stat.fsstat.try_into().unwrap());
        fsinfo.set(nfs_stat.fsinfo.try_into().unwrap());
        pathconf.set(nfs_stat.pathconf.try_into().unwrap());
        commit.set(nfs_stat.commit.try_into().unwrap());

        // Notify exporter that all metrics have been updated so the caller client can
        // receive a response.
        finished_sender.send(FinishedUpdate).unwrap();
    }
}
