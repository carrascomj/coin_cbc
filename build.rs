fn main() {
    use std::env;

    let is_linux = env::var("CARGO_CFG_UNIX");
    if is_linux.is_ok() {
        println!("cargo:rustc-link-lib=CbcSolver");
        return;
    }

    let is_win = env::var("CARGO_CFG_WINDOWS");
    if is_win.is_ok() {
        println!("cargo:rustc-link-lib=libOsi");
        let path = env::var("PATH");
        if let Ok(ok_path) = path {
            let osi_path = ok_path.split(";").find(|&s| s.contains("Osi"));
            if let Some(final_path) = osi_path {
                println!("cargo:rustc-link-search={}", final_path);
            }
        }
        println!("cargo:rustc-link-lib=libCgl");
        let path = env::var("PATH");
        if let Ok(ok_path) = path {
            let cgl_path = ok_path.split(";").find(|&s| s.contains("Cgl"));
            if let Some(final_path) = cgl_path {
                println!("cargo:rustc-link-search={}", final_path);
            }
        }
        println!("cargo:rustc-link-lib=libCoinUtils");
        let path = env::var("PATH");
        if let Ok(ok_path) = path {
            let utils_path = ok_path.split(";").find(|&s| s.contains("CoinUtils"));
            if let Some(final_path) = utils_path {
                println!("cargo:rustc-link-search={}", final_path);
            }
        }
        println!("cargo:rustc-link-lib=libCbcSolver");
        let path = env::var("PATH");
        if let Ok(ok_path) = path {
            let cbc_path = ok_path.split(";").find(|&s| s.contains("Cbc"));
            if let Some(final_path) = cbc_path {
                println!("cargo:rustc-link-search={}", final_path);
            }
        }
        println!("cargo:rustc-link-lib=libClpSolver");
        let path = env::var("PATH");
        if let Ok(ok_path) = path {
            let clp_path = ok_path.split(";").find(|&s| s.contains("Clp"));
            if let Some(final_path) = clp_path {
                println!("cargo:rustc-link-search={}", final_path);
            }
        }
        println!("cargo:rustc-link-lib=libOsiCbc");
        let path = env::var("PATH");
        if let Ok(ok_path) = path {
            let osi_cbc_path = ok_path.split(";").find(|&s| s.contains("OsiCbc"));
            if let Some(final_path) = osi_cbc_path {
                println!("cargo:rustc-link-search={}", final_path);
            }
        }
        println!("cargo:rustc-link-lib=libOsiClp");
        let path = env::var("PATH");
        if let Ok(ok_path) = path {
            let osi_clp_path = ok_path.split(";").find(|&s| s.contains("OsiClp"));
            if let Some(final_path) = osi_clp_path {
                println!("cargo:rustc-link-search={}", final_path);
            }
        }
        return;
    }
}
