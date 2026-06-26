#! /bin/sh

CRATEDIR=`dirname $0`/..
OUTFILE=$1
if [ -z "$OUTFILE" ]; then
	case $(uname -m) in
	i386)
		OUTFILE=${CRATEDIR}/src/ffi32.rs
		;;
	armv7)
		OUTFILE=${CRATEDIR}/src/ffi32.rs
		;;
	*)
		OUTFILE=${CRATEDIR}/src/ffi64.rs
		;;
	esac
fi

bindgen --generate functions,types,vars \
	--allowlist-function 'nfssvc' \
	--allowlist-type 'nfsstatsv1' \
	--allowlist-var 'NFSSTATS_V1' \
	--allowlist-var 'NFSSVC_GETSTATS' \
	--allowlist-var 'NFSSVC_NEWSTRUCT' \
	--allowlist-var 'NFSV4OP.*' \
	--with-derive-default \
	${CRATEDIR}/bindgen/wrapper.h > ${OUTFILE}
