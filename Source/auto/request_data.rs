// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

use std::fmt;

#[cfg(any(feature = "v2_42", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_42")))]
use crate::Request;

#[cfg(any(feature = "v2_42", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_42")))]
glib::wrapper! {
	#[doc(alias = "SoupRequestData")]
	pub struct RequestData(Object<ffi::SoupRequestData, ffi::SoupRequestDataClass>) @extends Request;

	match fn {
		type_ => || ffi::soup_request_data_get_type(),
	}
}

#[cfg(not(any(feature = "v2_42", feature = "dox")))]
glib::wrapper! {
	#[doc(alias = "SoupRequestData")]
	pub struct RequestData(Object<ffi::SoupRequestData, ffi::SoupRequestDataClass>);

	match fn {
		type_ => || ffi::soup_request_data_get_type(),
	}
}

impl RequestData {}

pub const NONE_REQUEST_DATA:Option<&RequestData> = None;

impl fmt::Display for RequestData {
	fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result { f.write_str("RequestData") }
}
