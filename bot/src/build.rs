use protobuf_codegen_pure::Customize;

fn main() {
    protobuf_codegen_pure::Codegen::new()
        .customize(Customize {
            ..Default::default()
        })
        .out_dir("src/core/protos")
        .input("../protos/Lakeshire.proto")
        .include("../protos")
        .run()
        .expect("protoc");
}
