// Copyright 2013-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib_sys;
use libc::size_t;
use std::vec::Vec;
use translate::*;
use Checksum;

impl Checksum {
    pub fn get_digest(self) -> Vec<u8> {
        unsafe {
            //Don't forget update when `ChecksumType` contains type bigger that Sha512.
            let mut digest_len: size_t = 512 / 8;
            let mut vec = Vec::with_capacity(digest_len as usize);

            glib_sys::g_checksum_get_digest(
                mut_override(self.to_glib_none().0),
                vec.as_mut_ptr(),
                &mut digest_len,
            );

            vec.set_len(digest_len);
            vec
        }
    }

    pub fn get_string(self) -> Option<String> {
        unsafe {
            from_glib_none(glib_sys::g_checksum_get_string(mut_override(
                self.to_glib_none().0,
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use {Checksum, ChecksumType};

    const CS_TYPE: ChecksumType = ChecksumType::Md5;
    const CS_VALUE: &str = "fc3ff98e8c6a0d3087d515c0473f8677";
    const CS_SLICE: &[u8] = &[
        0xfc, 0x3f, 0xf9, 0x8e, 0x8c, 0x6a, 0x0d, 0x30, 0x87, 0xd5, 0x15, 0xc0, 0x47, 0x3f, 0x86,
        0x77,
    ];

    #[test]
    fn update() {
        let mut cs = Checksum::new(CS_TYPE);
        cs.update("hello world!".as_bytes());
        assert_eq!(cs.get_string().unwrap(), CS_VALUE);
    }

    #[test]
    fn update_multi_call() {
        let mut cs = Checksum::new(CS_TYPE);
        cs.update("hello ".as_bytes());
        cs.update("world!".as_bytes());
        assert_eq!(cs.get_string().unwrap(), CS_VALUE);
    }

    #[test]
    fn get_digest() {
        let mut cs = Checksum::new(CS_TYPE);
        cs.update("hello world!".as_bytes());
        let vec = cs.get_digest();
        assert_eq!(vec, CS_SLICE);
    }
}
