fn compile_pb() {
    let mut codegen = protobuf_codegen_pure::Codegen::new();
    codegen.out_dir("../remote-file/src/protos")
        .include("src/protos/src/")
        .inputs(&["src/protos/src/base.proto"])
        .run()
        .expect("compile protoc failed.");
}

fn main() {
    compile_pb();
    println!("compile finish!");
}
