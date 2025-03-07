fn main() {
    embed_resource::compile("resource.rc", Vec::<&str>::new());
    /*
    // Fix error "linking with link.exe failed: exit code 1123"
    // ressources duplicate from two different place

    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon("favicon.ico");
        res.compile().expect("Erreur lors de la compilation des ressources");
    }
     */
}