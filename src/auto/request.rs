// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

use crate::Session;
use crate::URI;
use glib::object::IsA;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::pin::Pin;
use std::ptr;

glib::wrapper! {
    #[doc(alias = "SoupRequest")]
    pub struct Request(Object<ffi::SoupRequest, ffi::SoupRequestClass>);

    match fn {
        type_ => || ffi::soup_request_get_type(),
    }
}

pub const NONE_REQUEST: Option<&Request> = None;

pub trait RequestExt: 'static {
    #[doc(alias = "soup_request_get_content_length")]
    #[doc(alias = "get_content_length")]
    fn content_length(&self) -> i64;

    #[doc(alias = "soup_request_get_content_type")]
    #[doc(alias = "get_content_type")]
    fn content_type(&self) -> Option<glib::GString>;

    #[doc(alias = "soup_request_get_session")]
    #[doc(alias = "get_session")]
    fn session(&self) -> Option<Session>;

    #[doc(alias = "soup_request_get_uri")]
    #[doc(alias = "get_uri")]
    fn uri(&self) -> Option<URI>;

    #[doc(alias = "soup_request_send")]
    fn send(&self, cancellable: Option<&impl IsA<gio::Cancellable>>) -> Result<gio::InputStream, glib::Error>;

    #[doc(alias = "soup_request_send_async")]
    fn send_async<P: FnOnce(Result<gio::InputStream, glib::Error>) + Send + 'static>(&self, cancellable: Option<&impl IsA<gio::Cancellable>>, callback: P);

    
    fn send_async_future(&self) -> Pin<Box_<dyn std::future::Future<Output = Result<gio::InputStream, glib::Error>> + 'static>>;
}

impl<O: IsA<Request>> RequestExt for O {
    fn content_length(&self) -> i64 {
        unsafe {
            ffi::soup_request_get_content_length(self.as_ref().to_glib_none().0)
        }
    }

    fn content_type(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::soup_request_get_content_type(self.as_ref().to_glib_none().0))
        }
    }

    fn session(&self) -> Option<Session> {
        unsafe {
            from_glib_none(ffi::soup_request_get_session(self.as_ref().to_glib_none().0))
        }
    }

    fn uri(&self) -> Option<URI> {
        unsafe {
            from_glib_none(ffi::soup_request_get_uri(self.as_ref().to_glib_none().0))
        }
    }

    fn send(&self, cancellable: Option<&impl IsA<gio::Cancellable>>) -> Result<gio::InputStream, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::soup_request_send(self.as_ref().to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn send_async<P: FnOnce(Result<gio::InputStream, glib::Error>) + Send + 'static>(&self, cancellable: Option<&impl IsA<gio::Cancellable>>, callback: P) {
        let user_data: Box_<P> = Box_::new(callback);
        unsafe extern "C" fn send_async_trampoline<P: FnOnce(Result<gio::InputStream, glib::Error>) + Send + 'static>(_source_object: *mut glib::gobject_ffi::GObject, res: *mut gio::ffi::GAsyncResult, user_data: glib::ffi::gpointer) {
            let mut error = ptr::null_mut();
            let ret = ffi::soup_request_send_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) };
            let callback: Box_<P> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = send_async_trampoline::<P>;
        unsafe {
            ffi::soup_request_send_async(self.as_ref().to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, Some(callback), Box_::into_raw(user_data) as *mut _);
        }
    }

    
    fn send_async_future(&self) -> Pin<Box_<dyn std::future::Future<Output = Result<gio::InputStream, glib::Error>> + 'static>> {

        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.send_async(
                Some(cancellable),
                move |res| {
                    send.resolve(res);
                },
            );
        }))
    }
}

impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Request")
    }
}
