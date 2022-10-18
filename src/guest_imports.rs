#[allow(clippy::all)]
mod host {
    #[derive(Clone)]
    pub struct Httpresponse {
        pub data: String,
    }
    impl core::fmt::Debug for Httpresponse {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("Httpresponse")
                .field("data", &self.data)
                .finish()
        }
    }
    #[derive(Clone)]
    pub struct Httprequest<'a> {
        pub path: &'a str,
        pub method: &'a str,
        pub payload: &'a str,
        pub headers: &'a [(&'a str, &'a str)],
    }
    impl<'a> core::fmt::Debug for Httprequest<'a> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("Httprequest")
                .field("path", &self.path)
                .field("method", &self.method)
                .field("payload", &self.payload)
                .field("headers", &self.headers)
                .finish()
        }
    }
    pub fn http(request: Httprequest<'_>) -> Httpresponse {
        unsafe {
            let Httprequest {
                path: path0,
                method: method0,
                payload: payload0,
                headers: headers0,
            } = request;
            let vec1 = path0;
            let ptr1 = vec1.as_ptr() as i32;
            let len1 = vec1.len() as i32;
            let vec2 = method0;
            let ptr2 = vec2.as_ptr() as i32;
            let len2 = vec2.len() as i32;
            let vec3 = payload0;
            let ptr3 = vec3.as_ptr() as i32;
            let len3 = vec3.len() as i32;
            let vec7 = headers0;
            let len7 = vec7.len() as i32;
            let layout7 = core::alloc::Layout::from_size_align_unchecked(vec7.len() * 16, 4);
            let result7 = if layout7.size() != 0 {
                let ptr = std::alloc::alloc(layout7);
                if ptr.is_null() {
                    std::alloc::handle_alloc_error(layout7);
                }
                ptr
            } else {
                std::ptr::null_mut()
            };
            for (i, e) in vec7.into_iter().enumerate() {
                let base = result7 as i32 + (i as i32) * 16;
                {
                    let (t4_0, t4_1) = e;
                    let vec5 = t4_0;
                    let ptr5 = vec5.as_ptr() as i32;
                    let len5 = vec5.len() as i32;
                    *((base + 4) as *mut i32) = len5;
                    *((base + 0) as *mut i32) = ptr5;
                    let vec6 = t4_1;
                    let ptr6 = vec6.as_ptr() as i32;
                    let len6 = vec6.len() as i32;
                    *((base + 12) as *mut i32) = len6;
                    *((base + 8) as *mut i32) = ptr6;
                }
            }
            let ptr8 = __HOST_RET_AREA.0.as_mut_ptr() as i32;
            #[link(wasm_import_module = "host")]
            extern "C" {
                #[cfg_attr(
                    target_arch = "wasm32",
                    link_name = "http: func(request: record { path: string, method: string, payload: string, headers: list<tuple<string, string>> }) -> record { data: string }"
                )]
                #[cfg_attr(
                    not(target_arch = "wasm32"),
                    link_name = "host_http: func(request: record { path: string, method: string, payload: string, headers: list<tuple<string, string>> }) -> record { data: string }"
                )]
                fn wit_import(
                    _: i32,
                    _: i32,
                    _: i32,
                    _: i32,
                    _: i32,
                    _: i32,
                    _: i32,
                    _: i32,
                    _: i32,
                );
            }
            wit_import(
                ptr1,
                len1,
                ptr2,
                len2,
                ptr3,
                len3,
                result7 as i32,
                len7,
                ptr8,
            );
            let len9 = *((ptr8 + 4) as *const i32) as usize;
            if layout7.size() != 0 {
                std::alloc::dealloc(result7, layout7);
            }
            Httpresponse {
                data: String::from_utf8(Vec::from_raw_parts(
                    *((ptr8 + 0) as *const i32) as *mut _,
                    len9,
                    len9,
                ))
                .unwrap(),
            }
        }
    }

    #[repr(align(4))]
    struct __HostRetArea([u8; 8]);
    static mut __HOST_RET_AREA: __HostRetArea = __HostRetArea([0; 8]);
}
