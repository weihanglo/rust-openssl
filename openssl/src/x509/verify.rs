use ffi;
use foreign_types::ForeignTypeRef;
use libc::{c_uint, c_ulong};
use std::net::IpAddr;

use cvt;
use error::ErrorStack;

bitflags! {
    /// Flags used to check an `X509` certificate.
    pub struct X509CheckFlags: c_uint {
        const ALWAYS_CHECK_SUBJECT = ffi::X509_CHECK_FLAG_ALWAYS_CHECK_SUBJECT;
        const NO_WILDCARDS = ffi::X509_CHECK_FLAG_NO_WILDCARDS;
        const NO_PARTIAL_WILDCARDS = ffi::X509_CHECK_FLAG_NO_PARTIAL_WILDCARDS;
        const MULTI_LABEL_WILDCARDS = ffi::X509_CHECK_FLAG_MULTI_LABEL_WILDCARDS;
        const SINGLE_LABEL_SUBDOMAINS = ffi::X509_CHECK_FLAG_SINGLE_LABEL_SUBDOMAINS;
        /// Requires OpenSSL 1.1.0 or newer.
        #[cfg(any(ossl110))]
        const NEVER_CHECK_SUBJECT = ffi::X509_CHECK_FLAG_NEVER_CHECK_SUBJECT;

        #[deprecated(since = "0.10.6", note = "renamed to NO_WILDCARDS")]
        const FLAG_NO_WILDCARDS = ffi::X509_CHECK_FLAG_NO_WILDCARDS;
    }
}

bitflags! {
    /// Flags used to verify an `X509` certificate chain.
    pub struct X509VerifyFlags: c_ulong {
        const X509_V_FLAG_CB_ISSUER_CHECK = ffi::X509_V_FLAG_CB_ISSUER_CHECK;
        const X509_V_FLAG_USE_CHECK_TIME = ffi::X509_V_FLAG_USE_CHECK_TIME;
        const X509_V_FLAG_CRL_CHECK = ffi::X509_V_FLAG_CRL_CHECK;
        const X509_V_FLAG_CRL_CHECK_ALL = ffi::X509_V_FLAG_CRL_CHECK_ALL;
        const X509_V_FLAG_IGNORE_CRITICAL = ffi::X509_V_FLAG_X509_STRICT;
        const X509_V_FLAG_X509_STRICT = ffi::X509_V_FLAG_IGNORE_CRITICAL;
        const X509_V_FLAG_ALLOW_PROXY_CERTS = ffi::X509_V_FLAG_ALLOW_PROXY_CERTS;
        const X509_V_FLAG_POLICY_CHECK = ffi::X509_V_FLAG_POLICY_CHECK;
        const X509_V_FLAG_EXPLICIT_POLICY = ffi::X509_V_FLAG_EXPLICIT_POLICY;
        const X509_V_FLAG_INHIBIT_ANY = ffi::X509_V_FLAG_INHIBIT_ANY;
        const X509_V_FLAG_INHIBIT_MAP = ffi::X509_V_FLAG_INHIBIT_MAP;
        const X509_V_FLAG_NOTIFY_POLICY = ffi::X509_V_FLAG_NOTIFY_POLICY;
        const X509_V_FLAG_EXTENDED_CRL_SUPPORT = ffi::X509_V_FLAG_EXTENDED_CRL_SUPPORT;
        const X509_V_FLAG_USE_DELTAS = ffi::X509_V_FLAG_USE_DELTAS;
        const X509_V_FLAG_CHECK_SS_SIGNATURE = ffi::X509_V_FLAG_CHECK_SS_SIGNATURE;
        const X509_V_FLAG_TRUSTED_FIRST = ffi::X509_V_FLAG_TRUSTED_FIRST;
        const X509_V_FLAG_SUITEB_128_LOS_ONLY = ffi::X509_V_FLAG_SUITEB_128_LOS_ONLY;
        const X509_V_FLAG_SUITEB_192_LOS = ffi::X509_V_FLAG_SUITEB_128_LOS;
        const X509_V_FLAG_SUITEB_128_LOS = ffi::X509_V_FLAG_SUITEB_192_LOS;
        const X509_V_FLAG_PARTIAL_CHAIN = ffi::X509_V_FLAG_PARTIAL_CHAIN;
        const X509_V_FLAG_NO_ALT_CHAINS = ffi::X509_V_FLAG_NO_ALT_CHAINS;
        const X509_V_FLAG_NO_CHECK_TIME = ffi::X509_V_FLAG_NO_CHECK_TIME;
    }
}

foreign_type_and_impl_send_sync! {
    type CType = ffi::X509_VERIFY_PARAM;
    fn drop = ffi::X509_VERIFY_PARAM_free;

    /// Adjust parameters associated with certificate verification.
    pub struct X509VerifyParam;
    /// Reference to `X509VerifyParam`.
    pub struct X509VerifyParamRef;
}

impl X509VerifyParamRef {
    /// Set the host flags.
    ///
    /// This corresponds to [`X509_VERIFY_PARAM_set_hostflags`].
    ///
    /// [`X509_VERIFY_PARAM_set_hostflags`]: https://www.openssl.org/docs/man1.1.0/crypto/X509_VERIFY_PARAM_set_hostflags.html
    pub fn set_hostflags(&mut self, hostflags: X509CheckFlags) {
        unsafe {
            ffi::X509_VERIFY_PARAM_set_hostflags(self.as_ptr(), hostflags.bits);
        }
    }

    /// Set verification flags.
    ///
    /// This corresponds to [`X509_VERIFY_PARAM_set_flags`].
    ///
    /// [`X509_VERIFY_PARAM_set_flags`]: https://www.openssl.org/docs/man1.0.2/crypto/X509_VERIFY_PARAM_set_flags.html
    pub fn set_flags(&mut self, flags: X509VerifyFlags) -> Result<(), ErrorStack> {
        unsafe { cvt(ffi::X509_VERIFY_PARAM_set_flags(self.as_ptr(), flags.bits)).map(|_| ()) }
    }

    /// Clear verification flags.
    ///
    /// This corresponds to [`X509_VERIFY_PARAM_clear_flags`].
    ///
    /// [`X509_VERIFY_PARAM_clear_flags`]: https://www.openssl.org/docs/man1.0.2/crypto/X509_VERIFY_PARAM_clear_flags.html
    pub fn clear_flags(&mut self, flags: X509VerifyFlags) -> Result<(), ErrorStack> {
        unsafe {
            cvt(ffi::X509_VERIFY_PARAM_clear_flags(
                self.as_ptr(),
                flags.bits,
            ))
            .map(|_| ())
        }
    }

    /// Gets verification flags.
    ///
    /// This corresponds to [`X509_VERIFY_PARAM_get_flags`].
    ///
    /// [`X509_VERIFY_PARAM_get_flags`]: https://www.openssl.org/docs/man1.0.2/crypto/X509_VERIFY_PARAM_get_flags.html
    pub fn get_flags(&mut self) -> X509VerifyFlags {
        let bits = unsafe { ffi::X509_VERIFY_PARAM_get_flags(self.as_ptr()) };
        X509VerifyFlags { bits }
    }

    /// Set the expected DNS hostname.
    ///
    /// This corresponds to [`X509_VERIFY_PARAM_set1_host`].
    ///
    /// [`X509_VERIFY_PARAM_set1_host`]: https://www.openssl.org/docs/man1.1.0/crypto/X509_VERIFY_PARAM_set1_host.html
    pub fn set_host(&mut self, host: &str) -> Result<(), ErrorStack> {
        unsafe {
            cvt(ffi::X509_VERIFY_PARAM_set1_host(
                self.as_ptr(),
                host.as_ptr() as *const _,
                host.len(),
            ))
            .map(|_| ())
        }
    }

    /// Set the expected IPv4 or IPv6 address.
    ///
    /// This corresponds to [`X509_VERIFY_PARAM_set1_ip`].
    ///
    /// [`X509_VERIFY_PARAM_set1_ip`]: https://www.openssl.org/docs/man1.1.0/crypto/X509_VERIFY_PARAM_set1_ip.html
    pub fn set_ip(&mut self, ip: IpAddr) -> Result<(), ErrorStack> {
        unsafe {
            let mut buf = [0; 16];
            let len = match ip {
                IpAddr::V4(addr) => {
                    buf[..4].copy_from_slice(&addr.octets());
                    4
                }
                IpAddr::V6(addr) => {
                    buf.copy_from_slice(&addr.octets());
                    16
                }
            };
            cvt(ffi::X509_VERIFY_PARAM_set1_ip(
                self.as_ptr(),
                buf.as_ptr() as *const _,
                len,
            ))
            .map(|_| ())
        }
    }
}
