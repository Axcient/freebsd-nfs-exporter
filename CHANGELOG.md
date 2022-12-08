# Change Log

All notable changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](https://semver.org/).

## [0.3.2] - 2022-12-08

### Fixed

- Fixed a crash if any counter exceeded `i64::MAX_VALUE`.
  (#[17](https://github.com/Axcient/freebsd-nfs-exporter/pull/17))

## [0.3.1] - 2021-08-17
### Fixed

- Fixed the `nfs_nfsd_total_bytes` and `nfs_nfsd_total_duration` gauges on
  FreeBSD 13.0.
  (#[5](https://github.com/Axcient/freebsd-nfs-exporter/pull/5))
- Fixed parsing IPv6 addresses on the command line.
  (#[6](https://github.com/Axcient/freebsd-nfs-exporter/pull/6))
- Fixed several security vulnerabilities in dependencies: RUSTSEC-2021-0079,
  RUSTSEC-2021-0078, RUSTSEC-2021-0020, RUSTSEC-2021-0003.
  (#[7](https://github.com/Axcient/freebsd-nfs-exporter/pull/7))
