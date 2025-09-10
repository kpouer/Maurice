extern crate embed_resource;

fn main() {
    let _ = embed_resource::compile("windows/resources.rc", embed_resource::NONE);
}
