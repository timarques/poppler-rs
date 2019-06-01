extern crate bindgen;
extern crate semver;

use std::env;
use std::path::PathBuf;

/// Initialize the builder with some include paths
fn builder() -> bindgen::Builder {
    bindgen::Builder::default()
        .clang_arg("-I").clang_arg("/usr/include/glib-2.0")
        .clang_arg("-I").clang_arg("/usr/lib/glib-2.0/include")
        .clang_arg("-I").clang_arg("/usr/include/cairo")
        .clang_arg("-I").clang_arg("poppler/glib")
        .clang_arg("-I").clang_arg("poppler/build/glib")
        .clang_arg("-I").clang_arg("build")
        .whitelist_recursively(false)
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
    println!("cargo:rustc-link-lib=poppler");
    println!("cargo:rerun-if-changed=poppler");

    // main poppler module
    let b_poppler = builder()
        .header("build/poppler_wrp.h")
        .whitelist_type("_?Poppler.*")
        .whitelist_function("poppler_.*")
        //
        .whitelist_type("guint16")
        // blacklist those who are defined in other modules
        .blacklist_type("_PopplerDest") // poppler-action.h
        //
        .generate()
        .expect("Unable to generate bindings");
    into_bindings(b_poppler, "poppler");

    // poppler-document module
    let b_document = blacklist_types(builder())
        .header("build/poppler_document_wrp.h")
        .whitelist_type("_?Poppler.*")
        .whitelist_function("poppler_.*")
        //
        .blacklist_type("_PopplerDest") // poppler-action.h
        .whitelist_type("goffset")
        .whitelist_type("gint64")
        //
        .generate()
        .expect("Unable to generate bindings");
    into_bindings(b_document, "poppler_document");

    // poppler-page module
    let b_page = blacklist_types(builder())
        .header("build/poppler_page_wrp.h")
        .whitelist_type("_?Poppler.*")
        .whitelist_function("poppler_.*")
        //
        .blacklist_type("_PopplerDest") // poppler-action.h
        //
        .generate()
        .expect("Unable to generate bindings");
    into_bindings(b_page, "poppler_page");

    // poppler-action module
    let b_action = blacklist_types(builder())
        .header("build/poppler_action_wrp.h")
        .whitelist_type("_?Poppler.*")
        .whitelist_function("poppler_.*")
        // 
        .whitelist_type("guint8")
        .whitelist_type("_PopplerDest") // explicit
        //
        .generate()
        .expect("Unable to generate bindings");
    into_bindings(b_action, "poppler_action");
    


    // TODO
    // poppler_annot_wrp.h
    // poppler_attachment_wrp.h
    // poppler_features_wrp.h
    // poppler_form_field_wrp.h
    // poppler_layer_wrp.h
    // poppler_media_wrp.h
    // poppler_movie_wrp.h
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
        //
        .blacklist_type("_PopplerDocument")
        .blacklist_type("PopplerDocument")
        .blacklist_type("_PopplerIndexIter")
        .blacklist_type("PopplerIndexIter")
        .blacklist_type("_PopplerFontsIter")
        .blacklist_type("PopplerFontsIter")
        .blacklist_type("_PopplerLayersIter")
        .blacklist_type("PopplerLayersIter")
        .blacklist_type("_PopplerPoint")
        .blacklist_type("PopplerPoint")
        .blacklist_type("_PopplerRectangle")
        .blacklist_type("PopplerRectangle")
        .blacklist_type("_PopplerTextAttributes")
        .blacklist_type("PopplerTextAttributes")
        .blacklist_type("_PopplerColor")
        .blacklist_type("PopplerColor")
        .blacklist_type("_PopplerLinkMapping")
        .blacklist_type("PopplerLinkMapping")
        .blacklist_type("_PopplerPageTransition")
        .blacklist_type("PopplerPageTransition")
        .blacklist_type("_PopplerImageMapping")
        .blacklist_type("PopplerImageMapping")
        .blacklist_type("_PopplerFormFieldMapping")
        .blacklist_type("PopplerFormFieldMapping")
        .blacklist_type("_PopplerAnnotMapping")
        .blacklist_type("PopplerAnnotMapping")
        .blacklist_type("_PopplerPage")
        .blacklist_type("PopplerPage")
        .blacklist_type("_PopplerFontInfo")
        .blacklist_type("PopplerFontInfo")
        .blacklist_type("_PopplerLayer")
        .blacklist_type("PopplerLayer")
        .blacklist_type("_PopplerPSFile")
        .blacklist_type("PopplerPSFile")
        .blacklist_type("_PopplerAction")
        .blacklist_type("PopplerAction")
        // .blacklist_type("_PopplerDest") // poppler-action.h
        .blacklist_type("PopplerDest")
        .blacklist_type("_PopplerActionLayer")
        .blacklist_type("PopplerActionLayer")
        .blacklist_type("_PopplerFormField")
        .blacklist_type("PopplerFormField")
        .blacklist_type("_PopplerAttachment")
        .blacklist_type("PopplerAttachment")
        .blacklist_type("_PopplerMovie")
        .blacklist_type("PopplerMovie")
        .blacklist_type("_PopplerMedia")
        .blacklist_type("PopplerMedia")
        .blacklist_type("_PopplerAnnot")
        .blacklist_type("PopplerAnnot")
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
        .blacklist_type("_PopplerAnnotCalloutLine")
        .blacklist_type("PopplerAnnotCalloutLine")
        .blacklist_type("_PopplerAnnotLine")
        .blacklist_type("PopplerAnnotLine")
        .blacklist_type("_PopplerAnnotCircle")
        .blacklist_type("PopplerAnnotCircle")
        .blacklist_type("_PopplerAnnotSquare")
        .blacklist_type("PopplerAnnotSquare")
        .blacklist_type("_PopplerQuadrilateral")
        .blacklist_type("PopplerQuadrilateral")
        .blacklist_type("_PopplerStructureElement")
        .blacklist_type("PopplerStructureElement")
        .blacklist_type("_PopplerStructureElementIter")
        .blacklist_type("PopplerStructureElementIter")
        .blacklist_type("_PopplerTextSpan")
        .blacklist_type("PopplerTextSpan")
        //
}