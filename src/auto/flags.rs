// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

use bitflags::bitflags;
use glib::translate::*;
use glib::value::FromValue;
use glib::value::ToValue;
use glib::StaticType;
use glib::Type;
use std::fmt;

bitflags! {
    #[doc(alias = "SoupCacheability")]
    pub struct Cacheability: u32 {
        #[doc(alias = "SOUP_CACHE_CACHEABLE")]
        const CACHEABLE = ffi::SOUP_CACHE_CACHEABLE as u32;
        #[doc(alias = "SOUP_CACHE_UNCACHEABLE")]
        const UNCACHEABLE = ffi::SOUP_CACHE_UNCACHEABLE as u32;
        #[doc(alias = "SOUP_CACHE_INVALIDATES")]
        const INVALIDATES = ffi::SOUP_CACHE_INVALIDATES as u32;
        #[doc(alias = "SOUP_CACHE_VALIDATES")]
        const VALIDATES = ffi::SOUP_CACHE_VALIDATES as u32;
    }
}

impl fmt::Display for Cacheability {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for Cacheability {
    type GlibType = ffi::SoupCacheability;

    fn into_glib(self) -> ffi::SoupCacheability {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::SoupCacheability> for Cacheability {
    unsafe fn from_glib(value: ffi::SoupCacheability) -> Self {
        crate::skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for Cacheability {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::soup_cacheability_get_type()) }
    }
}

impl glib::value::ValueType for Cacheability {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for Cacheability {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        crate::skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for Cacheability {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    #[doc(alias = "SoupExpectation")]
    pub struct Expectation: u32 {
        #[doc(alias = "SOUP_EXPECTATION_UNRECOGNIZED")]
        const UNRECOGNIZED = ffi::SOUP_EXPECTATION_UNRECOGNIZED as u32;
        #[doc(alias = "SOUP_EXPECTATION_CONTINUE")]
        const CONTINUE = ffi::SOUP_EXPECTATION_CONTINUE as u32;
    }
}

impl fmt::Display for Expectation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for Expectation {
    type GlibType = ffi::SoupExpectation;

    fn into_glib(self) -> ffi::SoupExpectation {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::SoupExpectation> for Expectation {
    unsafe fn from_glib(value: ffi::SoupExpectation) -> Self {
        crate::skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for Expectation {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::soup_expectation_get_type()) }
    }
}

impl glib::value::ValueType for Expectation {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for Expectation {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        crate::skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for Expectation {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    #[doc(alias = "SoupMessageFlags")]
    pub struct MessageFlags: u32 {
        #[doc(alias = "SOUP_MESSAGE_NO_REDIRECT")]
        const NO_REDIRECT = ffi::SOUP_MESSAGE_NO_REDIRECT as u32;
        #[doc(alias = "SOUP_MESSAGE_CAN_REBUILD")]
        const CAN_REBUILD = ffi::SOUP_MESSAGE_CAN_REBUILD as u32;
        #[doc(alias = "SOUP_MESSAGE_OVERWRITE_CHUNKS")]
        const OVERWRITE_CHUNKS = ffi::SOUP_MESSAGE_OVERWRITE_CHUNKS as u32;
        #[doc(alias = "SOUP_MESSAGE_CONTENT_DECODED")]
        const CONTENT_DECODED = ffi::SOUP_MESSAGE_CONTENT_DECODED as u32;
        #[doc(alias = "SOUP_MESSAGE_CERTIFICATE_TRUSTED")]
        const CERTIFICATE_TRUSTED = ffi::SOUP_MESSAGE_CERTIFICATE_TRUSTED as u32;
        #[doc(alias = "SOUP_MESSAGE_NEW_CONNECTION")]
        const NEW_CONNECTION = ffi::SOUP_MESSAGE_NEW_CONNECTION as u32;
        #[doc(alias = "SOUP_MESSAGE_IDEMPOTENT")]
        const IDEMPOTENT = ffi::SOUP_MESSAGE_IDEMPOTENT as u32;
        #[doc(alias = "SOUP_MESSAGE_IGNORE_CONNECTION_LIMITS")]
        const IGNORE_CONNECTION_LIMITS = ffi::SOUP_MESSAGE_IGNORE_CONNECTION_LIMITS as u32;
        #[doc(alias = "SOUP_MESSAGE_DO_NOT_USE_AUTH_CACHE")]
        const DO_NOT_USE_AUTH_CACHE = ffi::SOUP_MESSAGE_DO_NOT_USE_AUTH_CACHE as u32;
    }
}

impl fmt::Display for MessageFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for MessageFlags {
    type GlibType = ffi::SoupMessageFlags;

    fn into_glib(self) -> ffi::SoupMessageFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::SoupMessageFlags> for MessageFlags {
    unsafe fn from_glib(value: ffi::SoupMessageFlags) -> Self {
        crate::skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for MessageFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::soup_message_flags_get_type()) }
    }
}

impl glib::value::ValueType for MessageFlags {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for MessageFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        crate::skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for MessageFlags {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[cfg(any(feature = "v2_48", feature = "dox"))]
bitflags! {
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_48")))]
    #[doc(alias = "SoupServerListenOptions")]
    pub struct ServerListenOptions: u32 {
        #[doc(alias = "SOUP_SERVER_LISTEN_HTTPS")]
        const HTTPS = ffi::SOUP_SERVER_LISTEN_HTTPS as u32;
        #[doc(alias = "SOUP_SERVER_LISTEN_IPV4_ONLY")]
        const IPV4_ONLY = ffi::SOUP_SERVER_LISTEN_IPV4_ONLY as u32;
        #[doc(alias = "SOUP_SERVER_LISTEN_IPV6_ONLY")]
        const IPV6_ONLY = ffi::SOUP_SERVER_LISTEN_IPV6_ONLY as u32;
    }
}

#[cfg(any(feature = "v2_48", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_48")))]
impl fmt::Display for ServerListenOptions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[cfg(any(feature = "v2_48", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_48")))]
#[doc(hidden)]
impl IntoGlib for ServerListenOptions {
    type GlibType = ffi::SoupServerListenOptions;

    fn into_glib(self) -> ffi::SoupServerListenOptions {
        self.bits()
    }
}

#[cfg(any(feature = "v2_48", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_48")))]
#[doc(hidden)]
impl FromGlib<ffi::SoupServerListenOptions> for ServerListenOptions {
    unsafe fn from_glib(value: ffi::SoupServerListenOptions) -> Self {
        crate::skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

#[cfg(any(feature = "v2_48", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_48")))]
impl StaticType for ServerListenOptions {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::soup_server_listen_options_get_type()) }
    }
}

#[cfg(any(feature = "v2_48", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_48")))]
impl glib::value::ValueType for ServerListenOptions {
    type Type = Self;
}

#[cfg(any(feature = "v2_48", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_48")))]
unsafe impl<'a> FromValue<'a> for ServerListenOptions {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        crate::skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

#[cfg(any(feature = "v2_48", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_48")))]
impl ToValue for ServerListenOptions {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

