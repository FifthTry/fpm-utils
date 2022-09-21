#[allow(clippy::all)]
pub mod guest_backend {
    #[allow(unused_imports)]
    use wit_bindgen_host_wasmtime_rust::{anyhow, wasmtime};
    #[derive(Clone)]
    pub struct Httprequest<'a> {
        pub path: &'a str,
        pub headers: &'a [(&'a str, &'a str)],
        pub querystring: &'a str,
        pub method: &'a str,
    }
    impl<'a> core::fmt::Debug for Httprequest<'a> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("Httprequest")
                .field("path", &self.path)
                .field("headers", &self.headers)
                .field("querystring", &self.querystring)
                .field("method", &self.method)
                .finish()
        }
    }

    /// Auxiliary data associated with the wasm exports.
    ///
    /// This is required to be stored within the data of a
    /// `Store<T>` itself so lifting/lowering state can be managed
    /// when translating between the host and wasm.
    #[derive(Default)]
    pub struct GuestBackendData {}
    #[allow(dead_code)]
    pub struct GuestBackend<T> {
        get_state: Box<dyn Fn(&mut T) -> &mut GuestBackendData + Send + Sync>,
        cabi_realloc: wasmtime::TypedFunc<(i32, i32, i32, i32), i32>,
        canonical_abi_free: wasmtime::TypedFunc<(i32, i32, i32), ()>,
        handlerequest: wasmtime::TypedFunc<(i32, i32, i32, i32, i32, i32, i32, i32), (i32,)>,
        memory: wasmtime::Memory,
    }
    impl<T> GuestBackend<T> {
        #[allow(unused_variables)]

        /// Adds any intrinsics, if necessary for this exported wasm
        /// functionality to the `linker` provided.
        ///
        /// The `get_state` closure is required to access the
        /// auxiliary data necessary for these wasm exports from
        /// the general store's state.
        pub fn add_to_linker(
            linker: &mut wasmtime::Linker<T>,
            get_state: impl Fn(&mut T) -> &mut GuestBackendData + Send + Sync + Copy + 'static,
        ) -> anyhow::Result<()> {
            Ok(())
        }

        /// Instantiates the provided `module` using the specified
        /// parameters, wrapping up the result in a structure that
        /// translates between wasm and the host.
        ///
        /// The `linker` provided will have intrinsics added to it
        /// automatically, so it's not necessary to call
        /// `add_to_linker` beforehand. This function will
        /// instantiate the `module` otherwise using `linker`, and
        /// both an instance of this structure and the underlying
        /// `wasmtime::Instance` will be returned.
        ///
        /// The `get_state` parameter is used to access the
        /// auxiliary state necessary for these wasm exports from
        /// the general store state `T`.
        pub fn instantiate(
            mut store: impl wasmtime::AsContextMut<Data = T>,
            module: &wasmtime::Module,
            linker: &mut wasmtime::Linker<T>,
            get_state: impl Fn(&mut T) -> &mut GuestBackendData + Send + Sync + Copy + 'static,
        ) -> anyhow::Result<(Self, wasmtime::Instance)> {
            Self::add_to_linker(linker, get_state)?;
            let instance = linker.instantiate(&mut store, module)?;
            Ok((Self::new(store, &instance, get_state)?, instance))
        }

        /// Low-level creation wrapper for wrapping up the exports
        /// of the `instance` provided in this structure of wasm
        /// exports.
        ///
        /// This function will extract exports from the `instance`
        /// defined within `store` and wrap them all up in the
        /// returned structure which can be used to interact with
        /// the wasm module.
        pub fn new(
            mut store: impl wasmtime::AsContextMut<Data = T>,
            instance: &wasmtime::Instance,
            get_state: impl Fn(&mut T) -> &mut GuestBackendData + Send + Sync + Copy + 'static,
        ) -> anyhow::Result<Self> {
            let mut store = store.as_context_mut();
            let cabi_realloc = instance
                .get_typed_func::<(i32, i32, i32, i32), i32, _>(&mut store, "cabi_realloc")?;
            let canonical_abi_free = instance
                .get_typed_func::<(i32, i32, i32), (), _>(&mut store, "canonical_abi_free")?;
            let handlerequest= instance.get_typed_func::<(i32,i32,i32,i32,i32,i32,i32,i32,), (i32,), _>(&mut store, "handlerequest: func(a: record { path: string, headers: list<tuple<string, string>>, querystring: string, method: string }) -> string")?;
            let memory = instance
                .get_memory(&mut store, "memory")
                .ok_or_else(|| anyhow::anyhow!("`memory` export not a memory"))?;
            Ok(GuestBackend {
                cabi_realloc,
                canonical_abi_free,
                handlerequest,
                memory,
                get_state: Box::new(get_state),
            })
        }
        pub fn handlerequest(
            &self,
            mut caller: impl wasmtime::AsContextMut<Data = T>,
            a: Httprequest<'_>,
        ) -> Result<String, wasmtime::Trap> {
            let func_canonical_abi_free = &self.canonical_abi_free;
            let func_cabi_realloc = &self.cabi_realloc;
            let memory = &self.memory;
            let Httprequest {
                path: path0,
                headers: headers0,
                querystring: querystring0,
                method: method0,
            } = a;
            let vec1 = path0;
            let ptr1 = func_cabi_realloc.call(&mut caller, (0, 0, 1, vec1.len() as i32))?;
            memory
                .data_mut(&mut caller)
                .store_many(ptr1, vec1.as_bytes())?;
            let vec5 = headers0;
            let len5 = vec5.len() as i32;
            let result5 = func_cabi_realloc.call(&mut caller, (0, 0, 4, len5 * 16))?;
            for (i, e) in vec5.into_iter().enumerate() {
                let base = result5 + (i as i32) * 16;
                {
                    let (t2_0, t2_1) = e;
                    let vec3 = t2_0;
                    let ptr3 = func_cabi_realloc.call(&mut caller, (0, 0, 1, vec3.len() as i32))?;
                    memory
                        .data_mut(&mut caller)
                        .store_many(ptr3, vec3.as_bytes())?;
                    memory.data_mut(&mut caller).store(
                        base + 4,
                        wit_bindgen_host_wasmtime_rust::rt::as_i32(vec3.len() as i32),
                    )?;
                    memory
                        .data_mut(&mut caller)
                        .store(base + 0, wit_bindgen_host_wasmtime_rust::rt::as_i32(ptr3))?;
                    let vec4 = t2_1;
                    let ptr4 = func_cabi_realloc.call(&mut caller, (0, 0, 1, vec4.len() as i32))?;
                    memory
                        .data_mut(&mut caller)
                        .store_many(ptr4, vec4.as_bytes())?;
                    memory.data_mut(&mut caller).store(
                        base + 12,
                        wit_bindgen_host_wasmtime_rust::rt::as_i32(vec4.len() as i32),
                    )?;
                    memory
                        .data_mut(&mut caller)
                        .store(base + 8, wit_bindgen_host_wasmtime_rust::rt::as_i32(ptr4))?;
                }
            }
            let vec6 = querystring0;
            let ptr6 = func_cabi_realloc.call(&mut caller, (0, 0, 1, vec6.len() as i32))?;
            memory
                .data_mut(&mut caller)
                .store_many(ptr6, vec6.as_bytes())?;
            let vec7 = method0;
            let ptr7 = func_cabi_realloc.call(&mut caller, (0, 0, 1, vec7.len() as i32))?;
            memory
                .data_mut(&mut caller)
                .store_many(ptr7, vec7.as_bytes())?;
            let (result8_0,) = self.handlerequest.call(
                &mut caller,
                (
                    ptr1,
                    vec1.len() as i32,
                    result5,
                    len5,
                    ptr6,
                    vec6.len() as i32,
                    ptr7,
                    vec7.len() as i32,
                ),
            )?;
            let load9 = memory.data_mut(&mut caller).load::<i32>(result8_0 + 0)?;
            let load10 = memory.data_mut(&mut caller).load::<i32>(result8_0 + 4)?;
            let ptr11 = load9;
            let len11 = load10;

            let data11 = copy_slice(&mut caller, memory, ptr11, len11, 1)?;
            func_canonical_abi_free.call(&mut caller, (ptr11, len11, 1))?;
            Ok(String::from_utf8(data11).map_err(|_| wasmtime::Trap::new("invalid utf-8"))?)
        }
    }
    use wit_bindgen_host_wasmtime_rust::rt::copy_slice;
    use wit_bindgen_host_wasmtime_rust::rt::RawMem;
}
