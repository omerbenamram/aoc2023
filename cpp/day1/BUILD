cc_library(
    name = "day1_lib",
    srcs = ["libday1.cc"],
    hdrs = ["libday1.h"],
)

cc_binary(
    name = "day1",
    srcs = ["day1.cc"],
    deps = [
        ":day1_lib",
    ],
)

cc_test(
    name = "test",
    size = "small",
    srcs = ["day1_test.cc"],
    deps = [
        ":day1_lib",
        "@googletest//:gtest_main",
    ],
)
