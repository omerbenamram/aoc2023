load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

GAZELLE_RUST_COMMIT = "00e88bceaa1a1c35d9c3019f65f3e20459fafe33"
GAZELLE_RUST_SHA256 = "312f581a19cc4828c93df5a30c20e3f34a67a3e21853c70f64e7a9e74e670230"

http_archive(
    name = "gazelle_rust",
    sha256 = GAZELLE_RUST_SHA256,
    strip_prefix = "gazelle_rust-{}".format(GAZELLE_RUST_COMMIT),
    url = "https://github.com/Calsign/gazelle_rust/archive/{}.zip".format(GAZELLE_RUST_COMMIT),
)

http_archive(
    name = "rules_rust",
    # NOTE: This patch is currently necessary for gazelle_rust to parse crate_universe lockfiles.
    patches = ["@gazelle_rust//patches:rules_rust.patch"],
    integrity = "sha256-dRdyJjgLdxvjbX78U42oQsQz8UzWw212YJdu+1Pe/oY=",
    urls = ["https://github.com/bazelbuild/rules_rust/releases/download/0.34.1/rules_rust-v0.34.1.tar.gz"],
)

load("@rules_rust//rust:repositories.bzl", "rules_rust_dependencies", "rust_register_toolchains")

rules_rust_dependencies()

rust_register_toolchains(
    edition = "2021",
    versions = ["1.74.0"],
)

load("@rules_rust//crate_universe:defs.bzl", "crate", "crates_repository")

# Example of using crate_universe. For gazelle_rust to work correctly with crate_universe
# dependencies, this must be paired with two gazelle directives; see BUILD.bazel.
crates_repository(
    name = "crates",
    cargo_lockfile = "//rust:Cargo.lock",
    lockfile = "//rust:Cargo.bazel.lock",
    manifests = [
        "//rust:Cargo.toml"
    ]
)

load("@crates//:defs.bzl", "crate_repositories")

crate_repositories()

# Load gazelle_rust transitive dependencies (includes gazelle). You can also load gazelle yourself,
# before these macros.

load("@gazelle_rust//:deps1.bzl", "gazelle_rust_dependencies1")

gazelle_rust_dependencies1()

load("@gazelle_rust//:deps2.bzl", "gazelle_rust_dependencies2")

gazelle_rust_dependencies2()