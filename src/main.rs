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
    let bytes = register_int_gauge_vec!("nfs_nfsd_total_bytes",
                                        "Total nfsd bytes per operation",
                                        &["method"])
        .expect("can not create gauge");
    let duration = register_int_gauge_vec!("nfs_nfsd_total_duration",
        "Total nfsd time spend processing each operation.  May wrap.",
        &["method"])
        .expect("can not create gauge");
    let rpcs = register_int_gauge_vec!("nfs_nfsd_requests_total",
                                       "Count of server RPCs",
                                       &["method"])
        .expect("can not create gauge");

    loop {
        // Will block until exporter receives http request.
        request_receiver.recv().unwrap();

        // Update metric with random value.
        let nfs_stat = nfs::collect().unwrap();

        macro_rules! set_rpcs {
            ($label:ident, $field:ident) => {
                rpcs.with_label_values(&[stringify!($label)])
                    .set(nfs_stat.$field.try_into().unwrap());
            };
        }

        bytes.with_label_values(&["read"])
            .set(nfs_stat.bytes.read.try_into().unwrap());
        bytes.with_label_values(&["write"])
            .set(nfs_stat.bytes.write.try_into().unwrap());
        duration.with_label_values(&["read"])
            .set(nfs_stat.duration.read.try_into().unwrap());
        duration.with_label_values(&["write"])
            .set(nfs_stat.duration.write.try_into().unwrap());
        duration.with_label_values(&["commit"])
            .set(nfs_stat.duration.commit.try_into().unwrap());
        set_rpcs!(Access, access);
        set_rpcs!(BackChannelCtl, backchannelctrl);
        set_rpcs!(BindConnToSess, bindconntosess);
        set_rpcs!(Close, close);
        set_rpcs!(Commit, commit);
        set_rpcs!(Create, v3create);
        set_rpcs!(CreateSession, createsess);
        set_rpcs!(CreateV4, create);
        set_rpcs!(DelegPurge, delegpurge);
        set_rpcs!(DelegReturn, delegreturn);
        set_rpcs!(DestroyClientId, destroyclid);
        set_rpcs!(DestroySession, destroysess);
        set_rpcs!(ExchangeId, exchangeid);
        set_rpcs!(FreeStateId, freestateid);
        set_rpcs!(FsInfo, fsinfo);
        set_rpcs!(FsStat, fsstat);
        set_rpcs!(GetAttr, getattr);
        set_rpcs!(GetDevInfo, getdevinfo);
        set_rpcs!(GetDevList, getdevlist);
        set_rpcs!(GetDirDeleg, getdirdeleg);
        set_rpcs!(GetFH, getfh);
        set_rpcs!(LayoutCommit, layoutcommit);
        set_rpcs!(LayoutGet, layoutget);
        set_rpcs!(LayoutReturn, layoutreturn);
        set_rpcs!(Link, link);
        set_rpcs!(Lock, lock);
        set_rpcs!(LockT, lockt);
        set_rpcs!(LockU, locku);
        set_rpcs!(Lookup, lookup);
        set_rpcs!(LookupP, lookupp);
        set_rpcs!(MkDir, mkdir);
        set_rpcs!(MkNod, mknod);
        set_rpcs!(Nverify, nverify);
        set_rpcs!(Open, open);
        set_rpcs!(OpenAttr, openattr);
        set_rpcs!(OpenConfirm, openconfirm);
        set_rpcs!(OpenDgrd, opendgrd);
        set_rpcs!(PathConf, pathconf);
        set_rpcs!(PutFH, putfh);
        set_rpcs!(Read, read);
        set_rpcs!(ReadDir, readdir);
        set_rpcs!(ReadDirPlus, readdirplus);
        set_rpcs!(ReadLink, readlink);
        set_rpcs!(ReclaimCompl, reclaimcompl);
        set_rpcs!(RelLockOwner, rellckown);
        set_rpcs!(Remove, remove);
        set_rpcs!(Rename, rename);
        set_rpcs!(Renew, renew);
        set_rpcs!(RestoreFH, restorefh);
        set_rpcs!(RmDir, rmdir);
        set_rpcs!(SaveFH, savefh);
        set_rpcs!(SecInfo, secinfo);
        set_rpcs!(SecInfoNoName, secinfononame);
        set_rpcs!(Sequence, sequence);
        set_rpcs!(SetAttr, setattr);
        set_rpcs!(SetClientId, setclid);
        set_rpcs!(SetClientIdConfirm, setclidcf);
        set_rpcs!(SetSSV, setssv);
        set_rpcs!(SymLink, symlink);
        set_rpcs!(TestStateId, teststateid);
        set_rpcs!(Verify, verify);
        set_rpcs!(WantDeleg, wantdeleg);
        set_rpcs!(Write, write);

        // Notify exporter that all metrics have been updated so the caller client can
        // receive a response.
        finished_sender.send(FinishedUpdate).unwrap();
    }
}
