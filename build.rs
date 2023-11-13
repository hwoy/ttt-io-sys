use std::path::Path;

fn main() {
    {
        let emsdk_include_dir_var = "EMSDK_INCLUDE_DIR";

        let bindings = bindgen::Builder::default()
            .header("u-tic-tac-toe/ttt_io.h")
            .allowlist_file("u-tic-tac-toe/ttt_io.h")
            .clang_arg(if let Ok(val) = std::env::var(emsdk_include_dir_var) {
                let mut set_arg = "-I".to_string();
                set_arg.push_str(&val);
                set_arg
            } else {
                "".to_string()
            })
            .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
            .generate()
            .expect("Unable to generate bindings");

        bindings
            .write_to_file("src/ttt_io_sys.rs")
            .expect("Couldn't write bindings!");
    }

    {
        let lib = "ttt_io";
        let path = ["u-tic-tac-toe/ttt_io.c"];

        if path.iter().all(|p| Path::new(p).exists()) {
            let mut builder = cc::Build::new();
            for p in path.iter() {
                builder.file(p);
            }
            builder.extra_warnings(true).compile(lib);
        }
    }
}
