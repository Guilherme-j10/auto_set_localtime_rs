extern crate embed_resource;

fn main() {
    println!("cargo:rerun-if-changed=localtime-manifest.rc");
    embed_resource::compile("localtime-manifest.rc", embed_resource::NONE);
}