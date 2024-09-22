use std::{ffi::OsStr, path::Path};

use anyhow::Result;
use cairo_lang_filesystem::{
    db::{CrateConfiguration, FilesGroup, FilesGroupEx},
    ids::{CrateLongId, Directory},
};

use cairo_lang_compiler::{
    compile_prepared_db,
    db::RootDatabase,
    diagnostics::{get_diagnostics_as_string, DiagnosticsReporter},
    project::ProjectError,
    CompilerConfig,
};
use cairo_lang_sierra::program::Program;
// use cairo_lang_utils::extract_matches;

// /// Compiles a Cairo project with input String.
// /// The project must be a valid Cairo project:
// /// Either a standalone `.cairo` file (a single crate), or a directory with a `cairo_project.toml`
// /// file.
// /// # Arguments
// /// * `path` - The path to the project.
// /// * `compiler_config` - The compiler configuration.
// /// # Returns
// /// * `Ok(Program)` - The compiled program.
// /// * `Err(anyhow::Error)` - Compilation failed.
// pub fn compile_cairo_project_with_input_string(
//     path: &Path,
//     input: &String,
//     compiler_config: CompilerConfig<'_>,
// ) -> Result<Program> {
//     let mut db = RootDatabase::builder().detect_corelib().build()?; //build a hashmap of corelib
//     let main_crate_ids = setup_project_with_input_string(&mut db, path, input)?; // Set up need to build file
//     if DiagnosticsReporter::stderr().check(&db) {
//         // TODO: Check if this need extra crate ids.
//         let err_string = get_diagnostics_as_string(&mut db, &[]);
//         anyhow::bail!("failed to compile:\n {}", err_string);
//     }
//     Ok(compile_prepared_db(&mut db, main_crate_ids, compiler_config)?.program)
// }

// /// Setup the 'db' to compile the project in the given string.
// /// Returns the ids of the project crates.
// pub fn setup_project_with_input_string(
//     db: &mut dyn SemanticGroup,
//     path: &Path,
//     input: &String,
// ) -> Result<Vec<CrateId>, ProjectError> {
//     Ok(vec![setup_single_file_project_with_input_string(
//         db, path, input,
//     )?])
// }

// /// Setup to 'db' to compile the file at the given path.
// /// Returns the id of the generated crate.
// pub fn setup_single_file_project_with_input_string(
//     db: &mut dyn SemanticGroup,
//     path: &Path,
//     input: &String,
// ) -> Result<CrateId, ProjectError> {
//     /*match path.extension().and_then(OsStr::to_str) {
//         Some("cairo") => (),
//         _ => {
//             return Err(ProjectError::BadFileExtension);
//         }
//     }*/
//     /*if !path.exists() {
//         return Err(ProjectError::NoSuchFile { path: path.to_string_lossy().to_string() });
//     }*/
//     let bad_path_err = || ProjectError::BadPath {
//         path: path.to_string_lossy().to_string(),
//     };
//     let file_stem = "astro";
//     // let file_stem = path.file_stem().and_then(OsStr::to_str).ok_or_else(bad_path_err)?;
//     if file_stem == "lib" {
//         let canonical = path.canonicalize().map_err(|_| bad_path_err())?;
//         let file_dir = canonical.parent().ok_or_else(bad_path_err)?;
//         let crate_name = file_dir.to_str().ok_or_else(bad_path_err)?;
//         let crate_id = db.intern_crate(CrateLongId::Real(crate_name.into()));
//         db.set_crate_config(
//             crate_id,
//             Some(CrateConfiguration::default_for_root(Directory::Real(
//                 file_dir.to_path_buf(),
//             ))),
//         );
//         Ok(crate_id)
//     } else {
//         // If file_stem is not lib, create a fake lib file.
//         let crate_id = db.intern_crate(CrateLongId::Real(file_stem.into()));
//         db.set_crate_config(
//             crate_id,
//             Some(CrateConfiguration::default_for_root(Directory::Real(
//                 path.parent().unwrap().to_path_buf(),
//             ))),
//         );

//         let module_id = ModuleId::CrateRoot(crate_id);
//         let file_id = db.module_main_file(module_id).unwrap();
//         db.as_files_group_mut().override_file_content(
//             file_id, //Some(Arc::new(format!("mod {file_stem};")))
//             Some(Arc::new(input.clone())),
//         );
//         Ok(crate_id)
//     }
// }

pub fn compile_cairo_project(path: &Path, compiler_config: CompilerConfig<'_>) -> Program {
    let bad_path_err = || ProjectError::BadPath {
        path: path.to_string_lossy().to_string(),
    };
    let mut db = RootDatabase::builder().detect_corelib().build()?;
    //build a hashmap of corelib
    let _file_stem = path
        .file_stem()
        .and_then(OsStr::to_str)
        .ok_or_else(bad_path_err)?;

    let canonical = path.canonicalize().map_err(|_| bad_path_err())?;
    let file_dir = canonical.parent().ok_or_else(bad_path_err)?;
    let crate_name = file_dir.to_str().ok_or_else(bad_path_err)?;
    let crate_id = db.intern_crate(CrateLongId::Real(crate_name.into()));
    db.set_crate_config(
        crate_id,
        Some(CrateConfiguration::default_for_root(Directory::Real(
            file_dir.to_path_buf(),
        ))),
    );

    if DiagnosticsReporter::stderr().check(&db) {
        // TODO: Check if this need extra crate ids.
        let err_string = get_diagnostics_as_string(&mut db, &[]);
        panic!("failed to compile:\n {}", err_string);
    }
    Ok(compile_prepared_db(&mut db, vec![crate_id], compiler_config)?.program).unwrap()
}
