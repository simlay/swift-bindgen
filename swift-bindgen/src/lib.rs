//! # Swift Bindgen
//!
//! Bindings generator for two-way bridging of Rust/Swift types.
//!
//! ## Contribute
//!
//! This is a work-in-progress by [Nikolai Vazquez](https://twitter.com/nikolaivazquez).
//! If you would like to get involved,
//! <a href="mailto:hello@nikolaivazquez.com?subject=I want to help with swift-bindgen&body=Hi Nikolai,%0A%0AMy name is YOUR NAME and I want to get involved with swift-bindgen by...">reach out</a>!
//!
//! If this project is useful to you, please support it by
//! [sponsoring on GitHub](https://github.com/sponsors/nvzqz) or
//! [donating directly](https://www.paypal.me/nvzqz)!

#![warn(missing_docs)]
#![allow(clippy::module_inception)]

/* EXAMPLE TBD FILE SNIPPET:
tbd-version:     4
targets:         [ armv7-ios, armv7s-ios, arm64-ios, arm64e-ios ]
install-name:    '/usr/lib/swift/libswiftCore.dylib'
current-version: 5.9.2
swift-abi-version: 7
exports:
  - targets:         [ armv7-ios, armv7s-ios ]
    symbols:         [ '_$sSi12SIMD2StorageV6_valueBi32_Bv2_vM', '_$sSi12SIMD2StorageV6_valueBi32_Bv2_vg',
    ]
    objc-classes:    [ _TtCs19__SwiftNativeNSData, __SwiftNativeNSDataBase, __SwiftNativeNSIndexSetBase ]
*/

use serde::Deserialize;
#[derive(Deserialize)]
struct TBD {
    #[serde(rename = "tbd-version")]
    tbd_version: u32,
    targets: Vec<String>,
    #[serde(rename = "install-name")]
    install_name: String,
    #[serde(rename = "current-version")]
    current_version: String,
    #[serde(rename = "swift-abi-version")]
    swift_abi_version: u32,
    exports: Vec<TBDExports>,
}
#[derive(Deserialize)]
struct TBDExports {
    targets: Vec<String>,
    symbols: Vec<String>,
    #[serde(rename = "objc-classes")]
    objc_classes: Option<Vec<String>>
}
use std::fs;
use std::error::Error;
#[test]
fn test_parse_tbd() {
    let libswiftCorePath = "/Applications/Xcode.app/Contents/Developer/Platforms/iPhoneOS.platform/Developer/SDKs/iPhoneOS17.2.sdk/usr/lib/swift/libswiftCore.tbd";
    let yaml: String = fs::read_to_string(libswiftCorePath).expect("Failed to read file");

    let deserialized_point: TBD = serde_yaml::from_str(&yaml).expect("Failed to deserialize yaml");
}
