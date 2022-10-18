update-contract:
	echo "Updating the wit contracts"
	wit-bindgen guest rust --import wits/host.wit && mv bindings.rs src/guest_imports.rs
	wit-bindgen guest rust --export wits/guest_backend.wit && mv bindings.rs src/guest_exports.rs
	wit-bindgen host wasmtime-rust --import wits/guest_backend.wit && mv bindings.rs crates/fpm-utils-host/src/backend_host_import.rs
	wit-bindgen host wasmtime-rust --export wits/host.wit && mv bindings.rs crates/fpm-utils-host/src/backend_host_export.rs