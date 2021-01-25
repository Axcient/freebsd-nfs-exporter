# FreeBSD NFS statistics exporter for Prometheus

[![Build Status](https://api.cirrus-ci.com/github/Axcient/freebsd-nfs-exporter.svg)](https://cirrus-ci.com/github/Axcient/freebsd-nfs-exporter)
[![crates.io](https://img.shields.io/crates/v/freebsd-nfs-exporter.svg)](https://crates.io/crates/freebsd-nfs-exporter)

## Overview

The is a [Prometheus](http://prometheus.io) exporter for
[FreeBSD's](http://www.freebsd.org) NFS statistics.  Currently only the server
statistics are supported; client stats will come later.

## Usage

```
cargo install freebsd-nfs-exporter
daemon nfs-exporter
```

# License

`freebsd-nfs-exporter` is primarily distributed under the terms of both the MIT
license and the Apache License (Version 2.0).

See LICENSE-APACHE, and LICENSE-MIT for details
