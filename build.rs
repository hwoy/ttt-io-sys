use std::path::Path;

fn main() {
    {
        let bindings = bindgen::Builder::default()
            .header("u-tic-tac-toe/ttt_io.h")
			.newtype_enum("ox_gameid")
            .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
            .generate()
            .expect("Unable to generate bindings");

        bindings
            .write_to_file("src/ttt-io.rs")
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
