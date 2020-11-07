fn main() {
    use std::env;

    let is_linux = env::var("CARGO_CFG_UNIX");
    if is_linux.is_ok() {
        println!("cargo:rustc-link-lib=CbcSolver");
        return;
    }

    let is_win = env::var("CARGO_CFG_WINDOWS");
    if is_win.is_ok() {
        println!("cargo:rustc-link-lib=libOsiClp");
        println!("cargo:rustc-link-lib=libOsiCbc");
        println!("cargo:rustc-link-lib=libCgl");
        println!("cargo:rustc-link-lib=libOsi");
        println!("cargo:rustc-link-lib=libCoinUtils");
        println!("cargo:rustc-link-lib=libClp");
        println!("cargo:rustc-link-lib=libClpSolver");
        println!("cargo:rustc-link-lib=libCbc");
        println!("cargo:rustc-link-lib=libCbcSolver");
        let path = env::var("PATH");
        if let Ok(ok_path) = path {
            let cbc_path = ok_path.split(";").find(|&s| s.contains("Cbc"));
            if let Some(final_path) = cbc_path {
                println!("cargo:rustc-link-search={}", final_path);
                println!(
                    "cargo:include={}",
                    std::path::Path::new(final_path)
                        .parent()
                        .unwrap()
                        .join("include")
                        .join("coin").to_str().unwrap()
                );
            }
        }
        return;
    }
}
