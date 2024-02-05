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
* test-case-core-3.3.1

# MUST_BUILD_CRATES_NOT_AVAILALBE.TXT

* cloudevents-sdk-0.7.0: actix-macros actix-rt assert-json-diff claim colored delegate-attr difference dtoa extend hostname is-terminal mime mockito proc-macro-error proc-macro-error-attr pulldown-cmark rmp rmp-serde rstest rustc_version rustix semver-parser serde_urlencoded signal-hook-registry snafu snafu-derive tower tower-layer tower-service unicase version-sync version_check
* delegate-attr-0.2.9: basic-toml trybuild
* hostname-0.3.1: matches pulldown-cmark semver-parser unicase version-sync version_check
* rustix-0.38.31: adler dashmap errno miniz_oxide rustix serial_test serial_test_derive
* test-case-3.3.1: console insta similar test-case-core test-case-macros
* test-case-macros-3.3.1: test-case-core
* ureq-2.9.1: adler humantime is-terminal miniz_oxide rustix rustls rustls-pemfile rustls-webpki sct webpki-roots
webpki-roots-0.25.4: asn1-rs asn1-rs-derive asn1-rs-impl der-parser deranged encoding_rs h2 http-body httpdate hyper hyper-rustls hyper-tls ipnet mime native-tls num-conv oid-registry openssl-probe openssl-sys pem pkg-config powerfmt rcgen reqwest rusticata-macros rustls rustls-pemfile rustls-webpki sct serde_urlencoded sync_wrapper time time-core time-macros tokio-native-tls tokio-rustls tower-service try-lock vcpkg want x509-parser yasna
