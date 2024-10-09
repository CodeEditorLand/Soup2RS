// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

#[cfg(any(feature = "v2_26", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_26")))]
use std::mem;

use glib::translate::*;

use crate::{Encoding, Expectation, MessageHeadersType};

glib::wrapper! {
	#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
	pub struct MessageHeaders(Boxed<ffi::SoupMessageHeaders>);

	match fn {
		copy => |ptr| glib::gobject_ffi::g_boxed_copy(ffi::soup_message_headers_get_type(), ptr as *mut _) as *mut ffi::SoupMessageHeaders,
		free => |ptr| glib::gobject_ffi::g_boxed_free(ffi::soup_message_headers_get_type(), ptr as *mut _),
		type_ => || ffi::soup_message_headers_get_type(),
	}
}

impl MessageHeaders {
	#[doc(alias = "soup_message_headers_new")]
	pub fn new(type_:MessageHeadersType) -> MessageHeaders {
		crate::assert_initialized_main_thread!();
		unsafe { from_glib_full(ffi::soup_message_headers_new(type_.into_glib())) }
	}

	#[doc(alias = "soup_message_headers_append")]
	pub fn append(&mut self, name:&str, value:&str) {
		unsafe {
			ffi::soup_message_headers_append(
				self.to_glib_none_mut().0,
				name.to_glib_none().0,
				value.to_glib_none().0,
			);
		}
	}

	#[cfg(any(feature = "v2_36", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_36")))]
	#[doc(alias = "soup_message_headers_clean_connection_headers")]
	pub fn clean_connection_headers(&mut self) {
		unsafe {
			ffi::soup_message_headers_clean_connection_headers(self.to_glib_none_mut().0);
		}
	}

	#[doc(alias = "soup_message_headers_clear")]
	pub fn clear(&mut self) {
		unsafe {
			ffi::soup_message_headers_clear(self.to_glib_none_mut().0);
		}
	}

	#[doc(alias = "soup_message_headers_foreach")]
	pub fn foreach<P:FnMut(&str, &str)>(&mut self, func:P) {
		let func_data:P = func;
		unsafe extern fn func_func<P:FnMut(&str, &str)>(
			name:*const libc::c_char,
			value:*const libc::c_char,
			user_data:glib::ffi::gpointer,
		) {
			let name:Borrowed<glib::GString> = from_glib_borrow(name);
			let value:Borrowed<glib::GString> = from_glib_borrow(value);
			let callback:*mut P = user_data as *const _ as usize as *mut P;
			(*callback)(name.as_str(), value.as_str());
		}
		let func = Some(func_func::<P> as _);
		let super_callback0:&P = &func_data;
		unsafe {
			ffi::soup_message_headers_foreach(
				self.to_glib_none_mut().0,
				func,
				super_callback0 as *const _ as usize as *mut _,
			);
		}
	}

	//#[cfg(any(feature = "v2_26", feature = "dox"))]
	//#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_26")))]
	//#[doc(alias = "soup_message_headers_free_ranges")]
	// pub fn free_ranges(&mut self, ranges: /*Ignored*/&mut Range) {
	//    unsafe { TODO: call ffi:soup_message_headers_free_ranges() }
	//}

	#[doc(alias = "soup_message_headers_get")]
	pub fn get(&mut self, name:&str) -> Option<glib::GString> {
		unsafe {
			from_glib_none(ffi::soup_message_headers_get(
				self.to_glib_none_mut().0,
				name.to_glib_none().0,
			))
		}
	}

	//#[cfg(any(feature = "v2_26", feature = "dox"))]
	//#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_26")))]
	//#[doc(alias = "soup_message_headers_get_content_disposition")]
	//#[doc(alias = "get_content_disposition")]
	// pub fn content_disposition(&mut self, params: /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 }) -> Option<glib::GString> {
	//    unsafe { TODO: call ffi:soup_message_headers_get_content_disposition() }
	//}

	#[doc(alias = "soup_message_headers_get_content_length")]
	#[doc(alias = "get_content_length")]
	pub fn content_length(&mut self) -> i64 {
		unsafe { ffi::soup_message_headers_get_content_length(self.to_glib_none_mut().0) }
	}

	#[cfg(any(feature = "v2_26", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_26")))]
	#[doc(alias = "soup_message_headers_get_content_range")]
	#[doc(alias = "get_content_range")]
	pub fn content_range(&mut self) -> Option<(i64, i64, i64)> {
		unsafe {
			let mut start = mem::MaybeUninit::uninit();
			let mut end = mem::MaybeUninit::uninit();
			let mut total_length = mem::MaybeUninit::uninit();
			let ret = from_glib(ffi::soup_message_headers_get_content_range(
				self.to_glib_none_mut().0,
				start.as_mut_ptr(),
				end.as_mut_ptr(),
				total_length.as_mut_ptr(),
			));
			let start = start.assume_init();
			let end = end.assume_init();
			let total_length = total_length.assume_init();
			if ret { Some((start, end, total_length)) } else { None }
		}
	}

	//#[cfg(any(feature = "v2_26", feature = "dox"))]
	//#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_26")))]
	//#[doc(alias = "soup_message_headers_get_content_type")]
	//#[doc(alias = "get_content_type")]
	// pub fn content_type(&mut self, params: /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 }) -> Option<glib::GString> {
	//    unsafe { TODO: call ffi:soup_message_headers_get_content_type() }
	//}

	#[doc(alias = "soup_message_headers_get_encoding")]
	#[doc(alias = "get_encoding")]
	pub fn encoding(&mut self) -> Encoding {
		unsafe { from_glib(ffi::soup_message_headers_get_encoding(self.to_glib_none_mut().0)) }
	}

	#[doc(alias = "soup_message_headers_get_expectations")]
	#[doc(alias = "get_expectations")]
	pub fn expectations(&mut self) -> Expectation {
		unsafe { from_glib(ffi::soup_message_headers_get_expectations(self.to_glib_none_mut().0)) }
	}

	#[cfg(any(feature = "v2_50", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_50")))]
	#[doc(alias = "soup_message_headers_get_headers_type")]
	#[doc(alias = "get_headers_type")]
	pub fn headers_type(&mut self) -> MessageHeadersType {
		unsafe { from_glib(ffi::soup_message_headers_get_headers_type(self.to_glib_none_mut().0)) }
	}

	#[cfg(any(feature = "v2_28", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
	#[doc(alias = "soup_message_headers_get_list")]
	#[doc(alias = "get_list")]
	pub fn list(&mut self, name:&str) -> Option<glib::GString> {
		unsafe {
			from_glib_none(ffi::soup_message_headers_get_list(
				self.to_glib_none_mut().0,
				name.to_glib_none().0,
			))
		}
	}

	#[cfg(any(feature = "v2_28", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
	#[doc(alias = "soup_message_headers_get_one")]
	#[doc(alias = "get_one")]
	pub fn one(&mut self, name:&str) -> Option<glib::GString> {
		unsafe {
			from_glib_none(ffi::soup_message_headers_get_one(
				self.to_glib_none_mut().0,
				name.to_glib_none().0,
			))
		}
	}

	//#[cfg(any(feature = "v2_26", feature = "dox"))]
	//#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_26")))]
	//#[doc(alias = "soup_message_headers_get_ranges")]
	//#[doc(alias = "get_ranges")]
	// pub fn ranges(&mut self, total_length: i64, ranges: /*Ignored*/Vec<Range>) ->
	// Option<i32> {    unsafe { TODO: call ffi:soup_message_headers_get_ranges() }
	//}

	#[cfg(any(feature = "v2_50", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_50")))]
	#[doc(alias = "soup_message_headers_header_contains")]
	pub fn header_contains(&mut self, name:&str, token:&str) -> bool {
		unsafe {
			from_glib(ffi::soup_message_headers_header_contains(
				self.to_glib_none_mut().0,
				name.to_glib_none().0,
				token.to_glib_none().0,
			))
		}
	}

	#[cfg(any(feature = "v2_50", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_50")))]
	#[doc(alias = "soup_message_headers_header_equals")]
	pub fn header_equals(&mut self, name:&str, value:&str) -> bool {
		unsafe {
			from_glib(ffi::soup_message_headers_header_equals(
				self.to_glib_none_mut().0,
				name.to_glib_none().0,
				value.to_glib_none().0,
			))
		}
	}

	#[doc(alias = "soup_message_headers_remove")]
	pub fn remove(&mut self, name:&str) {
		unsafe {
			ffi::soup_message_headers_remove(self.to_glib_none_mut().0, name.to_glib_none().0);
		}
	}

	#[doc(alias = "soup_message_headers_replace")]
	pub fn replace(&mut self, name:&str, value:&str) {
		unsafe {
			ffi::soup_message_headers_replace(
				self.to_glib_none_mut().0,
				name.to_glib_none().0,
				value.to_glib_none().0,
			);
		}
	}

	//#[cfg(any(feature = "v2_26", feature = "dox"))]
	//#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_26")))]
	//#[doc(alias = "soup_message_headers_set_content_disposition")]
	// pub fn set_content_disposition(&mut self, disposition: &str, params:
	// /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28
	// }/TypeId { ns_id: 0, id: 28 }) {    unsafe { TODO: call
	// ffi:soup_message_headers_set_content_disposition() }
	//}

	#[doc(alias = "soup_message_headers_set_content_length")]
	pub fn set_content_length(&mut self, content_length:i64) {
		unsafe {
			ffi::soup_message_headers_set_content_length(self.to_glib_none_mut().0, content_length);
		}
	}

	#[cfg(any(feature = "v2_26", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_26")))]
	#[doc(alias = "soup_message_headers_set_content_range")]
	pub fn set_content_range(&mut self, start:i64, end:i64, total_length:i64) {
		unsafe {
			ffi::soup_message_headers_set_content_range(
				self.to_glib_none_mut().0,
				start,
				end,
				total_length,
			);
		}
	}

	//#[cfg(any(feature = "v2_26", feature = "dox"))]
	//#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_26")))]
	//#[doc(alias = "soup_message_headers_set_content_type")]
	// pub fn set_content_type(&mut self, content_type: &str, params: /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 }) {
	//    unsafe { TODO: call ffi:soup_message_headers_set_content_type() }
	//}

	#[doc(alias = "soup_message_headers_set_encoding")]
	pub fn set_encoding(&mut self, encoding:Encoding) {
		unsafe {
			ffi::soup_message_headers_set_encoding(self.to_glib_none_mut().0, encoding.into_glib());
		}
	}

	#[doc(alias = "soup_message_headers_set_expectations")]
	pub fn set_expectations(&mut self, expectations:Expectation) {
		unsafe {
			ffi::soup_message_headers_set_expectations(
				self.to_glib_none_mut().0,
				expectations.into_glib(),
			);
		}
	}

	#[cfg(any(feature = "v2_26", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_26")))]
	#[doc(alias = "soup_message_headers_set_range")]
	pub fn set_range(&mut self, start:i64, end:i64) {
		unsafe {
			ffi::soup_message_headers_set_range(self.to_glib_none_mut().0, start, end);
		}
	}

	//#[cfg(any(feature = "v2_26", feature = "dox"))]
	//#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_26")))]
	//#[doc(alias = "soup_message_headers_set_ranges")]
	// pub fn set_ranges(&mut self, ranges: /*Ignored*/&mut Range, length: i32) {
	//    unsafe { TODO: call ffi:soup_message_headers_set_ranges() }
	//}
}
