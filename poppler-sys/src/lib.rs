#![allow(
    dead_code,
    non_upper_case_globals,
    non_camel_case_types,
    non_snake_case,
    improper_ctypes
)]

extern crate cairo;
extern crate cairo_sys;
extern crate glib;
extern crate glib_sys;
extern crate gio_sys;
extern crate gobject_sys;
extern crate gtypes;


mod dep_types {
    pub use gtypes::{gchar, gint, gdouble, gsize, guint};
    pub use std::os::raw::{c_char, c_int, c_uint, c_long, c_uchar, c_ulong, c_ushort};
    pub use cairo_sys::{cairo_t, cairo_surface_t, cairo_region_t};
    pub use glib_sys::{GType, GError, gboolean, gpointer, GDate, GArray, GList, GTime, GString, GQuark};
    pub use gobject_sys::{GObject, GObjectClass};
    pub type time_t = c_long;
    pub use gio_sys::{GCancellable, GInputStream, GFile};
}

pub mod poppler {
    use super::dep_types::*;
    include!(concat!(env!("OUT_DIR"), "/bindings_poppler.rs"));
}

pub mod poppler_document {
    use super::dep_types::*;
    use super::poppler::*;

    include!(concat!(env!("OUT_DIR"), "/bindings_poppler_document.rs"));
}

// pub mod poppler_page {
//     use super::dep_types::*;

//     include!(concat!(env!("OUT_DIR"), "/bindings_poppler_page.rs"));

// }

// include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn my_lib() {
    println!("hello world");
}
