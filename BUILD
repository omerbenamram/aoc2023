load("@gazelle//:def.bzl", "gazelle")

gazelle(name = "gazelle")

# gazelle:rust_lockfile rust/Cargo.bazel.lock
# gazelle:rust_crates_prefix @crates//:
# gazelle:rust_mode generate_from_cargo

gazelle(
    name = "rust_gazelle",
    gazelle = "@gazelle_rust//:gazelle_bin",
)
