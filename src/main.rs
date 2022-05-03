fn main() {
    let cargo_home = home::cargo_home().expect("could not get cargo-home");

    let registry_sources_dir = {
        let mut t = cargo_home.clone();
        t.push("registry");
        t.push("src");
        t.push("github.com-1ecc6299db9ec823/");
        t
    };

    let registry_sources = std::fs::read_dir(registry_sources_dir).unwrap();

    /*
    let git_checkout_sources =  {
        let mut t = cargo_home.clone();
        t.push("registry");
        t.push("src");
        t.push("github.com-1ecc6299db9ec823/");
        t
    };  */

    for krate in registry_sources {
        let p = krate.unwrap().path();
        let paths = std::fs::read_dir(&p)
            .unwrap()
            .map(|f| f.unwrap().path())
            .collect::<Vec<_>>();

        if !paths.iter().any(|p| {
            [".clippy.toml", "clippy.toml"].contains(&p.file_name().unwrap().to_str().unwrap())
        }) {
            continue;
        }

        let clippytoml = paths
            .iter()
            .find(|p| {
                [".clippy.toml", "clippy.toml"].contains(&p.file_name().unwrap().to_str().unwrap())
            })
            .cloned()
            .expect("could not find clippy.toml");

        let cargotoml = paths
            .iter()
            .find(|p| p.file_name().unwrap().to_str().unwrap() == "Cargo.toml")
            .cloned()
            .expect("could not find Cargo.toml");

        let clippy_toml_text = std::fs::read_to_string(&clippytoml).unwrap();

        if !clippy_toml_text.contains("msrv") {
            continue;
        }

        let clippy_msrv = clippy_toml_text.lines().find(|line| line.contains("msrv"));

        let cargo_toml_text = std::fs::read_to_string(&cargotoml).unwrap();

        let rust_version = cargo_toml_text
            .lines()
            .find(|line| line.contains("rust-version"));

        match (clippy_msrv, rust_version) {
            (Some(clippy), Some(rust_min)) => {
                println!(
                    "{}: clippy-msrv: '{}', rust-version: '{}'",
                    p.file_name().unwrap().to_str().unwrap(),
                    clippy,
                    rust_min,
                )
            }
            (None, Some(_rust_min)) => {}
            (Some(clippy), None) => {
                println!(
                    "{}: clippy-msrv: '{}', rust-version: None",
                    p.file_name().unwrap().to_str().unwrap(),
                    clippy
                )
            }
            (None, None) => {}
        }
    }
}
