extern crate bindgen;
extern crate semver;
extern crate pkg_config;

use std::env;
use std::path::PathBuf;

/// Initialize the builder with some include paths
fn builder() -> bindgen::Builder {
    let mut builder = bindgen::Builder::default();

    // have glib2 and cairo be included into clang
    let glib2 = pkg_config::Config::new()
        .atleast_version("2.60.0")
        .probe("glib-2.0")
        .expect("pkg-config could not find glib-2.0");
    let cairo = pkg_config::Config::new()
        .atleast_version("1.16.0")
        .probe("cairo")
        .expect("pkg-config could not find cairo");
    for incl in glib2.include_paths.iter()
        .chain(&cairo.include_paths) {
        builder = builder
            .clang_args(&["-I", incl.to_str().unwrap()]);
    }

    // have wrapping headers be included into clang
    // (the wrapping headers use files from poppler as a library,
    // already linked into rustc)
    builder.clang_arg("-I").clang_arg("build")
        // extra options
        .whitelist_recursively(false)
        // TODO: also add more types and functions? (cairo, etc)
        .whitelist_type("_?Poppler.*")
        .whitelist_function("poppler_.*")
        .whitelist_var("_?Poppler.*")
        .whitelist_var("_?poppler.*")
        .disable_name_namespacing()
}

/// Creates the bindings rs file
fn into_bindings(builder: bindgen::Bindings, name: &str) {
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_path = PathBuf::from(&out_dir);
    builder
        .write_to_file(out_path
            .join(format!("bindings_{}.rs", name))
        )
        .expect("Couldn't write bindings!");
}

fn main() {
    // have poppler be linked into rustc
    pkg_config::Config::new()
        .atleast_version("0.76.0")
        .probe("poppler")
        .expect("pkg-config could not find poppler")
        .libs
        .iter()
        .for_each(|lib| println!("cargo:rustc-link-lib={}", lib));

    // main poppler module
    let b_poppler = builder()
        .header("build/poppler_wrp.h")
        //
        .whitelist_type("guint16")
        // blacklist those who are defined in other modules
        // poppler-action.h
        .blacklist_type("_PopplerAction")
        .blacklist_type("_PopplerDest")
        .blacklist_type("_PopplerActionLayer")
        // poppler-attachment.h
        .blacklist_type("_PopplerAttachment")
        // poppler-page.h
        .blacklist_type("_PopplerPoint")
        .blacklist_type("_PopplerRectangle")
        .blacklist_type("_PopplerTextAttributes")
        .blacklist_type("_PopplerColor")
        .blacklist_type("_PopplerLinkMapping")
        .blacklist_type("_PopplerPageTransition")
        .blacklist_type("_PopplerImageMapping")
        .blacklist_type("_PopplerFormFieldMapping")
        .blacklist_type("_PopplerAnnotMapping")
        .blacklist_type("_PopplerQuadrilateral")
        // poppler-annot.h
        .blacklist_type("_PopplerAnnotCalloutLine")
        // poppler-movie.h
        .blacklist_type("_PopplerMovie")
        //
        .generate()
        .expect("Unable to generate bindings");
    into_bindings(b_poppler, "poppler");

    // poppler-document module
    let b_document = blacklist_types(builder())
        .header("build/poppler_document_wrp.h")
        //
        // poppler-action.h
        .blacklist_type("_PopplerAction")
        .blacklist_type("_PopplerDest")
        .blacklist_type("_PopplerActionLayer")
        // poppler-attachment.h
        .blacklist_type("_PopplerAttachment")
        // poppler-page.h
        .blacklist_type("_PopplerPoint")
        .blacklist_type("_PopplerRectangle")
        .blacklist_type("_PopplerTextAttributes")
        .blacklist_type("_PopplerColor")
        .blacklist_type("_PopplerLinkMapping")
        .blacklist_type("_PopplerPageTransition")
        .blacklist_type("_PopplerImageMapping")
        .blacklist_type("_PopplerFormFieldMapping")
        .blacklist_type("_PopplerAnnotMapping")
        .blacklist_type("_PopplerQuadrilateral")
        // poppler-annot.h
        .blacklist_type("_PopplerAnnotCalloutLine")
        // poppler-movie.h
        .blacklist_type("_PopplerMovie")
        //
        .whitelist_type("goffset")
        .whitelist_type("gint64")
        //
        .generate()
        .expect("Unable to generate bindings");
    into_bindings(b_document, "poppler_document");

    // poppler-page module
    let b_page = blacklist_types(builder())
        .header("build/poppler_page_wrp.h")
        //
        // poppler-action.h
        .blacklist_type("_PopplerAction")
        .blacklist_type("_PopplerDest")
        .blacklist_type("_PopplerActionLayer")
        // poppler-attachment.h
        .blacklist_type("_PopplerAttachment")
        // poppler-annot.h
        .blacklist_type("_PopplerAnnotCalloutLine")
        // poppler-movie.h
        .blacklist_type("_PopplerMovie")
        // explicit
        .whitelist_type("_PopplerPoint")
        .whitelist_type("_PopplerRectangle")
        .whitelist_type("_PopplerTextAttributes")
        .whitelist_type("_PopplerColor")
        .whitelist_type("_PopplerLinkMapping")
        .whitelist_type("_PopplerPageTransition")
        .whitelist_type("_PopplerImageMapping")
        .whitelist_type("_PopplerFormFieldMapping")
        .whitelist_type("_PopplerAnnotMapping")
        .whitelist_type("_PopplerQuadrilateral")
        //
        .generate()
        .expect("Unable to generate bindings");
    into_bindings(b_page, "poppler_page");

    // poppler-action module
    let b_action = blacklist_types(builder())
        .header("build/poppler_action_wrp.h")
        // 
        // poppler-attachment.h
        .blacklist_type("_PopplerAttachment")
        // poppler-page.h
        .blacklist_type("_PopplerPoint")
        .blacklist_type("_PopplerRectangle")
        .blacklist_type("_PopplerTextAttributes")
        .blacklist_type("_PopplerColor")
        .blacklist_type("_PopplerLinkMapping")
        .blacklist_type("_PopplerPageTransition")
        .blacklist_type("_PopplerImageMapping")
        .blacklist_type("_PopplerFormFieldMapping")
        .blacklist_type("_PopplerAnnotMapping")
        .blacklist_type("_PopplerQuadrilateral")
        // poppler-annot.h
        .blacklist_type("_PopplerAnnotCalloutLine")
        // poppler-movie.h
        .blacklist_type("_PopplerMovie")
        //
        .whitelist_type("guint8")
        // explicit
        .whitelist_type("_PopplerAction")
        .whitelist_type("_PopplerDest")
        .whitelist_type("_PopplerActionLayer")
        //
        .generate()
        .expect("Unable to generate bindings");
    into_bindings(b_action, "poppler_action");
    
    // poppler-annot module
    let b_annot = blacklist_types(builder())
        .header("build/poppler_annot_wrp.h")
        // 
        // poppler-action.h
        .blacklist_type("_PopplerAction")
        .blacklist_type("_PopplerDest")
        .blacklist_type("_PopplerActionLayer")
        // poppler-attachment.h
        .blacklist_type("_PopplerAttachment")
        // poppler-page.h
        .blacklist_type("_PopplerPoint")
        .blacklist_type("_PopplerRectangle")
        .blacklist_type("_PopplerTextAttributes")
        .blacklist_type("_PopplerColor")
        .blacklist_type("_PopplerLinkMapping")
        .blacklist_type("_PopplerPageTransition")
        .blacklist_type("_PopplerImageMapping")
        .blacklist_type("_PopplerFormFieldMapping")
        .blacklist_type("_PopplerAnnotMapping")
        .blacklist_type("_PopplerQuadrilateral")
        // poppler-movie.h
        .blacklist_type("_PopplerMovie")
        //
        // explicit
        .whitelist_type("_PopplerAnnotCalloutLine")
        //
        .generate()
        .expect("Unable to generate bindings");
    into_bindings(b_annot, "poppler_annot");

    // poppler-attachment module
    let b_attachment = blacklist_types(builder())
        .header("build/poppler_attachment_wrp.h")
        // 
        // poppler-action.h
        .blacklist_type("_PopplerAction")
        .blacklist_type("_PopplerDest")
        .blacklist_type("_PopplerActionLayer")
        // poppler-page.h
        .blacklist_type("_PopplerPoint")
        .blacklist_type("_PopplerRectangle")
        .blacklist_type("_PopplerTextAttributes")
        .blacklist_type("_PopplerColor")
        .blacklist_type("_PopplerLinkMapping")
        .blacklist_type("_PopplerPageTransition")
        .blacklist_type("_PopplerImageMapping")
        .blacklist_type("_PopplerFormFieldMapping")
        .blacklist_type("_PopplerAnnotMapping")
        .blacklist_type("_PopplerQuadrilateral")
        // poppler-annot.h
        .blacklist_type("_PopplerAnnotCalloutLine")
        // poppler-movie.h
        .blacklist_type("_PopplerMovie")
        // explicit
        .whitelist_type("_PopplerAttachment")
        //
        .generate()
        .expect("Unable to generate bindings");
    into_bindings(b_attachment, "poppler_attachment");

    // poppler-form-field module
    let b_form_field = blacklist_types(builder())
        .header("build/poppler_form_field_wrp.h")
        // 
        // poppler-action.h
        .blacklist_type("_PopplerAction")
        .blacklist_type("_PopplerDest")
        .blacklist_type("_PopplerActionLayer")
        // poppler-page.h
        .blacklist_type("_PopplerPoint")
        .blacklist_type("_PopplerRectangle")
        .blacklist_type("_PopplerTextAttributes")
        .blacklist_type("_PopplerColor")
        .blacklist_type("_PopplerLinkMapping")
        .blacklist_type("_PopplerPageTransition")
        .blacklist_type("_PopplerImageMapping")
        .blacklist_type("_PopplerFormFieldMapping")
        .blacklist_type("_PopplerAnnotMapping")
        .blacklist_type("_PopplerQuadrilateral")
        // poppler-annot.h
        .blacklist_type("_PopplerAnnotCalloutLine")
        // poppler-attachment.h
        .blacklist_type("_PopplerAttachment")
        // poppler-movie.h
        .blacklist_type("_PopplerMovie")
        //
        .generate()
        .expect("Unable to generate bindings");
    into_bindings(b_form_field, "poppler_form_field");

    // poppler-layer module
    let b_layer = blacklist_types(builder())
        .header("build/poppler_layer_wrp.h")
        // 
        // poppler-action.h
        .blacklist_type("_PopplerAction")
        .blacklist_type("_PopplerDest")
        .blacklist_type("_PopplerActionLayer")
        // poppler-page.h
        .blacklist_type("_PopplerPoint")
        .blacklist_type("_PopplerRectangle")
        .blacklist_type("_PopplerTextAttributes")
        .blacklist_type("_PopplerColor")
        .blacklist_type("_PopplerLinkMapping")
        .blacklist_type("_PopplerPageTransition")
        .blacklist_type("_PopplerImageMapping")
        .blacklist_type("_PopplerFormFieldMapping")
        .blacklist_type("_PopplerAnnotMapping")
        .blacklist_type("_PopplerQuadrilateral")
        // poppler-annot.h
        .blacklist_type("_PopplerAnnotCalloutLine")
        // poppler-attachment.h
        .blacklist_type("_PopplerAttachment")
        // poppler-movie.h
        .blacklist_type("_PopplerMovie")
        //
        .generate()
        .expect("Unable to generate bindings");
    into_bindings(b_layer, "poppler_layer");

    // poppler-media module
    let b_media = blacklist_types(builder())
        .header("build/poppler_media_wrp.h")
        // 
        // poppler-action.h
        .blacklist_type("_PopplerAction")
        .blacklist_type("_PopplerDest")
        .blacklist_type("_PopplerActionLayer")
        // poppler-page.h
        .blacklist_type("_PopplerPoint")
        .blacklist_type("_PopplerRectangle")
        .blacklist_type("_PopplerTextAttributes")
        .blacklist_type("_PopplerColor")
        .blacklist_type("_PopplerLinkMapping")
        .blacklist_type("_PopplerPageTransition")
        .blacklist_type("_PopplerImageMapping")
        .blacklist_type("_PopplerFormFieldMapping")
        .blacklist_type("_PopplerAnnotMapping")
        .blacklist_type("_PopplerQuadrilateral")
        // poppler-annot.h
        .blacklist_type("_PopplerAnnotCalloutLine")
        // poppler-attachment.h
        .blacklist_type("_PopplerAttachment")
        // poppler-movie.h
        .blacklist_type("_PopplerMovie")
        //
        .generate()
        .expect("Unable to generate bindings");
    into_bindings(b_media, "poppler_media");

    // poppler-movie module
    let b_movie = blacklist_types(builder())
        .header("build/poppler_movie_wrp.h")
        // 
        // poppler-action.h
        .blacklist_type("_PopplerAction")
        .blacklist_type("_PopplerDest")
        .blacklist_type("_PopplerActionLayer")
        // poppler-page.h
        .blacklist_type("_PopplerPoint")
        .blacklist_type("_PopplerRectangle")
        .blacklist_type("_PopplerTextAttributes")
        .blacklist_type("_PopplerColor")
        .blacklist_type("_PopplerLinkMapping")
        .blacklist_type("_PopplerPageTransition")
        .blacklist_type("_PopplerImageMapping")
        .blacklist_type("_PopplerFormFieldMapping")
        .blacklist_type("_PopplerAnnotMapping")
        .blacklist_type("_PopplerQuadrilateral")
        // poppler-annot.h
        .blacklist_type("_PopplerAnnotCalloutLine")
        // poppler-attachment.h
        .blacklist_type("_PopplerAttachment")
        // explicit
        .whitelist_type("_PopplerMovie")
        //
        .generate()
        .expect("Unable to generate bindings");
    into_bindings(b_movie, "poppler_movie");
}

/// Prevent re-defition of some types.
fn blacklist_types(builder: bindgen::Builder) ->  bindgen::Builder {
    builder
        //
        .blacklist_function("poppler_error_quark")
        .blacklist_function("poppler_get_backend")
        .blacklist_function("poppler_get_version")
        //
        .blacklist_type("PopplerError")
        .blacklist_type("PopplerPageTransitionType")
        .blacklist_type("PopplerPageTransitionAlignment")
        .blacklist_type("PopplerPageTransitionDirection")
        .blacklist_type("PopplerSelectionStyle")
        .blacklist_type("PopplerPrintFlags")
        .blacklist_type("PopplerFindFlags")
        .blacklist_type("PopplerBackend")
        // poppler-private.h
        .blacklist_type("_PopplerDocument")
        .blacklist_type("PopplerDocument")
        .blacklist_type("_PopplerPage")
        .blacklist_type("PopplerPage")
        .blacklist_type("_PopplerFontInfo")
        .blacklist_type("PopplerFontInfo")
        .blacklist_type("_PopplerLayer")
        .blacklist_type("PopplerLayer")
        .blacklist_type("_PopplerPSFile")
        .blacklist_type("PopplerPSFile")
        .blacklist_type("_PopplerFormField")
        .blacklist_type("PopplerFormField")
        .blacklist_type("_PopplerAnnot")
        .blacklist_type("PopplerAnnot")
        .blacklist_type("_PopplerStructureElement")
        .blacklist_type("PopplerStructureElement")
        //
        // poppler-document.cc
        .blacklist_type("_PopplerIndexIter")
        .blacklist_type("PopplerIndexIter")
        .blacklist_type("_PopplerFontsIter")
        .blacklist_type("PopplerFontsIter")
        .blacklist_type("_PopplerLayersIter")
        .blacklist_type("PopplerLayersIter")
        //
        // poppler-page.h
        // .blacklist_type("_PopplerPoint")
        .blacklist_type("PopplerPoint")
        // .blacklist_type("_PopplerRectangle")
        .blacklist_type("PopplerRectangle")
        // .blacklist_type("_PopplerTextAttributes")
        .blacklist_type("PopplerTextAttributes")
        // .blacklist_type("_PopplerColor")
        .blacklist_type("PopplerColor")
        // .blacklist_type("_PopplerLinkMapping")
        .blacklist_type("PopplerLinkMapping")
        // .blacklist_type("_PopplerPageTransition")
        .blacklist_type("PopplerPageTransition")
        // .blacklist_type("_PopplerImageMapping")
        .blacklist_type("PopplerImageMapping")
        // .blacklist_type("_PopplerFormFieldMapping")
        .blacklist_type("PopplerFormFieldMapping")
        // .blacklist_type("_PopplerAnnotMapping")
        .blacklist_type("PopplerAnnotMapping")
        // .blacklist_type("_PopplerQuadrilateral")
        .blacklist_type("PopplerQuadrilateral")
        //
        // poppler-action.h
        // .blacklist_type("_PopplerAction")
        .blacklist_type("PopplerAction")
        // .blacklist_type("_PopplerDest")
        .blacklist_type("PopplerDest")
        // .blacklist_type("_PopplerActionLayer")
        .blacklist_type("PopplerActionLayer")
        //
        // poppler-attachment.h
        // .blacklist_type("_PopplerAttachment")
        .blacklist_type("PopplerAttachment")
        //
        // poppler-movie.h
        // .blacklist_type("_PopplerMovie")
        .blacklist_type("PopplerMovie")
        //
        // poppler-media.cc
        .blacklist_type("_PopplerMedia")
        .blacklist_type("PopplerMedia")
        //
        // poppler-annot.cc
        .blacklist_type("_PopplerAnnotMarkup")
        .blacklist_type("PopplerAnnotMarkup")
        .blacklist_type("_PopplerAnnotText")
        .blacklist_type("PopplerAnnotText")
        .blacklist_type("_PopplerAnnotTextMarkup")
        .blacklist_type("PopplerAnnotTextMarkup")
        .blacklist_type("_PopplerAnnotFreeText")
        .blacklist_type("PopplerAnnotFreeText")
        .blacklist_type("_PopplerAnnotFileAttachment")
        .blacklist_type("PopplerAnnotFileAttachment")
        .blacklist_type("_PopplerAnnotMovie")
        .blacklist_type("PopplerAnnotMovie")
        .blacklist_type("_PopplerAnnotScreen")
        .blacklist_type("PopplerAnnotScreen")
        .blacklist_type("_PopplerAnnotLine")
        .blacklist_type("PopplerAnnotLine")
        .blacklist_type("_PopplerAnnotCircle")
        .blacklist_type("PopplerAnnotCircle")
        .blacklist_type("_PopplerAnnotSquare")
        .blacklist_type("PopplerAnnotSquare")
        //
        // poppler-annot.h
        // .blacklist_type("_PopplerAnnotCalloutLine")
        .blacklist_type("PopplerAnnotCalloutLine")
        // poppler-structure-element.cc
        .blacklist_type("_PopplerStructureElementIter")
        .blacklist_type("PopplerStructureElementIter")
        .blacklist_type("_PopplerTextSpan")
        .blacklist_type("PopplerTextSpan")
        //
}