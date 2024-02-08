# Working with `up-rust` on Android

## Compiling for Android

We do not use `cargo` when building for Android, but instead use the `Soong`
build system. The dependencies for `up-rust` can be packaged for `Soong` and the
`up-rust` crate built by following the steps below.

1. Get the Android Open Source Project
2. `git clone` this project (`up-rust`) into `<aosp-root>/external/rust`
3. Run `android/generate_all_deps_soong.sh`
   * Will download and apply patches as necessary to all dependencies
4. Run `android/generate_soong.sh` to generate the `Android.bp` Soong
   build configuration for `up-rust`
5. Setup your build environment (execute the following from `<aosp-root>`:
   1. `$ source build/envsetup.sh`
   2. Choose which architecture to compile for
      3. `x86_64`: useful for running on AVD in Android Studio
          * `$ lunch aosp_x86_64-trunk_staging-eng`
      4. `ARMv8-A`: useful if you want to deploy to a phone
         * `$ lunch armv8-trunk_staging-eng`
6. Compile `up-rust`. From `<aosp-root>/external/rust/up-rust`
   1. `$ m libup_rust`
      * This will compile both the library crate `up-rust` as well
         as the unit tests

## Testing on Android

A quick sanity check is to copy the unit tests over to either your
  emulated device or hardware device and run it

* The unit test binary can be found under the out path
  * `x86_64`: `<aosp-root>/out/target/product/generic_x86_64/testcases/uprotocol-sdk_test_src_lib/<unit-test-binary>`
  * `ARMv8-A`: `<aosp-root>/out/target/product/armv8/testcases/uprotocol-sdk_test_src_lib/arm64/<unit-test-binary>`
* Recall that you'd do that like this:
  1. `~/Android/Sdk/platform-tools/adb push <unit-test-binary> /data/local/tmp`
  2. `~/Android/Sdk/platform-tools/adb shell`
  3. Now from within the shell on the emulated or hardware device
     1. `cd /data/local/tmp`
     2. `chmod +x <unit-test-binary>`

## Developing on Android

The development process differs a bit since we do not use `cargo`, but
the Android Open Source Project build tool `Soong` instead.

You can create your crate just as you normally would, but then you must 
use the `cargo_embargo` tool in order to:
1. Create a `cargo_embargo.json` file (or manually if you know what you're doing)
   * Used by the `cargo_embargo` tool to create the `Android.bp`
2. Create an `Android.bp` file
   * The Android Open Source Project `Soong` uses these to build

# Crates that must be built and have all dependencies available:

## Crates build in this order, considering dependencies, not alphabetical

* android_system_properties-0.1.5
  * Done
* autocfg-1.1.0
  * Done
* cc-1.0.83
  * Done
* doc-comment-0.3.3
  * Done
* equivalent-1.0.1
  * Done
* home-0.5.9
  * Done
* iana-time-zone-0.1.59
  * Done
* linux-raw-sys-0.4.13
  * Done
* match_cfg-0.1.0
  * Done
* mediatype-0.19.18
  * Done
* syn-1.0
  * Done
* snafu-derive-0.6.10
  * Done
* snafu-0.6.10
  * In-progress
* protoc-bin-vendored-win32-3.0.0
  * Done
* protoc-bin-vendored-linux-ppcle_64-3.0.0
  * Done
* protoc-bin-vendored-linux-x86_64-3.0.0
  * Done
* protoc-bin-vendored-linux-aarch_64-3.0.0
  * Done
* protoc-bin-vendored-macos-x86_64-3.0.0
  * Done
* protoc-bin-vendored-3.0.0: protoc-bin-vendored-linux-aarch_64 protoc-bin-vendored-linux-x86_32 protoc-bin-vendored-linux-x86_64 protoc-bin-vendored-macos-x86_64
  * Done
* adler-1.0.2
  * Done
* sct-0.7.1
  * Done
* miniz_oxide-0.7.1: adler
  * Done
* delegate-attr-0.2.9: basic-toml trybuild
  * Done
* hostname-0.3.1: matches pulldown-cmark semver-parser unicase version-sync version_check
  * Done
* cloudevents-sdk-0.7.0: actix-macros actix-rt assert-json-diff claim colored delegate-attr difference dtoa extend hostname is-terminal mime mockito proc-macro-error proc-macro-error-attr pulldown-cmark rmp rmp-serde rstest rustc_version rustix semver-parser serde_urlencoded signal-hook-registry snafu snafu-derive tower tower-layer tower-service unicase version-sync version_check
  * Done
* rustls-webpki
  * Done
* ring
  * Done
* webpki-roots-0.25.4: asn1-rs asn1-rs-derive asn1-rs-impl der-parser deranged encoding_rs h2 http-body httpdate hyper hyper-rustls hyper-tls ipnet mime native-tls num-conv oid-registry openssl-probe openssl-sys pem pkg-config powerfmt rcgen reqwest rusticata-macros rustls rustls-pemfile rustls-webpki sct serde_urlencoded sync_wrapper time time-core time-macros tokio-native-tls tokio-rustls tower-service try-lock vcpkg want x509-parser yasna
  * Done
* rustls
  * Done
* ureq-2.9.1: adler humantime is-terminal miniz_oxide rustix rustls rustls-pemfile rustls-webpki sct webpki-roots
  * Done
* protobuf-support
  * Done
* protobuf
  * Done
* uuid
  * Done
* android-tzdata
  * Done
* chrono
  * Done
* test-case-core-3.3.1
  * Done
* test-case-macros-3.3.1: test-case-core
  * Done
* test-case-3.3.1: console insta similar test-case-core test-case-macros
  * Done

## Currently manually copying Rust code generated from .proto to src/out

Need to figure out how to automate this using cargo_embargo. Couldn't get it to work yet.

# MUST_BUILD_CRATES_NOT_AVAILALBE.TXT

* rustix-0.38.31: adler dashmap errno miniz_oxide rustix serial_test serial_test_derive
