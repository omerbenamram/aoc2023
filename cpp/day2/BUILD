cc_library(
    name = "day2_lib",
    srcs = ["libday2.cc"],
    hdrs = ["libday2.h"],
)

cc_binary(
    name = "day2",
    srcs = ["day2.cc"],
    deps = [
        ":day2_lib",
    ],
)

cc_test(
    name = "test",
    size = "small",
    srcs = ["day2_test.cc"],
    deps = [
        ":day2_lib",
        "@googletest//:gtest_main",
    ],
)
