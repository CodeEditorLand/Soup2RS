// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

use std::fmt;

use crate::Auth;

glib::wrapper! {
	#[doc(alias = "SoupAuthDigest")]
	pub struct AuthDigest(Object<ffi::SoupAuthDigest>) @extends Auth;

	match fn {
		type_ => || ffi::soup_auth_digest_get_type(),
	}
}

impl AuthDigest {}

impl fmt::Display for AuthDigest {
	fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result { f.write_str("AuthDigest") }
}
