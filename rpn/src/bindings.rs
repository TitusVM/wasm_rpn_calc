#[allow(dead_code)]
pub mod exports {
    #[allow(dead_code)]
    pub mod component {
        #[allow(dead_code)]
        pub mod rpn {
            #[allow(dead_code, clippy::all)]
            pub mod types {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
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
                type _EngineRep<T> = Option<T>;
                impl Engine {
                    /// Creates a new resource from the specified representation.
                    ///
                    /// This function will create a new resource handle by moving `val` onto
                    /// the heap and then passing that heap pointer to the component model to
                    /// create a handle. The owned handle is then returned as `Engine`.
                    pub fn new<T: GuestEngine>(val: T) -> Self {
                        Self::type_guard::<T>();
                        let val: _EngineRep<T> = Some(val);
                        let ptr: *mut _EngineRep<T> = _rt::Box::into_raw(
                            _rt::Box::new(val),
                        );
                        unsafe { Self::from_handle(T::_resource_new(ptr.cast())) }
                    }
                    /// Gets access to the underlying `T` which represents this resource.
                    pub fn get<T: GuestEngine>(&self) -> &T {
                        let ptr = unsafe { &*self.as_ptr::<T>() };
                        ptr.as_ref().unwrap()
                    }
                    /// Gets mutable access to the underlying `T` which represents this
                    /// resource.
                    pub fn get_mut<T: GuestEngine>(&mut self) -> &mut T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.as_mut().unwrap()
                    }
                    /// Consumes this resource and returns the underlying `T`.
                    pub fn into_inner<T: GuestEngine>(self) -> T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.take().unwrap()
                    }
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
                    #[doc(hidden)]
                    fn type_guard<T: 'static>() {
                        use core::any::TypeId;
                        static mut LAST_TYPE: Option<TypeId> = None;
                        unsafe {
                            assert!(! cfg!(target_feature = "atomics"));
                            let id = TypeId::of::<T>();
                            match LAST_TYPE {
                                Some(ty) => {
                                    assert!(
                                        ty == id, "cannot use two types with this resource type"
                                    )
                                }
                                None => LAST_TYPE = Some(id),
                            }
                        }
                    }
                    #[doc(hidden)]
                    pub unsafe fn dtor<T: 'static>(handle: *mut u8) {
                        Self::type_guard::<T>();
                        let _ = _rt::Box::from_raw(handle as *mut _EngineRep<T>);
                    }
                    fn as_ptr<T: GuestEngine>(&self) -> *mut _EngineRep<T> {
                        Engine::type_guard::<T>();
                        T::_resource_rep(self.handle()).cast()
                    }
                }
                /// A borrowed version of [`Engine`] which represents a borrowed value
                /// with the lifetime `'a`.
                #[derive(Debug)]
                #[repr(transparent)]
                pub struct EngineBorrow<'a> {
                    rep: *mut u8,
                    _marker: core::marker::PhantomData<&'a Engine>,
                }
                impl<'a> EngineBorrow<'a> {
                    #[doc(hidden)]
                    pub unsafe fn lift(rep: usize) -> Self {
                        Self {
                            rep: rep as *mut u8,
                            _marker: core::marker::PhantomData,
                        }
                    }
                    /// Gets access to the underlying `T` in this resource.
                    pub fn get<T: GuestEngine>(&self) -> &T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.as_ref().unwrap()
                    }
                    fn as_ptr<T: 'static>(&self) -> *mut _EngineRep<T> {
                        Engine::type_guard::<T>();
                        self.rep.cast()
                    }
                }
                unsafe impl _rt::WasmResource for Engine {
                    #[inline]
                    unsafe fn drop(_handle: u32) {
                        #[cfg(not(target_arch = "wasm32"))]
                        unreachable!();
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(
                                wasm_import_module = "[export]component:rpn/types@0.1.0"
                            )]
                            extern "C" {
                                #[link_name = "[resource-drop]engine"]
                                fn drop(_: u32);
                            }
                            drop(_handle);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_constructor_engine_cabi<T: GuestEngine>() -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = Engine::new(T::new());
                    (result0).take_handle() as i32
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_engine_push_operand_cabi<T: GuestEngine>(
                    arg0: *mut u8,
                    arg1: i32,
                ) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    T::push_operand(
                        EngineBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                    );
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_engine_push_operation_cabi<T: GuestEngine>(
                    arg0: *mut u8,
                    arg1: i32,
                ) {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    T::push_operation(
                        EngineBorrow::lift(arg0 as u32 as usize).get(),
                        Operation::_lift(arg1 as u8),
                    );
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_engine_execute_cabi<T: GuestEngine>(
                    arg0: *mut u8,
                ) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::execute(
                        EngineBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    _rt::as_i32(result0)
                }
                pub trait Guest {
                    type Engine: GuestEngine;
                }
                pub trait GuestEngine: 'static {
                    #[doc(hidden)]
                    unsafe fn _resource_new(val: *mut u8) -> u32
                    where
                        Self: Sized,
                    {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let _ = val;
                            unreachable!();
                        }
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(
                                wasm_import_module = "[export]component:rpn/types@0.1.0"
                            )]
                            extern "C" {
                                #[link_name = "[resource-new]engine"]
                                fn new(_: *mut u8) -> u32;
                            }
                            new(val)
                        }
                    }
                    #[doc(hidden)]
                    fn _resource_rep(handle: u32) -> *mut u8
                    where
                        Self: Sized,
                    {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let _ = handle;
                            unreachable!();
                        }
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(
                                wasm_import_module = "[export]component:rpn/types@0.1.0"
                            )]
                            extern "C" {
                                #[link_name = "[resource-rep]engine"]
                                fn rep(_: u32) -> *mut u8;
                            }
                            unsafe { rep(handle) }
                        }
                    }
                    fn new() -> Self;
                    fn push_operand(&self, operand: u32);
                    fn push_operation(&self, operation: Operation);
                    fn execute(&self) -> u32;
                }
                #[doc(hidden)]
                macro_rules! __export_component_rpn_types_0_1_0_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "component:rpn/types@0.1.0#[constructor]engine"] unsafe extern
                        "C" fn export_constructor_engine() -> i32 { $($path_to_types)*::
                        _export_constructor_engine_cabi::<<$ty as $($path_to_types)*::
                        Guest >::Engine > () } #[export_name =
                        "component:rpn/types@0.1.0#[method]engine.push-operand"] unsafe
                        extern "C" fn export_method_engine_push_operand(arg0 : * mut u8,
                        arg1 : i32,) { $($path_to_types)*::
                        _export_method_engine_push_operand_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Engine > (arg0, arg1) }
                        #[export_name =
                        "component:rpn/types@0.1.0#[method]engine.push-operation"] unsafe
                        extern "C" fn export_method_engine_push_operation(arg0 : * mut
                        u8, arg1 : i32,) { $($path_to_types)*::
                        _export_method_engine_push_operation_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Engine > (arg0, arg1) }
                        #[export_name =
                        "component:rpn/types@0.1.0#[method]engine.execute"] unsafe extern
                        "C" fn export_method_engine_execute(arg0 : * mut u8,) -> i32 {
                        $($path_to_types)*:: _export_method_engine_execute_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::Engine > (arg0) } const _ : () = {
                        #[doc(hidden)] #[export_name =
                        "component:rpn/types@0.1.0#[dtor]engine"]
                        #[allow(non_snake_case)] unsafe extern "C" fn dtor(rep : * mut
                        u8) { $($path_to_types)*:: Engine::dtor::< <$ty as
                        $($path_to_types)*:: Guest >::Engine > (rep) } }; };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_component_rpn_types_0_1_0_cabi;
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
    pub use alloc_crate::boxed::Box;
    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
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
    extern crate alloc as alloc_crate;
}
/// Generates `#[no_mangle]` functions to export the specified type as the
/// root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! export{ ($($t:tt)*) => (); }
/// # trait Guest {}
/// struct MyType;
///
/// impl Guest for MyType {
///     // ...
/// }
///
/// export!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]
macro_rules! __export_calculator_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*::
        exports::component::rpn::types::__export_component_rpn_types_0_1_0_cabi!($ty
        with_types_in $($path_to_types_root)*:: exports::component::rpn::types);
    };
}
#[doc(inline)]
pub(crate) use __export_calculator_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.30.0:calculator:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 429] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xac\x02\x01A\x02\x01\
A\x02\x01B\x0d\x01m\x04\x03add\x03sub\x03mul\x03div\x04\0\x09operation\x03\0\0\x04\
\0\x06engine\x03\x01\x01i\x02\x01@\0\0\x03\x04\0\x13[constructor]engine\x01\x04\x01\
h\x02\x01@\x02\x04self\x05\x07operandy\x01\0\x04\0\x1b[method]engine.push-operan\
d\x01\x06\x01@\x02\x04self\x05\x09operation\x01\x01\0\x04\0\x1d[method]engine.pu\
sh-operation\x01\x07\x01@\x01\x04self\x05\0y\x04\0\x16[method]engine.execute\x01\
\x08\x04\x01\x19component:rpn/types@0.1.0\x05\0\x04\x01\x1ecomponent:rpn/calcula\
tor@0.1.0\x04\0\x0b\x10\x01\0\x0acalculator\x03\0\0\0G\x09producers\x01\x0cproce\
ssed-by\x02\x0dwit-component\x070.215.0\x10wit-bindgen-rust\x060.30.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
