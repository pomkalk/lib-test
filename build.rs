use std::fs;
use toml::{Table, Value};

fn main () {
    fs::remove_dir_all("./src").unwrap();
    fs::create_dir_all("./src").unwrap();

    tonic_build::configure()
        .build_client(true)
        .build_server(true)
        .out_dir("src")
        .compile(&["protos/greeter.proto", "protos/models.proto"], &["protos"])
        .unwrap();

    let mut modules = Vec::new();
    let files = fs::read_dir("./src").unwrap();
    for item in files {
        let item = item.unwrap();
        let mod_name = item.file_name().into_string().unwrap().strip_suffix(".rs").unwrap().to_string();
        modules.push(mod_name);
    }

    //.map(|x| format!("#[cfg(feature=\"{}\")]\npub mod {};", x, x))
    let a = modules.clone().into_iter()
        .map(|x| format!("#[cfg(feature=\"{}\")]\npub mod {};", x, x))
        .collect::<Vec<String>>()
        .join("\n");
    fs::write("./src/lib.rs", a).unwrap();

    let toml_cfg = fs::read_to_string("./Cargo.toml").unwrap();
    let mut t = toml_cfg.parse::<Table>().unwrap();
    let mut f = Table::new();
    for module in modules {
        f.insert(module.into(), Value::Array(vec![]));
    }
    t.insert("features".into(), f.into());
    fs::write("./Cargo.toml", t.to_string()).unwrap();
}
