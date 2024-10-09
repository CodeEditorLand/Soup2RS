// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

#[cfg(any(feature = "v2_48", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_48")))]
use std::ptr;
use std::{boxed::Box as Box_, fmt, mem::transmute};

use glib::{
	object::{Cast, IsA},
	signal::{connect_raw, SignalHandlerId},
	translate::*,
	StaticType,
	ToValue,
};

#[cfg(any(feature = "v2_48", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_48")))]
use crate::ServerListenOptions;
#[cfg(any(feature = "v2_68", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_68")))]
use crate::WebsocketConnection;
#[cfg(any(feature = "v2_48", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_48")))]
use crate::URI;
use crate::{Address, AuthDomain, ClientContext, Message, Socket};

glib::wrapper! {
	#[doc(alias = "SoupServer")]
	pub struct Server(Object<ffi::SoupServer, ffi::SoupServerClass>);

	match fn {
		type_ => || ffi::soup_server_get_type(),
	}
}

impl Server {
	//#[doc(alias = "soup_server_new")]
	// pub fn new(optname1: &str, : /*Unknown
	// conversion*//*Unimplemented*/Fundamental: VarArgs) -> Option<Server> {
	//    unsafe { TODO: call ffi:soup_server_new() }
	//}
}

pub const NONE_SERVER:Option<&Server> = None;

pub trait ServerExt: 'static {
	#[cfg(any(feature = "v2_50", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_50")))]
	#[doc(alias = "soup_server_accept_iostream")]
	fn accept_iostream(
		&self,
		stream:&impl IsA<gio::IOStream>,
		local_addr:Option<&impl IsA<gio::SocketAddress>>,
		remote_addr:Option<&impl IsA<gio::SocketAddress>>,
	) -> Result<(), glib::Error>;

	#[doc(alias = "soup_server_add_auth_domain")]
	fn add_auth_domain(&self, auth_domain:&impl IsA<AuthDomain>);

	//#[cfg(any(feature = "v2_50", feature = "dox"))]
	//#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_50")))]
	//#[doc(alias = "soup_server_add_early_handler")]
	// fn add_early_handler(&self, path: Option<&str>, callback:
	// /*Unimplemented*/Fn(&Server, &Message, &str, /*Unimplemented*/HashTable
	// TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 }, &ClientContext),
	// user_data: /*Unimplemented*/Option<Fundamental: Pointer>);

	//#[doc(alias = "soup_server_add_handler")]
	// fn add_handler(&self, path: Option<&str>, callback:
	// /*Unimplemented*/Fn(&Server, &Message, &str, /*Unimplemented*/HashTable
	// TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 }, &ClientContext),
	// user_data: /*Unimplemented*/Option<Fundamental: Pointer>);

	#[cfg(any(feature = "v2_68", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_68")))]
	#[doc(alias = "soup_server_add_websocket_extension")]
	fn add_websocket_extension(&self, extension_type:glib::types::Type);

	#[cfg(any(feature = "v2_68", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_68")))]
	#[doc(alias = "soup_server_add_websocket_handler")]
	fn add_websocket_handler<P:Fn(&Server, &WebsocketConnection, &str, &ClientContext) + 'static>(
		&self,
		path:Option<&str>,
		origin:Option<&str>,
		protocols:&[&str],
		callback:P,
	);

	#[doc(alias = "soup_server_disconnect")]
	fn disconnect(&self);

	#[doc(alias = "soup_server_get_async_context")]
	#[doc(alias = "get_async_context")]
	fn async_context(&self) -> Option<glib::MainContext>;

	#[doc(alias = "soup_server_get_listener")]
	#[doc(alias = "get_listener")]
	fn listener(&self) -> Option<Socket>;

	#[doc(alias = "soup_server_get_listeners")]
	#[doc(alias = "get_listeners")]
	fn listeners(&self) -> Vec<gio::Socket>;

	#[doc(alias = "soup_server_get_port")]
	#[doc(alias = "get_port")]
	fn port(&self) -> u32;

	#[cfg(any(feature = "v2_48", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_48")))]
	#[doc(alias = "soup_server_get_uris")]
	#[doc(alias = "get_uris")]
	fn uris(&self) -> Vec<URI>;

	#[doc(alias = "soup_server_is_https")]
	fn is_https(&self) -> bool;

	#[cfg(any(feature = "v2_48", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_48")))]
	#[doc(alias = "soup_server_listen")]
	fn listen(
		&self,
		address:&impl IsA<gio::SocketAddress>,
		options:ServerListenOptions,
	) -> Result<(), glib::Error>;

	#[cfg(any(feature = "v2_48", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_48")))]
	#[doc(alias = "soup_server_listen_all")]
	fn listen_all(&self, port:u32, options:ServerListenOptions) -> Result<(), glib::Error>;

	#[cfg(any(feature = "v2_48", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_48")))]
	#[doc(alias = "soup_server_listen_fd")]
	fn listen_fd(&self, fd:i32, options:ServerListenOptions) -> Result<(), glib::Error>;

	#[cfg(any(feature = "v2_48", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_48")))]
	#[doc(alias = "soup_server_listen_local")]
	fn listen_local(&self, port:u32, options:ServerListenOptions) -> Result<(), glib::Error>;

	#[cfg(any(feature = "v2_48", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_48")))]
	#[doc(alias = "soup_server_listen_socket")]
	fn listen_socket(
		&self,
		socket:&impl IsA<gio::Socket>,
		options:ServerListenOptions,
	) -> Result<(), glib::Error>;

	#[doc(alias = "soup_server_pause_message")]
	fn pause_message(&self, msg:&impl IsA<Message>);

	#[doc(alias = "soup_server_quit")]
	fn quit(&self);

	#[doc(alias = "soup_server_remove_auth_domain")]
	fn remove_auth_domain(&self, auth_domain:&impl IsA<AuthDomain>);

	#[doc(alias = "soup_server_remove_handler")]
	fn remove_handler(&self, path:&str);

	#[cfg(any(feature = "v2_68", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_68")))]
	#[doc(alias = "soup_server_remove_websocket_extension")]
	fn remove_websocket_extension(&self, extension_type:glib::types::Type);

	#[doc(alias = "soup_server_run")]
	fn run(&self);

	#[doc(alias = "soup_server_run_async")]
	fn run_async(&self);

	#[cfg(any(feature = "v2_48", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_48")))]
	#[doc(alias = "soup_server_set_ssl_cert_file")]
	fn set_ssl_cert_file(&self, ssl_cert_file:&str, ssl_key_file:&str) -> Result<(), glib::Error>;

	#[doc(alias = "soup_server_unpause_message")]
	fn unpause_message(&self, msg:&impl IsA<Message>);

	#[cfg(any(feature = "v2_68", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_68")))]
	#[doc(alias = "add-websocket-extension")]
	fn set_add_websocket_extension(&self, add_websocket_extension:glib::types::Type);

	#[cfg(any(feature = "v2_44", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_44")))]
	#[doc(alias = "http-aliases")]
	fn http_aliases(&self) -> Vec<glib::GString>;

	#[cfg(any(feature = "v2_44", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_44")))]
	#[doc(alias = "http-aliases")]
	fn set_http_aliases(&self, http_aliases:&[&str]);

	#[cfg(any(feature = "v2_44", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_44")))]
	#[doc(alias = "https-aliases")]
	fn https_aliases(&self) -> Vec<glib::GString>;

	#[cfg(any(feature = "v2_44", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_44")))]
	#[doc(alias = "https-aliases")]
	fn set_https_aliases(&self, https_aliases:&[&str]);

	fn interface(&self) -> Option<Address>;

	#[doc(alias = "raw-paths")]
	fn is_raw_paths(&self) -> bool;

	#[cfg(any(feature = "v2_68", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_68")))]
	#[doc(alias = "remove-websocket-extension")]
	fn set_remove_websocket_extension(&self, remove_websocket_extension:glib::types::Type);

	#[doc(alias = "server-header")]
	fn server_header(&self) -> Option<glib::GString>;

	#[doc(alias = "server-header")]
	fn set_server_header(&self, server_header:Option<&str>);

	#[doc(alias = "ssl-cert-file")]
	fn ssl_cert_file(&self) -> Option<glib::GString>;

	#[doc(alias = "ssl-key-file")]
	fn ssl_key_file(&self) -> Option<glib::GString>;

	#[cfg(any(feature = "v2_38", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_38")))]
	#[doc(alias = "tls-certificate")]
	fn tls_certificate(&self) -> Option<gio::TlsCertificate>;

	#[doc(alias = "request-aborted")]
	fn connect_request_aborted<F:Fn(&Self, &Message, &ClientContext) + 'static>(
		&self,
		f:F,
	) -> SignalHandlerId;

	#[doc(alias = "request-finished")]
	fn connect_request_finished<F:Fn(&Self, &Message, &ClientContext) + 'static>(
		&self,
		f:F,
	) -> SignalHandlerId;

	#[doc(alias = "request-read")]
	fn connect_request_read<F:Fn(&Self, &Message, &ClientContext) + 'static>(
		&self,
		f:F,
	) -> SignalHandlerId;

	#[doc(alias = "request-started")]
	fn connect_request_started<F:Fn(&Self, &Message, &ClientContext) + 'static>(
		&self,
		f:F,
	) -> SignalHandlerId;

	#[cfg(any(feature = "v2_68", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_68")))]
	#[doc(alias = "add-websocket-extension")]
	fn connect_add_websocket_extension_notify<F:Fn(&Self) + 'static>(&self, f:F)
	-> SignalHandlerId;

	#[cfg(any(feature = "v2_44", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_44")))]
	#[doc(alias = "http-aliases")]
	fn connect_http_aliases_notify<F:Fn(&Self) + 'static>(&self, f:F) -> SignalHandlerId;

	#[cfg(any(feature = "v2_44", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_44")))]
	#[doc(alias = "https-aliases")]
	fn connect_https_aliases_notify<F:Fn(&Self) + 'static>(&self, f:F) -> SignalHandlerId;

	#[cfg(any(feature = "v2_68", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_68")))]
	#[doc(alias = "remove-websocket-extension")]
	fn connect_remove_websocket_extension_notify<F:Fn(&Self) + 'static>(
		&self,
		f:F,
	) -> SignalHandlerId;

	#[doc(alias = "server-header")]
	fn connect_server_header_notify<F:Fn(&Self) + 'static>(&self, f:F) -> SignalHandlerId;
}

impl<O:IsA<Server>> ServerExt for O {
	#[cfg(any(feature = "v2_50", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_50")))]
	fn accept_iostream(
		&self,
		stream:&impl IsA<gio::IOStream>,
		local_addr:Option<&impl IsA<gio::SocketAddress>>,
		remote_addr:Option<&impl IsA<gio::SocketAddress>>,
	) -> Result<(), glib::Error> {
		unsafe {
			let mut error = ptr::null_mut();
			let _ = ffi::soup_server_accept_iostream(
				self.as_ref().to_glib_none().0,
				stream.as_ref().to_glib_none().0,
				local_addr.map(|p| p.as_ref()).to_glib_none().0,
				remote_addr.map(|p| p.as_ref()).to_glib_none().0,
				&mut error,
			);
			if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
		}
	}

	fn add_auth_domain(&self, auth_domain:&impl IsA<AuthDomain>) {
		unsafe {
			ffi::soup_server_add_auth_domain(
				self.as_ref().to_glib_none().0,
				auth_domain.as_ref().to_glib_none().0,
			);
		}
	}

	//#[cfg(any(feature = "v2_50", feature = "dox"))]
	//#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_50")))]
	// fn add_early_handler(&self, path: Option<&str>, callback:
	// /*Unimplemented*/Fn(&Server, &Message, &str, /*Unimplemented*/HashTable
	// TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 }, &ClientContext),
	// user_data: /*Unimplemented*/Option<Fundamental: Pointer>) {    unsafe {
	// TODO: call ffi:soup_server_add_early_handler() }
	//}

	// fn add_handler(&self, path: Option<&str>, callback:
	// /*Unimplemented*/Fn(&Server, &Message, &str, /*Unimplemented*/HashTable
	// TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 }, &ClientContext),
	// user_data: /*Unimplemented*/Option<Fundamental: Pointer>) {    unsafe {
	// TODO: call ffi:soup_server_add_handler() }
	//}

	#[cfg(any(feature = "v2_68", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_68")))]
	fn add_websocket_extension(&self, extension_type:glib::types::Type) {
		unsafe {
			ffi::soup_server_add_websocket_extension(
				self.as_ref().to_glib_none().0,
				extension_type.into_glib(),
			);
		}
	}

	#[cfg(any(feature = "v2_68", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_68")))]
	fn add_websocket_handler<
		P:Fn(&Server, &WebsocketConnection, &str, &ClientContext) + 'static,
	>(
		&self,
		path:Option<&str>,
		origin:Option<&str>,
		protocols:&[&str],
		callback:P,
	) {
		let callback_data:Box_<P> = Box_::new(callback);
		unsafe extern fn callback_func<
			P:Fn(&Server, &WebsocketConnection, &str, &ClientContext) + 'static,
		>(
			server:*mut ffi::SoupServer,
			connection:*mut ffi::SoupWebsocketConnection,
			path:*const libc::c_char,
			client:*mut ffi::SoupClientContext,
			user_data:glib::ffi::gpointer,
		) {
			let server = from_glib_borrow(server);
			let connection = from_glib_borrow(connection);
			let path:Borrowed<glib::GString> = from_glib_borrow(path);
			let client = from_glib_borrow(client);
			let callback:&P = &*(user_data as *mut _);
			(*callback)(&server, &connection, path.as_str(), &client);
		}
		let callback = Some(callback_func::<P> as _);
		unsafe extern fn destroy_func<
			P:Fn(&Server, &WebsocketConnection, &str, &ClientContext) + 'static,
		>(
			data:glib::ffi::gpointer,
		) {
			let _callback:Box_<P> = Box_::from_raw(data as *mut _);
		}
		let destroy_call6 = Some(destroy_func::<P> as _);
		let super_callback0:Box_<P> = callback_data;
		unsafe {
			ffi::soup_server_add_websocket_handler(
				self.as_ref().to_glib_none().0,
				path.to_glib_none().0,
				origin.to_glib_none().0,
				protocols.to_glib_none().0,
				callback,
				Box_::into_raw(super_callback0) as *mut _,
				destroy_call6,
			);
		}
	}

	fn disconnect(&self) {
		unsafe {
			ffi::soup_server_disconnect(self.as_ref().to_glib_none().0);
		}
	}

	fn async_context(&self) -> Option<glib::MainContext> {
		unsafe {
			from_glib_none(ffi::soup_server_get_async_context(self.as_ref().to_glib_none().0))
		}
	}

	fn listener(&self) -> Option<Socket> {
		unsafe { from_glib_none(ffi::soup_server_get_listener(self.as_ref().to_glib_none().0)) }
	}

	fn listeners(&self) -> Vec<gio::Socket> {
		unsafe {
			FromGlibPtrContainer::from_glib_container(ffi::soup_server_get_listeners(
				self.as_ref().to_glib_none().0,
			))
		}
	}

	fn port(&self) -> u32 { unsafe { ffi::soup_server_get_port(self.as_ref().to_glib_none().0) } }

	#[cfg(any(feature = "v2_48", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_48")))]
	fn uris(&self) -> Vec<URI> {
		unsafe {
			FromGlibPtrContainer::from_glib_full(ffi::soup_server_get_uris(
				self.as_ref().to_glib_none().0,
			))
		}
	}

	fn is_https(&self) -> bool {
		unsafe { from_glib(ffi::soup_server_is_https(self.as_ref().to_glib_none().0)) }
	}

	#[cfg(any(feature = "v2_48", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_48")))]
	fn listen(
		&self,
		address:&impl IsA<gio::SocketAddress>,
		options:ServerListenOptions,
	) -> Result<(), glib::Error> {
		unsafe {
			let mut error = ptr::null_mut();
			let _ = ffi::soup_server_listen(
				self.as_ref().to_glib_none().0,
				address.as_ref().to_glib_none().0,
				options.into_glib(),
				&mut error,
			);
			if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
		}
	}

	#[cfg(any(feature = "v2_48", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_48")))]
	fn listen_all(&self, port:u32, options:ServerListenOptions) -> Result<(), glib::Error> {
		unsafe {
			let mut error = ptr::null_mut();
			let _ = ffi::soup_server_listen_all(
				self.as_ref().to_glib_none().0,
				port,
				options.into_glib(),
				&mut error,
			);
			if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
		}
	}

	#[cfg(any(feature = "v2_48", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_48")))]
	fn listen_fd(&self, fd:i32, options:ServerListenOptions) -> Result<(), glib::Error> {
		unsafe {
			let mut error = ptr::null_mut();
			let _ = ffi::soup_server_listen_fd(
				self.as_ref().to_glib_none().0,
				fd,
				options.into_glib(),
				&mut error,
			);
			if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
		}
	}

	#[cfg(any(feature = "v2_48", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_48")))]
	fn listen_local(&self, port:u32, options:ServerListenOptions) -> Result<(), glib::Error> {
		unsafe {
			let mut error = ptr::null_mut();
			let _ = ffi::soup_server_listen_local(
				self.as_ref().to_glib_none().0,
				port,
				options.into_glib(),
				&mut error,
			);
			if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
		}
	}

	#[cfg(any(feature = "v2_48", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_48")))]
	fn listen_socket(
		&self,
		socket:&impl IsA<gio::Socket>,
		options:ServerListenOptions,
	) -> Result<(), glib::Error> {
		unsafe {
			let mut error = ptr::null_mut();
			let _ = ffi::soup_server_listen_socket(
				self.as_ref().to_glib_none().0,
				socket.as_ref().to_glib_none().0,
				options.into_glib(),
				&mut error,
			);
			if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
		}
	}

	fn pause_message(&self, msg:&impl IsA<Message>) {
		unsafe {
			ffi::soup_server_pause_message(
				self.as_ref().to_glib_none().0,
				msg.as_ref().to_glib_none().0,
			);
		}
	}

	fn quit(&self) {
		unsafe {
			ffi::soup_server_quit(self.as_ref().to_glib_none().0);
		}
	}

	fn remove_auth_domain(&self, auth_domain:&impl IsA<AuthDomain>) {
		unsafe {
			ffi::soup_server_remove_auth_domain(
				self.as_ref().to_glib_none().0,
				auth_domain.as_ref().to_glib_none().0,
			);
		}
	}

	fn remove_handler(&self, path:&str) {
		unsafe {
			ffi::soup_server_remove_handler(self.as_ref().to_glib_none().0, path.to_glib_none().0);
		}
	}

	#[cfg(any(feature = "v2_68", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_68")))]
	fn remove_websocket_extension(&self, extension_type:glib::types::Type) {
		unsafe {
			ffi::soup_server_remove_websocket_extension(
				self.as_ref().to_glib_none().0,
				extension_type.into_glib(),
			);
		}
	}

	fn run(&self) {
		unsafe {
			ffi::soup_server_run(self.as_ref().to_glib_none().0);
		}
	}

	fn run_async(&self) {
		unsafe {
			ffi::soup_server_run_async(self.as_ref().to_glib_none().0);
		}
	}

	#[cfg(any(feature = "v2_48", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_48")))]
	fn set_ssl_cert_file(&self, ssl_cert_file:&str, ssl_key_file:&str) -> Result<(), glib::Error> {
		unsafe {
			let mut error = ptr::null_mut();
			let _ = ffi::soup_server_set_ssl_cert_file(
				self.as_ref().to_glib_none().0,
				ssl_cert_file.to_glib_none().0,
				ssl_key_file.to_glib_none().0,
				&mut error,
			);
			if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
		}
	}

	fn unpause_message(&self, msg:&impl IsA<Message>) {
		unsafe {
			ffi::soup_server_unpause_message(
				self.as_ref().to_glib_none().0,
				msg.as_ref().to_glib_none().0,
			);
		}
	}

	#[cfg(any(feature = "v2_68", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_68")))]
	fn set_add_websocket_extension(&self, add_websocket_extension:glib::types::Type) {
		unsafe {
			glib::gobject_ffi::g_object_set_property(
				self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
				b"add-websocket-extension\0".as_ptr() as *const _,
				add_websocket_extension.to_value().to_glib_none().0,
			);
		}
	}

	#[cfg(any(feature = "v2_44", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_44")))]
	fn http_aliases(&self) -> Vec<glib::GString> {
		unsafe {
			let mut value =
				glib::Value::from_type(<Vec<glib::GString> as StaticType>::static_type());
			glib::gobject_ffi::g_object_get_property(
				self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
				b"http-aliases\0".as_ptr() as *const _,
				value.to_glib_none_mut().0,
			);
			value.get().expect("Return Value for property `http-aliases` getter")
		}
	}

	#[cfg(any(feature = "v2_44", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_44")))]
	fn set_http_aliases(&self, http_aliases:&[&str]) {
		unsafe {
			glib::gobject_ffi::g_object_set_property(
				self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
				b"http-aliases\0".as_ptr() as *const _,
				http_aliases.to_value().to_glib_none().0,
			);
		}
	}

	#[cfg(any(feature = "v2_44", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_44")))]
	fn https_aliases(&self) -> Vec<glib::GString> {
		unsafe {
			let mut value =
				glib::Value::from_type(<Vec<glib::GString> as StaticType>::static_type());
			glib::gobject_ffi::g_object_get_property(
				self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
				b"https-aliases\0".as_ptr() as *const _,
				value.to_glib_none_mut().0,
			);
			value.get().expect("Return Value for property `https-aliases` getter")
		}
	}

	#[cfg(any(feature = "v2_44", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_44")))]
	fn set_https_aliases(&self, https_aliases:&[&str]) {
		unsafe {
			glib::gobject_ffi::g_object_set_property(
				self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
				b"https-aliases\0".as_ptr() as *const _,
				https_aliases.to_value().to_glib_none().0,
			);
		}
	}

	fn interface(&self) -> Option<Address> {
		unsafe {
			let mut value = glib::Value::from_type(<Address as StaticType>::static_type());
			glib::gobject_ffi::g_object_get_property(
				self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
				b"interface\0".as_ptr() as *const _,
				value.to_glib_none_mut().0,
			);
			value.get().expect("Return Value for property `interface` getter")
		}
	}

	fn is_raw_paths(&self) -> bool {
		unsafe {
			let mut value = glib::Value::from_type(<bool as StaticType>::static_type());
			glib::gobject_ffi::g_object_get_property(
				self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
				b"raw-paths\0".as_ptr() as *const _,
				value.to_glib_none_mut().0,
			);
			value.get().expect("Return Value for property `raw-paths` getter")
		}
	}

	#[cfg(any(feature = "v2_68", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_68")))]
	fn set_remove_websocket_extension(&self, remove_websocket_extension:glib::types::Type) {
		unsafe {
			glib::gobject_ffi::g_object_set_property(
				self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
				b"remove-websocket-extension\0".as_ptr() as *const _,
				remove_websocket_extension.to_value().to_glib_none().0,
			);
		}
	}

	fn server_header(&self) -> Option<glib::GString> {
		unsafe {
			let mut value = glib::Value::from_type(<glib::GString as StaticType>::static_type());
			glib::gobject_ffi::g_object_get_property(
				self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
				b"server-header\0".as_ptr() as *const _,
				value.to_glib_none_mut().0,
			);
			value.get().expect("Return Value for property `server-header` getter")
		}
	}

	fn set_server_header(&self, server_header:Option<&str>) {
		unsafe {
			glib::gobject_ffi::g_object_set_property(
				self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
				b"server-header\0".as_ptr() as *const _,
				server_header.to_value().to_glib_none().0,
			);
		}
	}

	fn ssl_cert_file(&self) -> Option<glib::GString> {
		unsafe {
			let mut value = glib::Value::from_type(<glib::GString as StaticType>::static_type());
			glib::gobject_ffi::g_object_get_property(
				self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
				b"ssl-cert-file\0".as_ptr() as *const _,
				value.to_glib_none_mut().0,
			);
			value.get().expect("Return Value for property `ssl-cert-file` getter")
		}
	}

	fn ssl_key_file(&self) -> Option<glib::GString> {
		unsafe {
			let mut value = glib::Value::from_type(<glib::GString as StaticType>::static_type());
			glib::gobject_ffi::g_object_get_property(
				self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
				b"ssl-key-file\0".as_ptr() as *const _,
				value.to_glib_none_mut().0,
			);
			value.get().expect("Return Value for property `ssl-key-file` getter")
		}
	}

	#[cfg(any(feature = "v2_38", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_38")))]
	fn tls_certificate(&self) -> Option<gio::TlsCertificate> {
		unsafe {
			let mut value =
				glib::Value::from_type(<gio::TlsCertificate as StaticType>::static_type());
			glib::gobject_ffi::g_object_get_property(
				self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
				b"tls-certificate\0".as_ptr() as *const _,
				value.to_glib_none_mut().0,
			);
			value.get().expect("Return Value for property `tls-certificate` getter")
		}
	}

	fn connect_request_aborted<F:Fn(&Self, &Message, &ClientContext) + 'static>(
		&self,
		f:F,
	) -> SignalHandlerId {
		unsafe extern fn request_aborted_trampoline<
			P:IsA<Server>,
			F:Fn(&P, &Message, &ClientContext) + 'static,
		>(
			this:*mut ffi::SoupServer,
			message:*mut ffi::SoupMessage,
			client:*mut ffi::SoupClientContext,
			f:glib::ffi::gpointer,
		) {
			let f:&F = &*(f as *const F);
			f(
				Server::from_glib_borrow(this).unsafe_cast_ref(),
				&from_glib_borrow(message),
				&from_glib_borrow(client),
			)
		}
		unsafe {
			let f:Box_<F> = Box_::new(f);
			connect_raw(
				self.as_ptr() as *mut _,
				b"request-aborted\0".as_ptr() as *const _,
				Some(transmute::<_, unsafe extern fn()>(
					request_aborted_trampoline::<Self, F> as *const (),
				)),
				Box_::into_raw(f),
			)
		}
	}

	fn connect_request_finished<F:Fn(&Self, &Message, &ClientContext) + 'static>(
		&self,
		f:F,
	) -> SignalHandlerId {
		unsafe extern fn request_finished_trampoline<
			P:IsA<Server>,
			F:Fn(&P, &Message, &ClientContext) + 'static,
		>(
			this:*mut ffi::SoupServer,
			message:*mut ffi::SoupMessage,
			client:*mut ffi::SoupClientContext,
			f:glib::ffi::gpointer,
		) {
			let f:&F = &*(f as *const F);
			f(
				Server::from_glib_borrow(this).unsafe_cast_ref(),
				&from_glib_borrow(message),
				&from_glib_borrow(client),
			)
		}
		unsafe {
			let f:Box_<F> = Box_::new(f);
			connect_raw(
				self.as_ptr() as *mut _,
				b"request-finished\0".as_ptr() as *const _,
				Some(transmute::<_, unsafe extern fn()>(
					request_finished_trampoline::<Self, F> as *const (),
				)),
				Box_::into_raw(f),
			)
		}
	}

	fn connect_request_read<F:Fn(&Self, &Message, &ClientContext) + 'static>(
		&self,
		f:F,
	) -> SignalHandlerId {
		unsafe extern fn request_read_trampoline<
			P:IsA<Server>,
			F:Fn(&P, &Message, &ClientContext) + 'static,
		>(
			this:*mut ffi::SoupServer,
			message:*mut ffi::SoupMessage,
			client:*mut ffi::SoupClientContext,
			f:glib::ffi::gpointer,
		) {
			let f:&F = &*(f as *const F);
			f(
				Server::from_glib_borrow(this).unsafe_cast_ref(),
				&from_glib_borrow(message),
				&from_glib_borrow(client),
			)
		}
		unsafe {
			let f:Box_<F> = Box_::new(f);
			connect_raw(
				self.as_ptr() as *mut _,
				b"request-read\0".as_ptr() as *const _,
				Some(transmute::<_, unsafe extern fn()>(
					request_read_trampoline::<Self, F> as *const (),
				)),
				Box_::into_raw(f),
			)
		}
	}

	fn connect_request_started<F:Fn(&Self, &Message, &ClientContext) + 'static>(
		&self,
		f:F,
	) -> SignalHandlerId {
		unsafe extern fn request_started_trampoline<
			P:IsA<Server>,
			F:Fn(&P, &Message, &ClientContext) + 'static,
		>(
			this:*mut ffi::SoupServer,
			message:*mut ffi::SoupMessage,
			client:*mut ffi::SoupClientContext,
			f:glib::ffi::gpointer,
		) {
			let f:&F = &*(f as *const F);
			f(
				Server::from_glib_borrow(this).unsafe_cast_ref(),
				&from_glib_borrow(message),
				&from_glib_borrow(client),
			)
		}
		unsafe {
			let f:Box_<F> = Box_::new(f);
			connect_raw(
				self.as_ptr() as *mut _,
				b"request-started\0".as_ptr() as *const _,
				Some(transmute::<_, unsafe extern fn()>(
					request_started_trampoline::<Self, F> as *const (),
				)),
				Box_::into_raw(f),
			)
		}
	}

	#[cfg(any(feature = "v2_68", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_68")))]
	fn connect_add_websocket_extension_notify<F:Fn(&Self) + 'static>(
		&self,
		f:F,
	) -> SignalHandlerId {
		unsafe extern fn notify_add_websocket_extension_trampoline<
			P:IsA<Server>,
			F:Fn(&P) + 'static,
		>(
			this:*mut ffi::SoupServer,
			_param_spec:glib::ffi::gpointer,
			f:glib::ffi::gpointer,
		) {
			let f:&F = &*(f as *const F);
			f(Server::from_glib_borrow(this).unsafe_cast_ref())
		}
		unsafe {
			let f:Box_<F> = Box_::new(f);
			connect_raw(
				self.as_ptr() as *mut _,
				b"notify::add-websocket-extension\0".as_ptr() as *const _,
				Some(transmute::<_, unsafe extern fn()>(
					notify_add_websocket_extension_trampoline::<Self, F> as *const (),
				)),
				Box_::into_raw(f),
			)
		}
	}

	#[cfg(any(feature = "v2_44", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_44")))]
	fn connect_http_aliases_notify<F:Fn(&Self) + 'static>(&self, f:F) -> SignalHandlerId {
		unsafe extern fn notify_http_aliases_trampoline<P:IsA<Server>, F:Fn(&P) + 'static>(
			this:*mut ffi::SoupServer,
			_param_spec:glib::ffi::gpointer,
			f:glib::ffi::gpointer,
		) {
			let f:&F = &*(f as *const F);
			f(Server::from_glib_borrow(this).unsafe_cast_ref())
		}
		unsafe {
			let f:Box_<F> = Box_::new(f);
			connect_raw(
				self.as_ptr() as *mut _,
				b"notify::http-aliases\0".as_ptr() as *const _,
				Some(transmute::<_, unsafe extern fn()>(
					notify_http_aliases_trampoline::<Self, F> as *const (),
				)),
				Box_::into_raw(f),
			)
		}
	}

	#[cfg(any(feature = "v2_44", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_44")))]
	fn connect_https_aliases_notify<F:Fn(&Self) + 'static>(&self, f:F) -> SignalHandlerId {
		unsafe extern fn notify_https_aliases_trampoline<P:IsA<Server>, F:Fn(&P) + 'static>(
			this:*mut ffi::SoupServer,
			_param_spec:glib::ffi::gpointer,
			f:glib::ffi::gpointer,
		) {
			let f:&F = &*(f as *const F);
			f(Server::from_glib_borrow(this).unsafe_cast_ref())
		}
		unsafe {
			let f:Box_<F> = Box_::new(f);
			connect_raw(
				self.as_ptr() as *mut _,
				b"notify::https-aliases\0".as_ptr() as *const _,
				Some(transmute::<_, unsafe extern fn()>(
					notify_https_aliases_trampoline::<Self, F> as *const (),
				)),
				Box_::into_raw(f),
			)
		}
	}

	#[cfg(any(feature = "v2_68", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_68")))]
	fn connect_remove_websocket_extension_notify<F:Fn(&Self) + 'static>(
		&self,
		f:F,
	) -> SignalHandlerId {
		unsafe extern fn notify_remove_websocket_extension_trampoline<
			P:IsA<Server>,
			F:Fn(&P) + 'static,
		>(
			this:*mut ffi::SoupServer,
			_param_spec:glib::ffi::gpointer,
			f:glib::ffi::gpointer,
		) {
			let f:&F = &*(f as *const F);
			f(Server::from_glib_borrow(this).unsafe_cast_ref())
		}
		unsafe {
			let f:Box_<F> = Box_::new(f);
			connect_raw(
				self.as_ptr() as *mut _,
				b"notify::remove-websocket-extension\0".as_ptr() as *const _,
				Some(transmute::<_, unsafe extern fn()>(
					notify_remove_websocket_extension_trampoline::<Self, F> as *const (),
				)),
				Box_::into_raw(f),
			)
		}
	}

	fn connect_server_header_notify<F:Fn(&Self) + 'static>(&self, f:F) -> SignalHandlerId {
		unsafe extern fn notify_server_header_trampoline<P:IsA<Server>, F:Fn(&P) + 'static>(
			this:*mut ffi::SoupServer,
			_param_spec:glib::ffi::gpointer,
			f:glib::ffi::gpointer,
		) {
			let f:&F = &*(f as *const F);
			f(Server::from_glib_borrow(this).unsafe_cast_ref())
		}
		unsafe {
			let f:Box_<F> = Box_::new(f);
			connect_raw(
				self.as_ptr() as *mut _,
				b"notify::server-header\0".as_ptr() as *const _,
				Some(transmute::<_, unsafe extern fn()>(
					notify_server_header_trampoline::<Self, F> as *const (),
				)),
				Box_::into_raw(f),
			)
		}
	}
}

impl fmt::Display for Server {
	fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result { f.write_str("Server") }
}
