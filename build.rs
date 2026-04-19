fn main() {
    let output = std::process::Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .unwrap();

    let hash = String::from_utf8(output.stdout).unwrap();
    let hash = hash.trim();
    let short = &hash[..7];

    println!("cargo:rustc-env=GIT_COMMIT_HASH={hash}");
    println!("cargo:rustc-env=GIT_COMMIT_SHORT={short}");
    println!("cargo:rerun-if-changed=.git/HEAD");
}
