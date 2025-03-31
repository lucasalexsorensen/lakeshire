fn main() {
    prost_build::compile_protos(&["../../protos/Lakeshire.proto"], &["../../protos"]).unwrap();
    tauri_build::build()
}
