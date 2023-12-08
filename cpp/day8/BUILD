cc_library(
    name = "day8_lib",
    srcs = ["libday8.cc"],
    hdrs = ["libday8.h"],
)

cc_binary(
    name = "day8",
    srcs = ["day8.cc"],
    deps = [
        ":day8_lib",
    ],
)

cc_test(
    name = "test",
    size = "small",
    srcs = ["day8_test.cc"],
    deps = [
        ":day8_lib",
        "@googletest//:gtest_main",
    ],
)
