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

        macro_rules! set_gauge {
            ($label:ident, $field:ident) => {
                rpcs.with_label_values(&[stringify!($label)])
                    .set(nfs_stat.$field.try_into().unwrap());
            };
        }

        set_gauge!(Access, access);
        set_gauge!(BackChannelCtl, backchannelctrl);
        set_gauge!(BindConnToSess, bindconntosess);
        set_gauge!(Close, close);
        set_gauge!(Commit, commit);
        set_gauge!(Create, v3create);
        set_gauge!(CreateSession, createsess);
        set_gauge!(CreateV4, create);
        set_gauge!(DelegPurge, delegpurge);
        set_gauge!(DelegReturn, delegreturn);
        set_gauge!(DestroyClientId, destroyclid);
        set_gauge!(DestroySession, destroysess);
        set_gauge!(ExchangeId, exchangeid);
        set_gauge!(FreeStateId, freestateid);
        set_gauge!(FsInfo, fsinfo);
        set_gauge!(FsStat, fsstat);
        set_gauge!(GetAttr, getattr);
        set_gauge!(GetDevInfo, getdevinfo);
        set_gauge!(GetDevList, getdevlist);
        set_gauge!(GetDirDeleg, getdirdeleg);
        set_gauge!(GetFH, getfh);
        set_gauge!(LayoutCommit, layoutcommit);
        set_gauge!(LayoutGet, layoutget);
        set_gauge!(LayoutReturn, layoutreturn);
        set_gauge!(Link, link);
        set_gauge!(Lock, lock);
        set_gauge!(LockT, lockt);
        set_gauge!(LockU, locku);
        set_gauge!(Lookup, lookup);
        set_gauge!(LookupP, lookupp);
        set_gauge!(MkDir, mkdir);
        set_gauge!(MkNod, mknod);
        set_gauge!(Nverify, nverify);
        set_gauge!(Open, open);
        set_gauge!(OpenAttr, openattr);
        set_gauge!(OpenConfirm, openconfirm);
        set_gauge!(OpenDgrd, opendgrd);
        set_gauge!(PathConf, pathconf);
        set_gauge!(PutFH, putfh);
        set_gauge!(Read, read);
        set_gauge!(ReadDir, readdir);
        set_gauge!(ReadDirPlus, readdirplus);
        set_gauge!(ReadLink, readlink);
        set_gauge!(ReclaimCompl, reclaimcompl);
        set_gauge!(RelLockOwner, rellckown);
        set_gauge!(Remove, remove);
        set_gauge!(Rename, rename);
        set_gauge!(Renew, renew);
        set_gauge!(RestoreFH, restorefh);
        set_gauge!(RmDir, rmdir);
        set_gauge!(SaveFH, savefh);
        set_gauge!(SecInfo, secinfo);
        set_gauge!(SecInfoNoName, secinfononame);
        set_gauge!(Sequence, sequence);
        set_gauge!(SetAttr, setattr);
        set_gauge!(SetClientId, setclid);
        set_gauge!(SetClientIdConfirm, setclidcf);
        set_gauge!(SetSSV, setssv);
        set_gauge!(SymLink, symlink);
        set_gauge!(TestStateId, teststateid);
        set_gauge!(Verify, verify);
        set_gauge!(WantDeleg, wantdeleg);
        set_gauge!(Write, write);

        // Notify exporter that all metrics have been updated so the caller client can
        // receive a response.
        finished_sender.send(FinishedUpdate).unwrap();
    }
}
