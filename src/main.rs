// vim: tw=80

use clap::{self, App, Arg};
use env_logger::{
    Builder,
    Env,
};
use prometheus_exporter::{
    FinishedUpdate,
    PrometheusExporter,
    prometheus::{
        register_int_gauge,
        register_int_gauge_vec,
    }
};
use std::{
    convert::TryInto,
    net::SocketAddr
};

mod nfs;

fn main() {
    let matches = App::new("nfs-exporter")
        .version("0.1")
        .about("Export NFS statistics to Prometheus")
        .arg(Arg::with_name("bind")
             .short("b")
             .value_name("ADDR")
             .help("Bind to this local address")
             .takes_value(true))
        .arg(Arg::with_name("client")
             .short("c")
             .help("Publish NFS client statistics"))
        .arg(Arg::with_name("server")
             .short("s")
             .help("Publish NFS server statistics"))
        .arg(Arg::with_name("port")
             .short("p")
             .value_name("PORT")
             .help("TCP port (default 9898)")
             .takes_value(true))
        .get_matches();
    let addr = matches.value_of("bind").unwrap_or("0.0.0.0");
    let port = matches.value_of("port").unwrap_or("9898");
    let (_c, s) = if !matches.is_present("client") && !matches.is_present("server") {
        // By default, print everything
        (true, true)
    } else if matches.is_present("server") {
        (false, true)
    } else {
        clap::Error::with_description(
            "client stats are TODO",
            clap::ErrorKind::InvalidValue)
            .exit();
    };

    // Setup logger with default level info so we can see the messages from
    // prometheus_exporter.
    Builder::from_env(Env::default().default_filter_or("info")).init();

    // Parse address used to bind exporter to.
    let addr_raw = format!("{}:{}", addr, port);
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
        "Total nfsd nanoseconds spend processing each operation.  May wrap.",
        &["method"])
        .expect("can not create gauge");
    let rpcs = register_int_gauge_vec!("nfs_nfsd_requests_total",
                                       "Count of server RPCs",
                                       &["method"])
        .expect("can not create gauge");
    let startcnt = register_int_gauge!("nfs_nfsd_start_count",
        "Total number of opreations started since boot")
        .expect("can not create gauge");
    let donecnt = register_int_gauge!("nfs_nfsd_done_count",
        "Total number of opreations completed since boot")
        .expect("can not create gauge");
    let busytime = register_int_gauge!("nfs_nfsd_busytime",
        "Total time in ns that nfsd was busy with at least one opeartion")
        .expect("can not create gauge");


    loop {
        // Will block until exporter receives http request.
        request_receiver.recv().unwrap();

        // Update metric with random value.
        let nfs_stat = nfs::collect().unwrap();

        if s {
            macro_rules! set_rpcs {
                ($label:ident, $field:ident) => {
                    rpcs.with_label_values(&[stringify!($label)])
                        .set(nfs_stat.$field.try_into().unwrap());
                };
            }

            bytes.with_label_values(&["Read"])
                .set(nfs_stat.bytes.read.try_into().unwrap());
            bytes.with_label_values(&["Write"])
                .set(nfs_stat.bytes.write.try_into().unwrap());
            duration.with_label_values(&["Read"])
                .set(nfs_stat.duration.read.try_into().unwrap());
            duration.with_label_values(&["Write"])
                .set(nfs_stat.duration.write.try_into().unwrap());
            duration.with_label_values(&["Commit"])
                .set(nfs_stat.duration.commit.try_into().unwrap());
            startcnt.set(nfs_stat.startcnt.try_into().unwrap());
            donecnt.set(nfs_stat.donecnt.try_into().unwrap());
            busytime.set(nfs_stat.busytime.try_into().unwrap());
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
        }

        // Notify exporter that all metrics have been updated so the caller client can
        // receive a response.
        finished_sender.send(FinishedUpdate).unwrap();
    }
}
