#[allow(dead_code)]
pub mod component {
    #[allow(dead_code)]
    pub mod rpn {
        #[allow(dead_code, clippy::all)]
        pub mod types {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            #[repr(u8)]
            #[derive(Clone, Copy, Eq, PartialEq)]
            pub enum Operation {
                Add,
                Sub,
                Mul,
                Div,
            }
            impl ::core::fmt::Debug for Operation {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    match self {
                        Operation::Add => f.debug_tuple("Operation::Add").finish(),
                        Operation::Sub => f.debug_tuple("Operation::Sub").finish(),
                        Operation::Mul => f.debug_tuple("Operation::Mul").finish(),
                        Operation::Div => f.debug_tuple("Operation::Div").finish(),
                    }
                }
            }
            impl Operation {
                #[doc(hidden)]
                pub unsafe fn _lift(val: u8) -> Operation {
                    if !cfg!(debug_assertions) {
                        return ::core::mem::transmute(val);
                    }
                    match val {
                        0 => Operation::Add,
                        1 => Operation::Sub,
                        2 => Operation::Mul,
                        3 => Operation::Div,
                        _ => panic!("invalid enum discriminant"),
                    }
                }
            }
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct Engine {
                handle: _rt::Resource<Engine>,
            }
            impl Engine {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
                    }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for Engine {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "component:rpn/types@0.1.0")]
                        extern "C" {
                            #[link_name = "[resource-drop]engine"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            impl Engine {
                #[allow(unused_unsafe, clippy::all)]
                pub fn new() -> Self {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "component:rpn/types@0.1.0")]
                        extern "C" {
                            #[link_name = "[constructor]engine"]
                            fn wit_import() -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import() -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import();
                        Engine::from_handle(ret as u32)
                    }
                }
            }
            impl Engine {
                #[allow(unused_unsafe, clippy::all)]
                pub fn push_operand(&self, operand: u32) {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "component:rpn/types@0.1.0")]
                        extern "C" {
                            #[link_name = "[method]engine.push-operand"]
                            fn wit_import(_: i32, _: i32);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, _rt::as_i32(&operand));
                    }
                }
            }
            impl Engine {
                #[allow(unused_unsafe, clippy::all)]
                pub fn push_operation(&self, operation: Operation) {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "component:rpn/types@0.1.0")]
                        extern "C" {
                            #[link_name = "[method]engine.push-operation"]
                            fn wit_import(_: i32, _: i32);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, operation.clone() as i32);
                    }
                }
            }
            impl Engine {
                #[allow(unused_unsafe, clippy::all)]
                pub fn execute(&self) -> u32 {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "component:rpn/types@0.1.0")]
                        extern "C" {
                            #[link_name = "[method]engine.execute"]
                            fn wit_import(_: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        ret as u32
                    }
                }
            }
        }
    }
}
mod _rt {
    use core::fmt;
    use core::marker;
    use core::sync::atomic::{AtomicU32, Ordering::Relaxed};
    /// A type which represents a component model resource, either imported or
    /// exported into this component.
    ///
    /// This is a low-level wrapper which handles the lifetime of the resource
    /// (namely this has a destructor). The `T` provided defines the component model
    /// intrinsics that this wrapper uses.
    ///
    /// One of the chief purposes of this type is to provide `Deref` implementations
    /// to access the underlying data when it is owned.
    ///
    /// This type is primarily used in generated code for exported and imported
    /// resources.
    #[repr(transparent)]
    pub struct Resource<T: WasmResource> {
        handle: AtomicU32,
        _marker: marker::PhantomData<T>,
    }
    /// A trait which all wasm resources implement, namely providing the ability to
    /// drop a resource.
    ///
    /// This generally is implemented by generated code, not user-facing code.
    #[allow(clippy::missing_safety_doc)]
    pub unsafe trait WasmResource {
        /// Invokes the `[resource-drop]...` intrinsic.
        unsafe fn drop(handle: u32);
    }
    impl<T: WasmResource> Resource<T> {
        #[doc(hidden)]
        pub unsafe fn from_handle(handle: u32) -> Self {
            debug_assert!(handle != u32::MAX);
            Self {
                handle: AtomicU32::new(handle),
                _marker: marker::PhantomData,
            }
        }
        /// Takes ownership of the handle owned by `resource`.
        ///
        /// Note that this ideally would be `into_handle` taking `Resource<T>` by
        /// ownership. The code generator does not enable that in all situations,
        /// unfortunately, so this is provided instead.
        ///
        /// Also note that `take_handle` is in theory only ever called on values
        /// owned by a generated function. For example a generated function might
        /// take `Resource<T>` as an argument but then call `take_handle` on a
        /// reference to that argument. In that sense the dynamic nature of
        /// `take_handle` should only be exposed internally to generated code, not
        /// to user code.
        #[doc(hidden)]
        pub fn take_handle(resource: &Resource<T>) -> u32 {
            resource.handle.swap(u32::MAX, Relaxed)
        }
        #[doc(hidden)]
        pub fn handle(resource: &Resource<T>) -> u32 {
            resource.handle.load(Relaxed)
        }
    }
    impl<T: WasmResource> fmt::Debug for Resource<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Resource").field("handle", &self.handle).finish()
        }
    }
    impl<T: WasmResource> Drop for Resource<T> {
        fn drop(&mut self) {
            unsafe {
                match self.handle.load(Relaxed) {
                    u32::MAX => {}
                    other => T::drop(other),
                }
            }
        }
    }
    pub fn as_i32<T: AsI32>(t: T) -> i32 {
        t.as_i32()
    }
    pub trait AsI32 {
        fn as_i32(self) -> i32;
    }
    impl<'a, T: Copy + AsI32> AsI32 for &'a T {
        fn as_i32(self) -> i32 {
            (*self).as_i32()
        }
    }
    impl AsI32 for i32 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u32 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for i16 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u16 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for i8 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u8 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for char {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for usize {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
}
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.30.0:app:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 413] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xa3\x02\x01A\x02\x01\
A\x02\x01B\x0d\x01m\x04\x03add\x03sub\x03mul\x03div\x04\0\x09operation\x03\0\0\x04\
\0\x06engine\x03\x01\x01i\x02\x01@\0\0\x03\x04\0\x13[constructor]engine\x01\x04\x01\
h\x02\x01@\x02\x04self\x05\x07operandy\x01\0\x04\0\x1b[method]engine.push-operan\
d\x01\x06\x01@\x02\x04self\x05\x09operation\x01\x01\0\x04\0\x1d[method]engine.pu\
sh-operation\x01\x07\x01@\x01\x04self\x05\0y\x04\0\x16[method]engine.execute\x01\
\x08\x03\x01\x19component:rpn/types@0.1.0\x05\0\x04\x01\x15component:rpn-cmd/app\
\x04\0\x0b\x09\x01\0\x03app\x03\0\0\0G\x09producers\x01\x0cprocessed-by\x02\x0dw\
it-component\x070.215.0\x10wit-bindgen-rust\x060.30.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
