default: gen lint

gen:
    flutter pub get
   
    cargo xcode --manifest-path=native/Cargo.toml --output-dir=native
    flutter_rust_bridge_codegen \
        --rust-input native/src/api.rs \
        --dart-output lib/native/bridge_generated.dart \
        --c-output ios/Runner/bridge_generated.h \
        --dart-decl-output lib/native/bridge_definitions.dart \
        --wasm
    cp ios/Runner/bridge_generated.h macos/Runner/bridge_generated.h

    
    cargo xcode --manifest-path=rust-security/Cargo.toml --output-dir=rust-security
    flutter_rust_bridge_codegen \
        --rust-input rust-security/src/api.rs \
        --dart-output lib/rust_security/bridge_generated.dart \
        --c-output ios/Runner/rust_security_bridge_generated.h \
        --dart-decl-output lib/rust_security/bridge_definitions.dart \
        --wasm
    cp ios/Runner/rust_security_bridge_generated.h macos/Runner/rust_security_bridge_generated.h

    
lint:
    cp rustfmt.toml native/rustfmt.toml
    cd native && cargo fmt
    dart format .

    cp rustfmt.toml rust-security/rustfmt.toml
    cd rust-security && cargo fmt
    dart format .

clean:
    flutter clean
    cd native && cargo clean
    cd rust-security && cargo clean
    
serve *args='':
    flutter pub run flutter_rust_bridge:serve {{args}}

# vim:expandtab:sw=4:ts=4
