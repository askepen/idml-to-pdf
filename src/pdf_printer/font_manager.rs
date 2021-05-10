use crate::idml_parser::IDMLResources;
use dirs;
use libharu_sys::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::CString;
use std::fs;
use std::iter::FromIterator;
use std::path::PathBuf;
use std::ptr;

#[derive(Debug)]
pub struct FontLibrary {
    fonts: HashMap<(String, String), HPDF_Font>,
}

impl FontLibrary {
    pub fn font_from_idml_name_and_style(&self, name: &str, style: &str) -> Option<&HPDF_Font> {
        // println!("{} {}", name, style);
        self.fonts.get(&(name.to_string(), style.to_string()))
    }

    pub fn new(
        idml_resources: &IDMLResources,
        pdf_doc: HPDF_Doc,
        resource_dir: &Option<PathBuf>,
    ) -> Result<FontLibrary, String> {
        // Load every font from every font-family in the IDML resources
        let font_refs = idml_resources
            .fonts()
            .font_families()
            .into_iter()
            .flat_map(|family| {
                family.fonts().iter().map(|font| {
                    (
                        (
                            font.font_family().to_string(),
                            font.font_style_name().to_string(),
                        ),
                        load_font_from_id(resource_dir, pdf_doc, font.post_script_name()).unwrap(),
                    )
                })
            });

        let font_lib = FontLibrary {
            fonts: HashMap::from_iter(font_refs),
        };

        Ok(font_lib)
    }
}

fn load_font_from_id(
    resource_dir: &Option<PathBuf>,
    pdf_doc: HPDF_Doc,
    id: &str,
) -> Result<HPDF_Font, String> {
    /// Get a list of paths to every file matching font_name in a given directory
    fn find_font_in_dir(font_name: &str, dir: &PathBuf) -> Vec<PathBuf> {
        fs::read_dir(dir)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .filter(|path| {
                path.file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .contains(font_name)
            })
            .collect()
    }

    let mut font_lookup = vec![];

    // Search in provided font directory
    if let Some(font_dir) = resource_dir {
        font_lookup.append(&mut find_font_in_dir(id, &font_dir));
    }

    // // Search in the OS font directory
    if let Some(font_dir) = dirs::font_dir() {
        font_lookup.append(&mut find_font_in_dir(id, &font_dir));
    }

    unsafe {
        match &font_lookup[..] {
            // [] => Err("No font matched id".to_string()),
            [] => {
                // println!("No font matched: {}", id);
                let font = HPDF_GetFont(
                    pdf_doc,
                    CString::new("Times-Roman").unwrap().as_ptr(),
                    ptr::null_mut(),
                );
                Ok(font)
            }
            [font_path] => {
                let font_name = HPDF_LoadTTFontFromFile(
                    pdf_doc,
                    CString::new(font_path.to_str().unwrap()).unwrap().as_ptr(),
                    HPDF_TRUE,
                );
                println!("{:?}: {:#?}", id, font_path);
                let font = HPDF_GetFont(pdf_doc, font_name, ptr::null_mut());
                Ok(font)
            }
            [font_path, ..] => {
                let font_name = HPDF_LoadTTFontFromFile(
                    pdf_doc,
                    CString::new(font_path.to_str().unwrap()).unwrap().as_ptr(),
                    HPDF_FALSE,
                );
                let font = HPDF_GetFont(pdf_doc, font_name, ptr::null_mut());
                Ok(font)
            }
        }
    }
}
