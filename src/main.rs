#![deny(anonymous_parameters, coerce_never, const_err, dead_code, deprecated, exceeding_bitshifts,
        illegal_floating_point_literal_pattern, improper_ctypes, incoherent_fundamental_impls,
        invalid_type_param_default, late_bound_lifetime_arguments, legacy_constructor_visibility,
        legacy_directory_ownership, legacy_imports, missing_copy_implementations, missing_docs,
        missing_fragment_specifier, mutable_transmutes, no_mangle_const_items,
        no_mangle_generic_items, non_camel_case_types, non_shorthand_field_patterns,
        non_snake_case, non_upper_case_globals, overflowing_literals,
        parenthesized_params_in_types_and_modules, path_statements, patterns_in_fns_without_body,
        plugin_as_library, private_in_public, private_no_mangle_fns, private_no_mangle_statics,
        pub_use_of_private_extern_crate, renamed_and_removed_lints,
        resolve_trait_on_defaulted_unit, safe_extern_statics, safe_packed_borrows,
        stable_features, trivial_numeric_casts, tyvar_behind_raw_pointer, unconditional_recursion,
        unions_with_drop_fields, unknown_crate_types, unreachable_code, unreachable_patterns,
        unreachable_pub, unsafe_code, unstable_features, unused_allocation, unused_assignments,
        unused_attributes, unused_comparisons, unused_doc_comment, unused_extern_crates,
        unused_features, unused_import_braces, unused_imports, unused_macros, unused_must_use,
        unused_mut, unused_parens, unused_qualifications, unused_unsafe, unused_variables,
        while_true)]
#![warn(unknown_lints)]
#![allow(box_pointers, elided_lifetime_in_path, missing_debug_implementations,
         single_use_lifetime, trivial_casts, unused_results, variant_size_differences, warnings)]
#![cfg_attr(feature = "clippy", feature(plugin))]

//! # cargo-make
//!
//! Rust task runner and build tool.<br>
//! The cargo-make task runner enables to define and configure sets of tasks and run them as a flow.<br>
//! A task is a command or a script to execute.<br>
//! Tasks can have dependencies which are also tasks that will be executed before the task itself.<br>
//! With a simple toml based configuration file, you can define a multi platform build script that can run build, test,
//! documentation generation, bench tests execution, security validations and more by running a single command.
//!
//! ## Installation
//! In order to install, just run the following command
//!
//! ```sh
//! cargo install cargo-make
//! ```
//!
//! This will install cargo-make in your ~/.cargo/bin.<br>
//! Make sure to add ~/.cargo/bin directory to your PATH variable.
//!
//! # Contributing
//! See [contributing guide](https://github.com/sagiegurari/simple_redis/blob/master/.github/CONTRIBUTING.md)
//!
//! # License
//! Developed by Sagie Gur-Ari and licensed under the
//! [Apache 2](https://github.com/sagiegurari/simple_redis/blob/master/LICENSE) open source license.
//!

extern crate ci_info;
extern crate clap;
extern crate fern;
extern crate glob;
extern crate indexmap;
#[macro_use]
extern crate log;
extern crate rand;
extern crate run_script;
extern crate rust_info;
extern crate semver;
#[macro_use]
extern crate serde_derive;
extern crate shell2batch;
extern crate toml;

// make types public for docs
pub mod types;

mod logger;
mod config;
mod cache;
mod descriptor;
mod environment;
mod command;
mod installer;
mod scriptengine;
mod condition;
mod runner;
mod cli;
mod version;

#[cfg(test)]
#[path = "./main_test.rs"]
mod main_test;

#[cfg(test)]
#[path = "./test.rs"]
mod test;

fn main() {
    cli::run_cli();
}
