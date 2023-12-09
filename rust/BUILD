load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_library", "rust_test")

rust_library(
    name = "aoc2023",
    srcs = ["src/lib.rs"],
    compile_data = [":Cargo.toml"],
    edition = "2021",
    visibility = ["//visibility:public"],
    deps = [
        "@crates//:once_cell",  # keep
        "@crates//:regex",  # keep
    ],
)

rust_binary(
    name = "day1",
    srcs = ["src/bin/day1.rs"],
    compile_data = [":Cargo.toml"],
    visibility = ["//visibility:public"],
    deps = [
        "@crates//:anyhow",
        "@crates//:itertools",
        "@crates//:regex",
    ],
)

rust_binary(
    name = "day2",
    srcs = ["src/bin/day2.rs"],
    compile_data = [":Cargo.toml"],
    visibility = ["//visibility:public"],
    deps = [
        ":aoc2023",
        "@crates//:once_cell",  # keep
        "@crates//:regex",  # keep

        "@crates//:anyhow",
        "@crates//:itertools"
    ],
)

rust_binary(
    name = "day3",
    srcs = ["src/bin/day3.rs"],
    compile_data = [":Cargo.toml"],
    visibility = ["//visibility:public"],
    deps = [
        ":aoc2023",
        "@crates//:once_cell",  # keep
        "@crates//:regex",  # keep
        
        "@crates//:anyhow",
        "@crates//:itertools",
    ],
)

rust_binary(
    name = "day4",
    srcs = ["src/bin/day4.rs"],
    compile_data = [":Cargo.toml"],
    visibility = ["//visibility:public"],
    deps = ["@crates//:anyhow"],
)

rust_binary(
    name = "day5",
    srcs = ["src/bin/day5.rs"],
    compile_data = [":Cargo.toml"],
    visibility = ["//visibility:public"],
    deps = [
        "@crates//:anyhow",
        "@crates//:itertools",
        "@crates//:log",
    ],
)

rust_binary(
    name = "day6",
    srcs = ["src/bin/day6.rs"],
    compile_data = [":Cargo.toml"],
    visibility = ["//visibility:public"],
    deps = [
        ":aoc2023",
        "@crates//:once_cell",  # keep
        "@crates//:regex",  # keep
        
        "@crates//:anyhow",
        "@crates//:itertools",
        "@crates//:roots",
    ],
)

rust_binary(
    name = "day7",
    srcs = ["src/bin/day7.rs"],
    compile_data = [":Cargo.toml"],
    visibility = ["//visibility:public"],
    deps = [
        ":aoc2023",
        "@crates//:once_cell",  # keep
        "@crates//:regex",  # keep

        "@crates//:anyhow",
        "@crates//:itertools",
    ],
)

rust_binary(
    name = "day8",
    srcs = ["src/bin/day8.rs"],
    compile_data = [":Cargo.toml"],
    visibility = ["//visibility:public"],
    deps = [
        ":aoc2023",
        "@crates//:once_cell",  # keep
        "@crates//:regex",  # keep
        
        "@crates//:anyhow",
        "@crates//:maplit",
    ],
)

rust_binary(
    name = "day9",
    srcs = ["src/bin/day9.rs"],
    compile_data = [":Cargo.toml"],
    visibility = ["//visibility:public"],
    deps = ["@crates//:anyhow"],
)

rust_test(
    name = "day1_test",
    compile_data = [":Cargo.toml"],
    crate = ":day1",
)

rust_test(
    name = "day2_test",
    compile_data = [":Cargo.toml"],
    crate = ":day2",
)

rust_test(
    name = "day3_test",
    compile_data = [":Cargo.toml"],
    crate = ":day3",
    deps = ["@crates//:textwrap"],
)

rust_test(
    name = "day4_test",
    compile_data = [":Cargo.toml"],
    crate = ":day4",
    deps = ["@crates//:textwrap"],
)

rust_test(
    name = "day5_test",
    compile_data = [":Cargo.toml"],
    crate = ":day5",
    deps = [
        "@crates//:env_logger",
        "@crates//:maplit",
        "@crates//:textwrap",
    ],
)

rust_test(
    name = "day6_test",
    compile_data = [":Cargo.toml"],
    crate = ":day6",
    deps = ["@crates//:textwrap"],
)

rust_test(
    name = "day7_test",
    compile_data = [":Cargo.toml"],
    crate = ":day7",
    deps = ["@crates//:textwrap"],
)

rust_test(
    name = "day8_test",
    compile_data = [":Cargo.toml"],
    crate = ":day8",
    deps = ["@crates//:textwrap"],
)

rust_test(
    name = "day9_test",
    compile_data = [":Cargo.toml"],
    crate = ":day9",
    deps = ["@crates//:textwrap"],
)
