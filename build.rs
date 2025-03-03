fn main() {
    embed_resource::compile("resource.rc");
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon("favicon.ico");
        res.compile().expect("Erreur lors de la compilation des ressources");
    }
}
