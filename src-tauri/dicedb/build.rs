use std::fs;
use std::io::Write;
use std::path::Path;

fn main() {
    // Create output directory
    let out_dir = "src/wire";
    fs::create_dir_all(out_dir).unwrap();

    // List of proto files (without paths/extensions)
    let protos = ["cmd", "res"];

    // Generate each proto file separately
    for proto in &protos {
        let mut config = prost_build::Config::new();
        config.out_dir(out_dir);

        // Compile single proto file
        config
            .compile_protos(&[format!("protos/{}.proto", proto)], &["protos"])
            .unwrap_or_else(|e| panic!("Failed to compile {}: {}", proto, e));

        // Rename the output file to match our desired pattern
        let default_path = format!("{}/{}.rs", out_dir, proto);
        let desired_path = format!("{}/{}.rs", out_dir, proto);
        if Path::new(&default_path).exists() {
            fs::rename(default_path, desired_path).unwrap();
        }
    }

    // Generate mod.rs
    generate_mod_file(&protos, out_dir).unwrap();
}

fn generate_mod_file(protos: &[&str], out_dir: &str) -> std::io::Result<()> {
    let mut mod_file = fs::File::create(format!("{}/mod.rs", out_dir))?;
    writeln!(mod_file, "// Auto-generated module file")?;

    for proto in protos {
        writeln!(mod_file, "pub mod {};", proto)?;
    }

    Ok(())
}
