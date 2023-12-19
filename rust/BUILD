load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_library", "rust_test")

rust_library(
    name = "aoc2023",
    srcs = ["src/lib.rs"],
    compile_data = [":Cargo.toml"],
    edition = "2021",
    visibility = ["//visibility:public"],
    deps = [
        "@crates//:env_logger",
        "@crates//:grid",
        "@crates//:log",
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
        "@crates//:anyhow",
        "@crates//:itertools",
        "@crates//:once_cell",  # keep
        "@crates//:regex",  # keep
    ],
)

rust_binary(
    name = "day3",
    srcs = ["src/bin/day3.rs"],
    compile_data = [":Cargo.toml"],
    visibility = ["//visibility:public"],
    deps = [
        ":aoc2023",
        "@crates//:anyhow",
        "@crates//:itertools",
        "@crates//:once_cell",  # keep
        "@crates//:regex",  # keep
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
        "@crates//:anyhow",
        "@crates//:itertools",
        "@crates//:once_cell",  # keep
        "@crates//:regex",  # keep
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
        "@crates//:anyhow",
        "@crates//:itertools",
        "@crates//:once_cell",  # keep
        "@crates//:regex",  # keep
    ],
)

rust_binary(
    name = "day8",
    srcs = ["src/bin/day8.rs"],
    compile_data = [":Cargo.toml"],
    visibility = ["//visibility:public"],
    deps = [
        ":aoc2023",
        "@crates//:anyhow",
        "@crates//:maplit",
        "@crates//:once_cell",  # keep
        "@crates//:regex",  # keep
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

rust_binary(
    name = "day10",
    srcs = ["src/bin/day10.rs"],
    compile_data = [":Cargo.toml"],
    visibility = ["//visibility:public"],
    deps = [
        "@crates//:anyhow",
        "@crates//:geo",
        "@crates//:itertools",
        "@crates//:petgraph",
    ],
)

rust_binary(
    name = "day13",
    srcs = ["src/bin/day13.rs"],
    compile_data = [":Cargo.toml"],
    visibility = ["//visibility:public"],
    deps = [
        "@crates//:anyhow",
        "@crates//:ndarray",
    ],
)

rust_binary(
    name = "day14",
    srcs = ["src/bin/day14.rs"],
    compile_data = [":Cargo.toml"],
    visibility = ["//visibility:public"],
    deps = [
        "@crates//:anyhow",
        "@crates//:grid",
    ],
)

rust_binary(
    name = "day15",
    srcs = ["src/bin/day15.rs"],
    compile_data = [":Cargo.toml"],
    visibility = ["//visibility:public"],
    deps = ["@crates//:anyhow"],
)

rust_binary(
    name = "day16",
    srcs = ["src/bin/day16.rs"],
    compile_data = [":Cargo.toml"],
    visibility = ["//visibility:public"],
    deps = [
        ":aoc2023",
        "@crates//:anyhow",
        "@crates//:grid",
        "@crates//:rayon",
    ],
)

rust_binary(
    name = "day17",
    srcs = ["src/bin/day17.rs"],
    compile_data = [":Cargo.toml"],
    visibility = ["//visibility:public"],
    deps = [
        ":aoc2023",
        "@crates//:anyhow",
        "@crates//:env_logger",
        "@crates//:grid",
        "@crates//:log",
        "@crates//:pathfinding",
    ],
)

rust_binary(
    name = "day18",
    srcs = ["src/bin/day18.rs"],
    compile_data = [":Cargo.toml"],
    visibility = ["//visibility:public"],
    deps = [
        ":aoc2023",
        "@crates//:once_cell",  # keep
        "@crates//:regex",  # keep
        "@crates//:anyhow",
    ],
)

rust_binary(
    name = "day19",
    srcs = ["src/bin/day19.rs"],
    compile_data = [":Cargo.toml"],
    visibility = ["//visibility:public"],
    deps = [
        ":aoc2023",
        "@crates//:anyhow",
    ],
)

rust_test(
    name = "day10_test",
    compile_data = [":Cargo.toml"],
    crate = ":day10",
    deps = ["@crates//:textwrap"],
)

rust_test(
    name = "day13_test",
    compile_data = [":Cargo.toml"],
    crate = ":day13",
    deps = ["@crates//:textwrap"],
)

rust_test(
    name = "day14_test",
    compile_data = [":Cargo.toml"],
    crate = ":day14",
    deps = ["@crates//:textwrap"],
)

rust_test(
    name = "day15_test",
    compile_data = [":Cargo.toml"],
    crate = ":day15",
)

rust_test(
    name = "day16_test",
    compile_data = [":Cargo.toml"],
    crate = ":day16",
)

rust_test(
    name = "day17_test",
    compile_data = [":Cargo.toml"],
    crate = ":day17",
)

rust_test(
    name = "day18_test",
    compile_data = [":Cargo.toml"],
    crate = ":day18",
    deps = ["@crates//:textwrap"],
)

rust_test(
    name = "day19_test",
    compile_data = [":Cargo.toml"],
    crate = ":day19",
    deps = ["@crates//:textwrap"],
)
