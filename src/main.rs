// vim: tw=80

use env_logger::{
    Builder,
    Env,
};
use prometheus_exporter::{
    FinishedUpdate,
    PrometheusExporter,
    prometheus::register_int_gauge_vec,
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
    let rpcs = register_int_gauge_vec!("nfs_nfsd_requests_total",
                                       "Count of server RPCs",
                                       &["method"])
        .expect("can not create gauge");

    loop {
        // Will block until exporter receives http request.
        request_receiver.recv().unwrap();

        // Update metric with random value.
        let nfs_stat = nfs::collect().unwrap();
        rpcs.with_label_values(&["Access"]).set(nfs_stat.access.try_into().unwrap());
        rpcs.with_label_values(&["Commit"]).set(nfs_stat.commit.try_into().unwrap());
        rpcs.with_label_values(&["Create"]).set(nfs_stat.create.try_into().unwrap());
        rpcs.with_label_values(&["FsInfo"]).set(nfs_stat.fsinfo.try_into().unwrap());
        rpcs.with_label_values(&["FsStat"]).set(nfs_stat.fsstat.try_into().unwrap());
        rpcs.with_label_values(&["GetAttr"]).set(nfs_stat.getattr.try_into().unwrap());
        rpcs.with_label_values(&["Link"]).set(nfs_stat.link.try_into().unwrap());
        rpcs.with_label_values(&["Lookup"]).set(nfs_stat.lookup.try_into().unwrap());
        rpcs.with_label_values(&["MkDir"]).set(nfs_stat.mkdir.try_into().unwrap());
        rpcs.with_label_values(&["MkNod"]).set(nfs_stat.mknod.try_into().unwrap());
        rpcs.with_label_values(&["PathConf"]).set(nfs_stat.pathconf.try_into().unwrap());
        rpcs.with_label_values(&["Read"]).set(nfs_stat.read.try_into().unwrap());
        rpcs.with_label_values(&["SetAttr"]).set(nfs_stat.setattr.try_into().unwrap());
        rpcs.with_label_values(&["ReadDir"]).set(nfs_stat.readdir.try_into().unwrap());
        rpcs.with_label_values(&["ReadDirPlus"]).set(nfs_stat.readdirplus.try_into().unwrap());
        rpcs.with_label_values(&["ReadLink"]).set(nfs_stat.readlink.try_into().unwrap());
        rpcs.with_label_values(&["Remove"]).set(nfs_stat.remove.try_into().unwrap());
        rpcs.with_label_values(&["Rename"]).set(nfs_stat.rename.try_into().unwrap());
        rpcs.with_label_values(&["RmDir"]).set(nfs_stat.rmdir.try_into().unwrap());
        rpcs.with_label_values(&["SymLink"]).set(nfs_stat.symlink.try_into().unwrap());
        rpcs.with_label_values(&["Write"]).set(nfs_stat.write.try_into().unwrap());
        // Notify exporter that all metrics have been updated so the caller client can
        // receive a response.
        finished_sender.send(FinishedUpdate).unwrap();
    }
}
