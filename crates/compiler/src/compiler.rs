use std::{ffi::OsStr, path::Path};

use anyhow::Result;
use cairo_lang_compiler::{
    compile_prepared_db_program,
    db::RootDatabase,
    diagnostics::{get_diagnostics_as_string, DiagnosticsReporter},
    project::ProjectError,
    CompilerConfig,
};
use cairo_lang_runner::{RunResultStarknet, SierraCasmRunner, StarknetState};
use cairo_lang_sierra_generator::replace_ids::DebugReplacer;
// use cairo_lang_sierra_generator::{db::SierraGenGroup, program_generator::SierraProgramWithDebug};

use cairo_lang_filesystem::{
    db::{CrateConfiguration, FilesGroup, FilesGroupEx},
    ids::{CrateId, CrateLongId, Directory},
};
use cairo_lang_sierra::program::Program;
use cairo_lang_starknet::contract::get_contracts_info;
// use cairo_lang_starknet::contract::get_contracts_info;
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

pub fn compile_cairo_project(
    path: &Path,
    compiler_config: CompilerConfig<'_>,
) -> Result<(Program, CrateId)> {
    compile_cairo_project_with_db(path, compiler_config, &mut perpare_db()?)
}

pub fn compile_cairo_project_with_db(
    path: &Path,
    compiler_config: CompilerConfig<'_>,
    mut db: &mut RootDatabase,
) -> Result<(Program, CrateId)> {
    let bad_path_err = || ProjectError::BadPath {
        path: path.to_string_lossy().to_string(),
    };

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
        anyhow::bail!("failed to compile:\n {}", err_string);
    }
    Ok((
        compile_prepared_db_program(&mut db, vec![crate_id], compiler_config)?,
        crate_id,
    ))
}

fn perpare_db() -> anyhow::Result<RootDatabase> {
    let mut db_builder = RootDatabase::builder();
    db_builder.detect_corelib();
    db_builder.skip_auto_withdraw_gas();
    Ok(db_builder.build()?)
}

pub fn run_cairo_project(
    path: &Path,
    compiler_config: CompilerConfig<'_>,
) -> anyhow::Result<RunResultStarknet> {
    let mut db = perpare_db()?;
    let (program, crate_id) = compile_cairo_project_with_db(path, compiler_config, &mut db)?;

    let main_crate_ids = vec![crate_id];
    let replacer = DebugReplacer { db: &db };
    let contracts_info = get_contracts_info(&db, main_crate_ids, &replacer)?;

    let runner = SierraCasmRunner::new(program.clone(), None, contracts_info, None)
        .map_err(|err| anyhow::anyhow!(err.to_string()))?;

    let result = runner
        .run_function_with_starknet_context(
            runner
                .find_function("::main")
                .map_err(|err| anyhow::anyhow!(err.to_string()))?,
            &[],
            None,
            StarknetState::default(),
        )
        // .with_context(|| "Failed to run the function.")?;
        .map_err(|err| anyhow::anyhow!(err.to_string()))?;

    Ok(result)
}

pub fn run_cairo_program(
    program: Program,
    _compiler_config: CompilerConfig<'_>,
) -> anyhow::Result<RunResultStarknet> {
    let db = perpare_db()?;

    let main_crate_ids = vec![];
    let replacer = DebugReplacer { db: &db };
    let contracts_info = get_contracts_info(&db, main_crate_ids, &replacer)?;

    let runner = SierraCasmRunner::new(program.clone(), None, contracts_info, None)
        .map_err(|err| anyhow::anyhow!(err.to_string()))?;

    let result = runner
        .run_function_with_starknet_context(
            runner
                .find_function("::main")
                .map_err(|err| anyhow::anyhow!(err.to_string()))?,
            &[],
            None,
            StarknetState::default(),
        )
        // .with_context(|| "Failed to run the function.")?;
        .map_err(|err| anyhow::anyhow!(err.to_string()))?;

    Ok(result)
}
