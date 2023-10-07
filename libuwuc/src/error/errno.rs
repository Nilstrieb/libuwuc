/* modified by hand to add the Error impls and make them i32! */
/* automatically generated by rust-bindgen 0.66.1 */
/* bindgen /nix/store/y8wfrgk7br5rfz4221lfb9v8w3n0cnyd-glibc-2.37-8-dev/include/bits/errno.h -- -D_ERRNO_H > errno.rs */

pub const EPERM: i32 = 1;
pub const ENOENT: i32 = 2;
pub const ESRCH: i32 = 3;
pub const EINTR: i32 = 4;
pub const EIO: i32 = 5;
pub const ENXIO: i32 = 6;
pub const E2BIG: i32 = 7;
pub const ENOEXEC: i32 = 8;
pub const EBADF: i32 = 9;
pub const ECHILD: i32 = 10;
pub const EAGAIN: i32 = 11;
pub const ENOMEM: i32 = 12;
pub const EACCES: i32 = 13;
pub const EFAULT: i32 = 14;
pub const ENOTBLK: i32 = 15;
pub const EBUSY: i32 = 16;
pub const EEXIST: i32 = 17;
pub const EXDEV: i32 = 18;
pub const ENODEV: i32 = 19;
pub const ENOTDIR: i32 = 20;
pub const EISDIR: i32 = 21;
pub const EINVAL: i32 = 22;
pub const ENFILE: i32 = 23;
pub const EMFILE: i32 = 24;
pub const ENOTTY: i32 = 25;
pub const ETXTBSY: i32 = 26;
pub const EFBIG: i32 = 27;
pub const ENOSPC: i32 = 28;
pub const ESPIPE: i32 = 29;
pub const EROFS: i32 = 30;
pub const EMLINK: i32 = 31;
pub const EPIPE: i32 = 32;
pub const EDOM: i32 = 33;
pub const ERANGE: i32 = 34;
pub const EDEADLK: i32 = 35;
pub const ENAMETOOLONG: i32 = 36;
pub const ENOLCK: i32 = 37;
pub const ENOSYS: i32 = 38;
pub const ENOTEMPTY: i32 = 39;
pub const ELOOP: i32 = 40;
pub const EWOULDBLOCK: i32 = 11;
pub const ENOMSG: i32 = 42;
pub const EIDRM: i32 = 43;
pub const ECHRNG: i32 = 44;
pub const EL2NSYNC: i32 = 45;
pub const EL3HLT: i32 = 46;
pub const EL3RST: i32 = 47;
pub const ELNRNG: i32 = 48;
pub const EUNATCH: i32 = 49;
pub const ENOCSI: i32 = 50;
pub const EL2HLT: i32 = 51;
pub const EBADE: i32 = 52;
pub const EBADR: i32 = 53;
pub const EXFULL: i32 = 54;
pub const ENOANO: i32 = 55;
pub const EBADRQC: i32 = 56;
pub const EBADSLT: i32 = 57;
pub const EDEADLOCK: i32 = 35;
pub const EBFONT: i32 = 59;
pub const ENOSTR: i32 = 60;
pub const ENODATA: i32 = 61;
pub const ETIME: i32 = 62;
pub const ENOSR: i32 = 63;
pub const ENONET: i32 = 64;
pub const ENOPKG: i32 = 65;
pub const EREMOTE: i32 = 66;
pub const ENOLINK: i32 = 67;
pub const EADV: i32 = 68;
pub const ESRMNT: i32 = 69;
pub const ECOMM: i32 = 70;
pub const EPROTO: i32 = 71;
pub const EMULTIHOP: i32 = 72;
pub const EDOTDOT: i32 = 73;
pub const EBADMSG: i32 = 74;
pub const EOVERFLOW: i32 = 75;
pub const ENOTUNIQ: i32 = 76;
pub const EBADFD: i32 = 77;
pub const EREMCHG: i32 = 78;
pub const ELIBACC: i32 = 79;
pub const ELIBBAD: i32 = 80;
pub const ELIBSCN: i32 = 81;
pub const ELIBMAX: i32 = 82;
pub const ELIBEXEC: i32 = 83;
pub const EILSEQ: i32 = 84;
pub const ERESTART: i32 = 85;
pub const ESTRPIPE: i32 = 86;
pub const EUSERS: i32 = 87;
pub const ENOTSOCK: i32 = 88;
pub const EDESTADDRREQ: i32 = 89;
pub const EMSGSIZE: i32 = 90;
pub const EPROTOTYPE: i32 = 91;
pub const ENOPROTOOPT: i32 = 92;
pub const EPROTONOSUPPORT: i32 = 93;
pub const ESOCKTNOSUPPORT: i32 = 94;
pub const EOPNOTSUPP: i32 = 95;
pub const EPFNOSUPPORT: i32 = 96;
pub const EAFNOSUPPORT: i32 = 97;
pub const EADDRINUSE: i32 = 98;
pub const EADDRNOTAVAIL: i32 = 99;
pub const ENETDOWN: i32 = 100;
pub const ENETUNREACH: i32 = 101;
pub const ENETRESET: i32 = 102;
pub const ECONNABORTED: i32 = 103;
pub const ECONNRESET: i32 = 104;
pub const ENOBUFS: i32 = 105;
pub const EISCONN: i32 = 106;
pub const ENOTCONN: i32 = 107;
pub const ESHUTDOWN: i32 = 108;
pub const ETOOMANYREFS: i32 = 109;
pub const ETIMEDOUT: i32 = 110;
pub const ECONNREFUSED: i32 = 111;
pub const EHOSTDOWN: i32 = 112;
pub const EHOSTUNREACH: i32 = 113;
pub const EALREADY: i32 = 114;
pub const EINPROGRESS: i32 = 115;
pub const ESTALE: i32 = 116;
pub const EUCLEAN: i32 = 117;
pub const ENOTNAM: i32 = 118;
pub const ENAVAIL: i32 = 119;
pub const EISNAM: i32 = 120;
pub const EREMOTEIO: i32 = 121;
pub const EDQUOT: i32 = 122;
pub const ENOMEDIUM: i32 = 123;
pub const EMEDIUMTYPE: i32 = 124;
pub const ECANCELED: i32 = 125;
pub const ENOKEY: i32 = 126;
pub const EKEYEXPIRED: i32 = 127;
pub const EKEYREVOKED: i32 = 128;
pub const EKEYREJECTED: i32 = 129;
pub const EOWNERDEAD: i32 = 130;
pub const ENOTRECOVERABLE: i32 = 131;
pub const ERFKILL: i32 = 132;
pub const EHWPOISON: i32 = 133;
pub const ENOTSUP: i32 = 95;

impl super::Error {
    pub const PERM: Self = Self(EPERM);
    pub const NOENT: Self = Self(ENOENT);
    pub const SRCH: Self = Self(ESRCH);
    pub const INTR: Self = Self(EINTR);
    pub const IO: Self = Self(EIO);
    pub const NXIO: Self = Self(ENXIO);
    pub const E2BIG: Self = Self(E2BIG);
    pub const NOEXEC: Self = Self(ENOEXEC);
    pub const BADF: Self = Self(EBADF);
    pub const CHILD: Self = Self(ECHILD);
    pub const AGAIN: Self = Self(EAGAIN);
    pub const NOMEM: Self = Self(ENOMEM);
    pub const ACCES: Self = Self(EACCES);
    pub const FAULT: Self = Self(EFAULT);
    pub const NOTBLK: Self = Self(ENOTBLK);
    pub const BUSY: Self = Self(EBUSY);
    pub const EXIST: Self = Self(EEXIST);
    pub const XDEV: Self = Self(EXDEV);
    pub const NODEV: Self = Self(ENODEV);
    pub const NOTDIR: Self = Self(ENOTDIR);
    pub const ISDIR: Self = Self(EISDIR);
    pub const INVAL: Self = Self(EINVAL);
    pub const NFILE: Self = Self(ENFILE);
    pub const MFILE: Self = Self(EMFILE);
    pub const NOTTY: Self = Self(ENOTTY);
    pub const TXTBSY: Self = Self(ETXTBSY);
    pub const FBIG: Self = Self(EFBIG);
    pub const NOSPC: Self = Self(ENOSPC);
    pub const SPIPE: Self = Self(ESPIPE);
    pub const ROFS: Self = Self(EROFS);
    pub const MLINK: Self = Self(EMLINK);
    pub const PIPE: Self = Self(EPIPE);
    pub const DOM: Self = Self(EDOM);
    pub const RANGE: Self = Self(ERANGE);
    pub const DEADLK: Self = Self(EDEADLK);
    pub const NAMETOOLONG: Self = Self(ENAMETOOLONG);
    pub const NOLCK: Self = Self(ENOLCK);
    pub const NOSYS: Self = Self(ENOSYS);
    pub const NOTEMPTY: Self = Self(ENOTEMPTY);
    pub const LOOP: Self = Self(ELOOP);
    pub const WOULDBLOCK: Self = Self(EWOULDBLOCK);
    pub const NOMSG: Self = Self(ENOMSG);
    pub const IDRM: Self = Self(EIDRM);
    pub const CHRNG: Self = Self(ECHRNG);
    pub const L2NSYNC: Self = Self(EL2NSYNC);
    pub const L3HLT: Self = Self(EL3HLT);
    pub const L3RST: Self = Self(EL3RST);
    pub const LNRNG: Self = Self(ELNRNG);
    pub const UNATCH: Self = Self(EUNATCH);
    pub const NOCSI: Self = Self(ENOCSI);
    pub const L2HLT: Self = Self(EL2HLT);
    pub const BADE: Self = Self(EBADE);
    pub const BADR: Self = Self(EBADR);
    pub const XFULL: Self = Self(EXFULL);
    pub const NOANO: Self = Self(ENOANO);
    pub const BADRQC: Self = Self(EBADRQC);
    pub const BADSLT: Self = Self(EBADSLT);
    pub const DEADLOCK: Self = Self(EDEADLOCK);
    pub const BFONT: Self = Self(EBFONT);
    pub const NOSTR: Self = Self(ENOSTR);
    pub const NODATA: Self = Self(ENODATA);
    pub const TIME: Self = Self(ETIME);
    pub const NOSR: Self = Self(ENOSR);
    pub const NONET: Self = Self(ENONET);
    pub const NOPKG: Self = Self(ENOPKG);
    pub const REMOTE: Self = Self(EREMOTE);
    pub const NOLINK: Self = Self(ENOLINK);
    pub const ADV: Self = Self(EADV);
    pub const SRMNT: Self = Self(ESRMNT);
    pub const COMM: Self = Self(ECOMM);
    pub const PROTO: Self = Self(EPROTO);
    pub const MULTIHOP: Self = Self(EMULTIHOP);
    pub const DOTDOT: Self = Self(EDOTDOT);
    pub const BADMSG: Self = Self(EBADMSG);
    pub const OVERFLOW: Self = Self(EOVERFLOW);
    pub const NOTUNIQ: Self = Self(ENOTUNIQ);
    pub const BADFD: Self = Self(EBADFD);
    pub const REMCHG: Self = Self(EREMCHG);
    pub const LIBACC: Self = Self(ELIBACC);
    pub const LIBBAD: Self = Self(ELIBBAD);
    pub const LIBSCN: Self = Self(ELIBSCN);
    pub const LIBMAX: Self = Self(ELIBMAX);
    pub const LIBEXEC: Self = Self(ELIBEXEC);
    pub const ILSEQ: Self = Self(EILSEQ);
    pub const RESTART: Self = Self(ERESTART);
    pub const STRPIPE: Self = Self(ESTRPIPE);
    pub const USERS: Self = Self(EUSERS);
    pub const NOTSOCK: Self = Self(ENOTSOCK);
    pub const DESTADDRREQ: Self = Self(EDESTADDRREQ);
    pub const MSGSIZE: Self = Self(EMSGSIZE);
    pub const PROTOTYPE: Self = Self(EPROTOTYPE);
    pub const NOPROTOOPT: Self = Self(ENOPROTOOPT);
    pub const PROTONOSUPPORT: Self = Self(EPROTONOSUPPORT);
    pub const SOCKTNOSUPPORT: Self = Self(ESOCKTNOSUPPORT);
    pub const OPNOTSUPP: Self = Self(EOPNOTSUPP);
    pub const PFNOSUPPORT: Self = Self(EPFNOSUPPORT);
    pub const AFNOSUPPORT: Self = Self(EAFNOSUPPORT);
    pub const ADDRINUSE: Self = Self(EADDRINUSE);
    pub const ADDRNOTAVAIL: Self = Self(EADDRNOTAVAIL);
    pub const NETDOWN: Self = Self(ENETDOWN);
    pub const NETUNREACH: Self = Self(ENETUNREACH);
    pub const NETRESET: Self = Self(ENETRESET);
    pub const CONNABORTED: Self = Self(ECONNABORTED);
    pub const CONNRESET: Self = Self(ECONNRESET);
    pub const NOBUFS: Self = Self(ENOBUFS);
    pub const ISCONN: Self = Self(EISCONN);
    pub const NOTCONN: Self = Self(ENOTCONN);
    pub const SHUTDOWN: Self = Self(ESHUTDOWN);
    pub const TOOMANYREFS: Self = Self(ETOOMANYREFS);
    pub const TIMEDOUT: Self = Self(ETIMEDOUT);
    pub const CONNREFUSED: Self = Self(ECONNREFUSED);
    pub const HOSTDOWN: Self = Self(EHOSTDOWN);
    pub const HOSTUNREACH: Self = Self(EHOSTUNREACH);
    pub const ALREADY: Self = Self(EALREADY);
    pub const INPROGRESS: Self = Self(EINPROGRESS);
    pub const STALE: Self = Self(ESTALE);
    pub const UCLEAN: Self = Self(EUCLEAN);
    pub const NOTNAM: Self = Self(ENOTNAM);
    pub const NAVAIL: Self = Self(ENAVAIL);
    pub const ISNAM: Self = Self(EISNAM);
    pub const REMOTEIO: Self = Self(EREMOTEIO);
    pub const DQUOT: Self = Self(EDQUOT);
    pub const NOMEDIUM: Self = Self(ENOMEDIUM);
    pub const MEDIUMTYPE: Self = Self(EMEDIUMTYPE);
    pub const CANCELED: Self = Self(ECANCELED);
    pub const NOKEY: Self = Self(ENOKEY);
    pub const KEYEXPIRED: Self = Self(EKEYEXPIRED);
    pub const KEYREVOKED: Self = Self(EKEYREVOKED);
    pub const KEYREJECTED: Self = Self(EKEYREJECTED);
    pub const OWNERDEAD: Self = Self(EOWNERDEAD);
    pub const NOTRECOVERABLE: Self = Self(ENOTRECOVERABLE);
    pub const RFKILL: Self = Self(ERFKILL);
    pub const HWPOISON: Self = Self(EHWPOISON);
    pub const NOTSUP: Self = Self(ENOTSUP);

    pub fn simple_str(self) -> &'static str {
        match self {
            Self::PERM => "EPERM",
            Self::NOENT => "ENOENT",
            Self::SRCH => "ESRCH",
            Self::INTR => "EINTR",
            Self::IO => "EIO",
            Self::NXIO => "ENXIO",
            Self::E2BIG => "E2BIG",
            Self::NOEXEC => "ENOEXEC",
            Self::BADF => "EBADF",
            Self::CHILD => "ECHILD",
            Self::AGAIN => "EAGAIN",
            Self::NOMEM => "ENOMEM",
            Self::ACCES => "EACCES",
            Self::FAULT => "EFAULT",
            Self::NOTBLK => "ENOTBLK",
            Self::BUSY => "EBUSY",
            Self::EXIST => "EEXIST",
            Self::XDEV => "EXDEV",
            Self::NODEV => "ENODEV",
            Self::NOTDIR => "ENOTDIR",
            Self::ISDIR => "EISDIR",
            Self::INVAL => "EINVAL",
            Self::NFILE => "ENFILE",
            Self::MFILE => "EMFILE",
            Self::NOTTY => "ENOTTY",
            Self::TXTBSY => "ETXTBSY",
            Self::FBIG => "EFBIG",
            Self::NOSPC => "ENOSPC",
            Self::SPIPE => "ESPIPE",
            Self::ROFS => "EROFS",
            Self::MLINK => "EMLINK",
            Self::PIPE => "EPIPE",
            Self::DOM => "EDOM",
            Self::RANGE => "ERANGE",
            Self::DEADLK => "EDEADLK",
            Self::NAMETOOLONG => "ENAMETOOLONG",
            Self::NOLCK => "ENOLCK",
            Self::NOSYS => "ENOSYS",
            Self::NOTEMPTY => "ENOTEMPTY",
            Self::LOOP => "ELOOP",
            Self::NOMSG => "ENOMSG",
            Self::IDRM => "EIDRM",
            Self::CHRNG => "ECHRNG",
            Self::L2NSYNC => "EL2NSYNC",
            Self::L3HLT => "EL3HLT",
            Self::L3RST => "EL3RST",
            Self::LNRNG => "ELNRNG",
            Self::UNATCH => "EUNATCH",
            Self::NOCSI => "ENOCSI",
            Self::L2HLT => "EL2HLT",
            Self::BADE => "EBADE",
            Self::BADR => "EBADR",
            Self::XFULL => "EXFULL",
            Self::NOANO => "ENOANO",
            Self::BADRQC => "EBADRQC",
            Self::BADSLT => "EBADSLT",
            Self::BFONT => "EBFONT",
            Self::NOSTR => "ENOSTR",
            Self::NODATA => "ENODATA",
            Self::TIME => "ETIME",
            Self::NOSR => "ENOSR",
            Self::NONET => "ENONET",
            Self::NOPKG => "ENOPKG",
            Self::REMOTE => "EREMOTE",
            Self::NOLINK => "ENOLINK",
            Self::ADV => "EADV",
            Self::SRMNT => "ESRMNT",
            Self::COMM => "ECOMM",
            Self::PROTO => "EPROTO",
            Self::MULTIHOP => "EMULTIHOP",
            Self::DOTDOT => "EDOTDOT",
            Self::BADMSG => "EBADMSG",
            Self::OVERFLOW => "EOVERFLOW",
            Self::NOTUNIQ => "ENOTUNIQ",
            Self::BADFD => "EBADFD",
            Self::REMCHG => "EREMCHG",
            Self::LIBACC => "ELIBACC",
            Self::LIBBAD => "ELIBBAD",
            Self::LIBSCN => "ELIBSCN",
            Self::LIBMAX => "ELIBMAX",
            Self::LIBEXEC => "ELIBEXEC",
            Self::ILSEQ => "EILSEQ",
            Self::RESTART => "ERESTART",
            Self::STRPIPE => "ESTRPIPE",
            Self::USERS => "EUSERS",
            Self::NOTSOCK => "ENOTSOCK",
            Self::DESTADDRREQ => "EDESTADDRREQ",
            Self::MSGSIZE => "EMSGSIZE",
            Self::PROTOTYPE => "EPROTOTYPE",
            Self::NOPROTOOPT => "ENOPROTOOPT",
            Self::PROTONOSUPPORT => "EPROTONOSUPPORT",
            Self::SOCKTNOSUPPORT => "ESOCKTNOSUPPORT",
            Self::OPNOTSUPP => "EOPNOTSUPP",
            Self::PFNOSUPPORT => "EPFNOSUPPORT",
            Self::AFNOSUPPORT => "EAFNOSUPPORT",
            Self::ADDRINUSE => "EADDRINUSE",
            Self::ADDRNOTAVAIL => "EADDRNOTAVAIL",
            Self::NETDOWN => "ENETDOWN",
            Self::NETUNREACH => "ENETUNREACH",
            Self::NETRESET => "ENETRESET",
            Self::CONNABORTED => "ECONNABORTED",
            Self::CONNRESET => "ECONNRESET",
            Self::NOBUFS => "ENOBUFS",
            Self::ISCONN => "EISCONN",
            Self::NOTCONN => "ENOTCONN",
            Self::SHUTDOWN => "ESHUTDOWN",
            Self::TOOMANYREFS => "ETOOMANYREFS",
            Self::TIMEDOUT => "ETIMEDOUT",
            Self::CONNREFUSED => "ECONNREFUSED",
            Self::HOSTDOWN => "EHOSTDOWN",
            Self::HOSTUNREACH => "EHOSTUNREACH",
            Self::ALREADY => "EALREADY",
            Self::INPROGRESS => "EINPROGRESS",
            Self::STALE => "ESTALE",
            Self::UCLEAN => "EUCLEAN",
            Self::NOTNAM => "ENOTNAM",
            Self::NAVAIL => "ENAVAIL",
            Self::ISNAM => "EISNAM",
            Self::REMOTEIO => "EREMOTEIO",
            Self::DQUOT => "EDQUOT",
            Self::NOMEDIUM => "ENOMEDIUM",
            Self::MEDIUMTYPE => "EMEDIUMTYPE",
            Self::CANCELED => "ECANCELED",
            Self::NOKEY => "ENOKEY",
            Self::KEYEXPIRED => "EKEYEXPIRED",
            Self::KEYREVOKED => "EKEYREVOKED",
            Self::KEYREJECTED => "EKEYREJECTED",
            Self::OWNERDEAD => "EOWNERDEAD",
            Self::NOTRECOVERABLE => "ENOTRECOVERABLE",
            Self::RFKILL => "ERFKILL",
            Self::HWPOISON => "EHWPOISON",
            _ => "<invalid error>",
        }
    }
}