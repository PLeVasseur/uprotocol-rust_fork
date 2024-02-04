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
* http-body-0.4.6
* io-lifetimes-1.0.11
* ipnet-2.9.0
* openssl-probe-0.1.5
* pkg-config-0.3.29
* protoc-bin-vendored-linux-aarch_64-3.0.0
* protoc-bin-vendored-linux-ppcle_64-3.0.0
* protoc-bin-vendored-linux-x86_32-3.0.0
* protoc-bin-vendored-linux-x86_64-3.0.0
* protoc-bin-vendored-macos-x86_64-3.0.0
* protoc-bin-vendored-win32-3.0.0
* rustls-pemfile-1.0.4
* serde_urlencoded-0.7.1
* test-case-core-3.3.1
* try-lock-0.2.5
* value-bag-1.7.0
* waker-fn-1.1.1

# MUST_BUILD_CRATES_NOT_AVAILALBE.TXT

adler-1.0.2: autocfg
async-channel-2.1.1: concurrent-queue easy-parallel event-listener event-listener-strategy futures-lite parking
async-executor-1.8.0: async-channel async-io async-lock autocfg concurrent-queue easy-parallel event-listener event-listener-strategy futures-lite linux-raw-sys parking polling rustix waker-fn
async-global-executor-2.4.1: async-channel async-executor async-io async-lock atomic-waker autocfg blocking concurrent-queue doc-comment event-listener event-listener-strategy futures-lite linux-raw-sys parking piper polling rustix
async-io-2.3.1: async-channel async-io async-lock async-net atomic-waker autocfg blocking concurrent-queue event-listener event-listener-strategy futures-lite inotify inotify-sys io-lifetimes linux-raw-sys parking piper polling rustix signal-hook signal-hook-registry timerfd
async-lock-3.3.0: async-channel concurrent-queue event-listener event-listener-strategy futures-lite parking waker-fn
atomic-waker-1.1.2: autocfg
blocking-1.5.1: async-channel async-lock atomic-waker concurrent-queue event-listener event-listener-strategy futures-lite parking piper
cloudevents-sdk-0.7.0: actix-macros actix-rt assert-json-diff autocfg claim colored delegate-attr difference doc-comment dtoa equivalent extend hostname iana-time-zone is-terminal linux-raw-sys match_cfg mime mockito proc-macro-error proc-macro-error-attr pulldown-cmark rmp rmp-serde rstest rustc_version rustix semver-parser serde_urlencoded signal-hook-registry snafu snafu-derive tower tower-layer tower-service unicase version-sync version_check
concurrent-queue-2.4.0: autocfg easy-parallel
delegate-attr-0.2.9: basic-toml trybuild
encoding_rs-0.8.33: bincode
event-listener-4.0.3: anstyle autocfg clap_builder concurrent-queue futures-lite is-terminal linux-raw-sys parking rustix waker-fn
event-listener-strategy-0.4.0: concurrent-queue event-listener futures-lite parking
futures-lite-2.2.0: parking spin_on waker-fn
h2-0.3.24: autocfg cc equivalent rustls rustls-webpki sct tokio-rustls webpki-roots
hostname-0.3.1: match_cfg matches pulldown-cmark semver-parser unicase version-sync version_check
httpdate-1.0.3: anstyle autocfg clap_builder is-terminal linux-raw-sys rustix
hyper-0.14.28: http-body httpdate humantime ipnetwork matches pnet_base pnet_datalink pnet_sys pretty_env_logger quick-error spmc tower tower-layer tower-service try-lock want
hyper-tls-0.5.0: autocfg cc http-body httpdate hyper native-tls openssl-probe openssl-sys pkg-config tokio-native-tls tower-service try-lock want
kv-log-macro-1.0.7: async-log async-log-attributes backtrace backtrace-sys cc clicolors-control console femme termios
miniz_oxide-0.7.1: adler
openssl-sys-0.9.99: cc pkg-config vcpkg
parking-2.2.0: easy-parallel
piper-0.2.1: async-channel async-executor async-io async-lock atomic-waker autocfg concurrent-queue easy-parallel event-listener event-listener-strategy futures-lite io-lifetimes linux-raw-sys parking polling rustix waker-fn
polling-3.3.2: easy-parallel linux-raw-sys rustix signal-hook signal-hook-registry
protoc-bin-vendored-3.0.0: protoc-bin-vendored-linux-aarch_64 protoc-bin-vendored-linux-ppcle_64 protoc-bin-vendored-linux-x86_32 protoc-bin-vendored-linux-x86_64 protoc-bin-vendored-macos-x86_64 protoc-bin-vendored-win32
reqwest-0.11.24: adler32 alloc-no-stdlib alloc-stdlib autocfg brotli brotli-decompressor cc doc-comment encoding_rs equivalent h2 http-body httpdate humantime hyper hyper-tls ipnet is-terminal libflate libflate_lz77 linux-raw-sys mime native-tls openssl-probe openssl-sys pkg-config rle-decode-fast rustix rustls-pemfile serde_urlencoded sync_wrapper tokio-native-tls tower-service try-lock vcpkg want
rustix-0.38.31: adler autocfg dashmap errno linux-raw-sys miniz_oxide rustix serial_test serial_test_derive
sct-0.7.1: cc
snafu-0.6.10: doc-comment snafu-derive
sync_wrapper-0.1.2: autocfg
test-case-3.3.1: console insta similar test-case-core test-case-macros
test-case-macros-3.3.1: test-case-core
tower-service-0.3.2: autocfg tower-layer
ureq-2.9.1: adler cc humantime is-terminal linux-raw-sys miniz_oxide rustix rustls rustls-pemfile rustls-webpki sct webpki-roots
vcpkg-0.2.15: tempdir
want-0.3.1: autocfg futures-channel-preview futures-core-preview futures-util-preview tokio-executor tokio-sync try-lock
webpki-roots-0.25.4: asn1-rs asn1-rs-derive asn1-rs-impl autocfg cc der-parser deranged encoding_rs equivalent h2 http-body httpdate hyper hyper-rustls hyper-tls iana-time-zone ipnet mime native-tls num-conv oid-registry openssl-probe openssl-sys pem pkg-config powerfmt rcgen reqwest rusticata-macros rustls rustls-pemfile rustls-webpki sct serde_urlencoded sync_wrapper time time-core time-macros tokio-native-tls tokio-rustls tower-service try-lock vcpkg want x509-parser yasna
