fn main() {
    ::capnpc::CompilerCommand::new()
        .src_prefix("schema/")
        .file("schema/simstate.capnp")
        .run()
        .expect("compiling schema");
}
