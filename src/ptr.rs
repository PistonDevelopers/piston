//! Dynamic reflection of temporary pointers.

use std::intrinsics::{ get_tydesc, TypeId };

/// A safe temporary wrapper for pointers.
///
/// This can be used to call a generic callback,
/// where the type is agreed upon, but not verified by type system.
///
/// Used to work around limitations with the type system,
/// such as calling methods that might be or not be supported by a type.
///
/// Also used to work around lifetime constraints on objects,
/// when dynamic reflection is desired.
///
/// In general it is used to call closures of the form:
///
/// ```ignore
/// |&Ptr| -> U
/// ```
///
/// This kind of closure can represent any callback,
/// as long as the caller and receiver agrees about the type.
///
/// When casting from the pointer, the type id is checked,
/// and the returned reference inherits the lifetime,
/// making it safe to use in the closure callback.
/// If the type id does not match, an error message is returned,
/// with the expected type name and the type name it was attempted to cast to.
///
/// `Ptr(type id, raw ptr, type desc)`
pub struct Ptr(TypeId, *const u8, &'static str);

impl Ptr {
    /// Calls a closure with a raw pointer.
    ///
    /// Trusts the caller that the type is correct.
    /// Does not check the size.
    /// While this method is does not call unsafe code,
    /// it is unsafe to use if the type is wrong.
    #[inline(always)]
    pub unsafe fn with<T: 'static, U>(ptr: *const u8, f: |&Ptr| -> U) -> U {
        let name = (*get_tydesc::<T>()).name;
        f(&Ptr(TypeId::of::<&T>(), ptr, name))
    }

    /// Calls a closure with a string slice.
    #[inline(always)]
    pub fn with_str<U>(text: &str, f: |&Ptr| -> U) -> U {
        unsafe { Ptr::with::<&str, U>(&text as *const &str as *const u8, f) }
    }

    /// Calls a closure with a slice.
    #[inline(always)]
    pub fn with_slice<T: 'static, U>(arr: &[T], f: |&Ptr| -> U) -> U {
        unsafe { Ptr::with::<&[T], U>(&arr as *const &[T] as *const u8, f) }
    }

    /// Calls a closure with a reference.
    #[inline(always)]
    pub fn with_ref<T: 'static, U>(val: &T, f: |&Ptr| -> U) -> U {
        unsafe { Ptr::with::<T, U>(val as *const T as *const u8, f) }
    }

    /// Casts pointer into a reference.
    /// If the type does not match, an error message is returned
    /// with the expted type and attempted cast.
    #[inline(always)]
    pub fn cast<'a, T: 'static>(&'a self) -> Result<&'a T, String> {
        let id = TypeId::of::<&T>();
        let &Ptr(ptr_id, blob, name) = self;
        if ptr_id == id {
            Ok(unsafe { &*(blob as *const T) })
        } else {
            let expected_name = unsafe {
                (*get_tydesc::<T>()).name
            };
            Err(format!("Expected `{}`, found `{}`", expected_name, name))
        }
    }

    /// Cast into a string slice.
    /// Fails if the pointer is not a string slice.
    #[inline(always)]
    pub fn expect_str(&self) -> &str {
        *self.cast::<&str>().unwrap()
    }

    /// Cast into a slice.
    /// Fails if the pointer is not a slice of correct type.
    #[inline(always)]
    pub fn expect_slice<T: 'static>(&self) -> &[T] {
        *self.cast::<&[T]>().unwrap()
    }

    /// Cast into a reference.
    /// Fails if the pointer is not a reference of correct type.
    #[inline(always)]
    pub fn expect<T: 'static>(&self) -> &T {
        self.cast::<T>().unwrap()
    }
}


