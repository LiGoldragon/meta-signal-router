use std::{env, path::PathBuf};

use schema_rust::build::{DependencySchema, GenerationDriver, GenerationPlan};

fn main() {
    SchemaBuild::from_environment().run();
}

struct SchemaBuild {
    crate_root: PathBuf,
}

impl SchemaBuild {
    fn from_environment() -> Self {
        Self {
            crate_root: PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("manifest dir set")),
        }
    }

    fn run(&self) {
        println!("cargo:rerun-if-changed=schema/lib.schema");
        println!("cargo:rerun-if-changed=src/schema/lib.rs");
        println!("cargo:rerun-if-env-changed=DEP_SIGNAL_ROUTER_SCHEMA_DIR");
        println!(
            "cargo::metadata=schema-dir={}",
            self.crate_root.join("schema").display()
        );

        let ordinary_signal =
            DependencySchema::from_cargo_metadata("signal-router", "signal-router", "0.4.1")
                .expect("read signal-router schema metadata")
                .expect("signal-router schema directory exposed via DEP_SIGNAL_ROUTER_SCHEMA_DIR");

        GenerationDriver::new(
            GenerationPlan::wire_contract(&self.crate_root, "meta-signal-router", "0.3.0")
                .with_dependency_schema(ordinary_signal),
        )
        .generate()
        .expect("generate meta-signal-router schema artifacts")
        .write_or_check("META_SIGNAL_ROUTER_UPDATE_SCHEMA_ARTIFACTS")
        .expect("checked-in meta-signal-router schema artifacts are fresh");
    }
}
