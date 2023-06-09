// generated by `sqlx migrate build-script`
fn main() {
    if let Ok(profile) = std::env::var("PROFILE") {
        println!("cargo:rustc-cfg=build={:?}", profile);
    }

    // trigger recompilation when a new migration is added
    println!("cargo:rerun-if-changed=migrations");
}
