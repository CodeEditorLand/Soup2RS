// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

use std::fmt;

use glib::{object::IsA, translate::*};

use crate::{Message, Request};

glib::wrapper! {
	#[doc(alias = "SoupRequestHTTP")]
	pub struct RequestHTTP(Object<ffi::SoupRequestHTTP, ffi::SoupRequestHTTPClass>) @extends Request;

	match fn {
		type_ => || ffi::soup_request_http_get_type(),
	}
}

pub const NONE_REQUEST_HTTP:Option<&RequestHTTP> = None;

pub trait RequestHTTPExt: 'static {
	#[doc(alias = "soup_request_http_get_message")]
	#[doc(alias = "get_message")]
	fn message(&self) -> Option<Message>;
}

impl<O:IsA<RequestHTTP>> RequestHTTPExt for O {
	fn message(&self) -> Option<Message> {
		unsafe {
			from_glib_full(ffi::soup_request_http_get_message(self.as_ref().to_glib_none().0))
		}
	}
}

impl fmt::Display for RequestHTTP {
	fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result { f.write_str("RequestHTTP") }
}
