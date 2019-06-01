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
    use super::poppler_action::{_PopplerAction, _PopplerDest, _PopplerActionLayer};
    use super::poppler_attachment::_PopplerAttachment;
    use super::poppler_page::{
        _PopplerPoint,
        _PopplerRectangle,
        _PopplerTextAttributes,
        _PopplerColor,
        _PopplerLinkMapping,
        _PopplerPageTransition,
        _PopplerImageMapping,
        _PopplerFormFieldMapping,
        _PopplerAnnotMapping,
        _PopplerQuadrilateral
    };
    include!(concat!(env!("OUT_DIR"), "/bindings_poppler.rs"));
}

pub mod poppler_document {
    use super::dep_types::*;
    use super::poppler::*;

    include!(concat!(env!("OUT_DIR"), "/bindings_poppler_document.rs"));
}

pub mod poppler_page {
    use super::dep_types::*;
    use super::poppler::*;

    include!(concat!(env!("OUT_DIR"), "/bindings_poppler_page.rs"));
}

pub mod poppler_action {
    use super::dep_types::*;
    use super::poppler::*;

    include!(concat!(env!("OUT_DIR"), "/bindings_poppler_action.rs"));
}

pub mod poppler_annot {
    use super::dep_types::*;
    use super::poppler::*;

    include!(concat!(env!("OUT_DIR"), "/bindings_poppler_annot.rs"));
}

pub mod poppler_attachment {
    use super::dep_types::*;
    use super::poppler::*;

    include!(concat!(env!("OUT_DIR"), "/bindings_poppler_attachment.rs"));
}