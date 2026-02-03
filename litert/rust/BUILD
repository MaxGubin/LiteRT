# Copyright 2025 Google LLC.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#      http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

load("//third_party/bazel_rules/rules_cc/cc:cc_library.bzl", "cc_library")
load("//third_party/bazel_rules/rules_rust/bindgen:defs.bzl", "rust_bindgen")
load("//third_party/bazel_rules/rules_rust/rust:defs.bzl", "rust_binary", "rust_doc", "rust_library", "rust_test")

package(
    default_applicable_licenses = ["//third_party/odml:license"],  # copybara:comment
)

cc_library(
    name = "c_wrapper",
    hdrs = ["wrapper.h"],
    deps = [
        "//third_party/odml/litert/litert/c:litert_common",
        "//third_party/odml/litert/litert/c:litert_compiled_model",
        "//third_party/odml/litert/litert/c:litert_environment",
        "//third_party/odml/litert/litert/c:litert_environment_options",
        "//third_party/odml/litert/litert/c:litert_event",
        "//third_party/odml/litert/litert/c:litert_metrics",
        "//third_party/odml/litert/litert/c:litert_model",
        "//third_party/odml/litert/litert/c:litert_op_options",
        "//third_party/odml/litert/litert/c:litert_opaque_options",
        "//third_party/odml/litert/litert/c:litert_options",
        "//third_party/odml/litert/litert/c:litert_tensor_buffer",
        "//third_party/odml/litert/litert/c/internal:litert_logging",
    ],
)

rust_bindgen(
    name = "bindgen",
    bindgen_flags = [
        "--no-layout-tests",
        "--with-derive-default",
    ],
    cc_lib = ":c_wrapper",
    clang_flags = [
        "-D__XMMINTRIN_H",
        "-D__EMMINTRIN_H",
    ],
    header = "wrapper.h",
)

rust_library(
    name = "litert",
    srcs = [
        "bindgen.rs",
        "src/bindings.rs",
        "src/compiled_model.rs",
        "src/environment.rs",
        "src/error.rs",
        "src/helper_funs.rs",
        "src/lib.rs",
        "src/macros.rs",
        "src/model.rs",
        "src/tensor_buffer.rs",
    ],
    crate_root = "src/lib.rs",
    edition = "2021",
    rustc_env = {"BINDGEN_RS_FILE": "../bindgen.rs"},
    rustc_flags = ["--cfg=bindgen_rs_file"],
    deps = [
        ":bindgen",
    ],
)

rust_test(
    name = "litert_test",
    crate = ":litert",
    rustc_env = {"BINDGEN_RS_FILE": "../bindgen.rs"},
    rustc_flags = ["--cfg=bindgen_rs_file"],
    deps = [
        ":litert",
        "//third_party/odml/litert/litert/c:litert_runtime_c_api_shared_lib",
    ],
)

rust_doc(
    name = "litert_doc",
    crate = ":litert",
    tags = [
        "manual",
    ],
)

rust_binary(
    name = "example_segmentation",
    srcs = ["example/segmentation_main.rs"],
    crate_features = ["png"],
    edition = "2021",
    rustc_flags = [
    ],
    deps = [
        ":litert",
        "//third_party/odml/litert/litert/c:litert_runtime_c_api_shared_lib",
        "//third_party/rust/clap/v4:clap",
        "//third_party/rust/image/v0_25:image",
    ],
)
