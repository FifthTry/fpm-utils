#[allow(clippy::all)]
mod guest_backend {
    #[derive(Clone, serde::Serialize)]
    pub struct Httprequest {
        pub path: String,
        pub headers: Vec<(String, String)>,
        pub querystring: String,
        pub payload: String,
        pub method: String,
    }
    impl core::fmt::Debug for Httprequest {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("Httprequest")
                .field("path", &self.path)
                .field("headers", &self.headers)
                .field("querystring", &self.querystring)
                .field("payload", &self.payload)
                .field("method", &self.method)
                .finish()
        }
    }
    #[derive(Clone)]
    pub struct Httpresponse {
        pub data: String,
        pub success: bool,
    }
    impl core::fmt::Debug for Httpresponse {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("Httpresponse")
                .field("data", &self.data)
                .field("success", &self.success)
                .finish()
        }
    }
    #[export_name = "handlerequest: func(a: record { path: string, headers: list<tuple<string, string>>, querystring: string, payload: string, method: string }) -> record { data: string, success: bool }"]
    unsafe extern "C" fn __wit_bindgen_guest_backend_handlerequest(
        arg0: i32,
        arg1: i32,
        arg2: i32,
        arg3: i32,
        arg4: i32,
        arg5: i32,
        arg6: i32,
        arg7: i32,
        arg8: i32,
        arg9: i32,
    ) -> i32 {
        let len0 = arg1 as usize;
        let base3 = arg2;
        let len3 = arg3;
        let mut result3 = Vec::with_capacity(len3 as usize);
        for i in 0..len3 {
            let base = base3 + i * 16;
            result3.push({
                let len1 = *((base + 4) as *const i32) as usize;
                let len2 = *((base + 12) as *const i32) as usize;

                (
                    String::from_utf8(Vec::from_raw_parts(
                        *((base + 0) as *const i32) as *mut _,
                        len1,
                        len1,
                    ))
                    .unwrap(),
                    String::from_utf8(Vec::from_raw_parts(
                        *((base + 8) as *const i32) as *mut _,
                        len2,
                        len2,
                    ))
                    .unwrap(),
                )
            });
        }
        if len3 != 0 {
            std::alloc::dealloc(
                base3 as *mut _,
                std::alloc::Layout::from_size_align_unchecked((len3 as usize) * 16, 4),
            );
        }
        let len4 = arg5 as usize;
        let len5 = arg7 as usize;
        let len6 = arg9 as usize;
        let result = <super::GuestBackend as GuestBackend>::handlerequest(Httprequest {
            path: String::from_utf8(Vec::from_raw_parts(arg0 as *mut _, len0, len0)).unwrap(),
            headers: result3,
            querystring: String::from_utf8(Vec::from_raw_parts(arg4 as *mut _, len4, len4))
                .unwrap(),
            payload: String::from_utf8(Vec::from_raw_parts(arg6 as *mut _, len5, len5)).unwrap(),
            method: String::from_utf8(Vec::from_raw_parts(arg8 as *mut _, len6, len6)).unwrap(),
        });
        let ptr7 = __GUEST_BACKEND_RET_AREA.0.as_mut_ptr() as i32;
        let Httpresponse {
            data: data8,
            success: success8,
        } = result;
        let vec9 = (data8.into_bytes()).into_boxed_slice();
        let ptr9 = vec9.as_ptr() as i32;
        let len9 = vec9.len() as i32;
        core::mem::forget(vec9);
        *((ptr7 + 4) as *mut i32) = len9;
        *((ptr7 + 0) as *mut i32) = ptr9;
        *((ptr7 + 8) as *mut u8) = (match success8 {
            true => 1,
            false => 0,
        }) as u8;
        ptr7
    }

    #[repr(align(4))]
    struct __GuestBackendRetArea([u8; 12]);
    static mut __GUEST_BACKEND_RET_AREA: __GuestBackendRetArea = __GuestBackendRetArea([0; 12]);
    pub trait GuestBackend {
        fn handlerequest(a: Httprequest) -> Httpresponse;
    }
}
