#![crate_name = "rcls_engine"]
#![crate_type = "lib"]
#![deny(non_camel_case_types)]
#![deny(unused_parens)]
#![deny(non_upper_case_globals)]
#![deny(unused_qualifications)]
#![deny(missing_docs)] // FIXME: deny this
#![deny(unused_results)]
#![allow(type_alias_bounds)]
#![warn(non_camel_case_types)]
#![allow(missing_copy_implementations)]
#![doc(html_root_url = "https://dotmal.github.io/recluse")]

/*!
# The Recluse Game Engine

## General Info
Version: 1.0.0
Revision: 0
Release: 0

## Features
- Static and dynamic rigid bodies.
- Common convex primitives: cone, box, ball, cylinder.
- Concave geometries build from convex primitives (aka. compound geometries).
- Stable stacking.
- Island based sleeping (objects deactivation).
- Ray casting.
- Swept sphere based continuous collision detection.
- Ball-in-socket joint.
- FixedJoint joint.
- Sensors.
- Deformable bodies (aka. soft-bodies)
- Kinematic bodies

## What's missing?
- more joints, joint limits, joint motors and breakable joints.
- parallel pipeline
- GPU-based pipeline
*/

#[macro_use]
extern crate approx;
#[macro_use]
extern crate downcast_rs;
#[macro_use]
extern crate bitflags;

extern crate nalgebra as alg;
#[cfg(feature = "dim2")]
extern crate ncollide2d as collision;
#[cfg(feature = "dim3")]
extern crate ncollide3d as collision;
extern crate num_traits as num;

/*
 * The two following crates are pulled-in for
 * measuring time.
 */
#[cfg(not(any(target_arch = "wasm32", target_arch = "asmjs")))]
extern crate time;

#[cfg(all(any(target_arch = "wasm32", target_arch = "asmjs"), feature = "stdweb",))]
#[macro_use]
extern crate stdweb;

#[cfg(all(
    any(target_arch = "wasm32", target_arch = "asmjs"),
    feature = "use-wasm-bindgen",
))]
extern crate wasm_bindgen;

macro_rules! try_ret {
    ($val: expr) => {
        try_ret!($val, ())
    };
    ($val: expr, $ret: expr) => {
        if let Some(val) = $val {
            val
        } else {
            return $ret;
        }
    };
}

macro_rules! try_continue {
    ($val: expr) => {
        if let Some(val) = $val {
            val
        } else {
            continue;
        }
    };
}

macro_rules! desc_setters(
    ($($with_method: ident, $set_method: ident $(, $arg: ident: $t: ty)*)*) => {
        $(
            #[allow(missing_docs)]
            #[inline]
            pub fn $with_method(mut self $(, $arg: $t)*) -> Self {
                $(
                    self.$arg = $arg;
                )*
                self
            }

            #[allow(missing_docs)]
            #[inline]
            pub fn $set_method(&mut self $(, $arg: $t)*) -> &mut Self {
                $(
                    self.$arg = $arg;
                )*
                self
            }
        )*
    }
);

macro_rules! desc_custom_setters(
    ($($this: ident.$with_method: ident, $set_method: ident $(, $arg: ident: $t: ty)* | $imp: expr)*) => {
        $(
            #[allow(missing_docs)]
            #[inline]
            pub fn $with_method(mut $this $(, $arg: $t)*) -> Self {
                $imp
                $this
            }

            #[allow(missing_docs)]
            #[inline]
            pub fn $set_method(&mut $this $(, $arg: $t)*) -> &mut Self {
                $imp
                $this
            }
        )*
    }
);

macro_rules! desc_custom_getters(
    ($($this: ident.$get_method: ident: $t: ty | $imp: expr)*) => {
        $(
            #[allow(missing_docs)]
            #[inline]
            pub fn $get_method(&$this) -> $t {
                $imp
            }
        )*
    }
);

macro_rules! desc_getters(
    ($([val] $name: ident -> $val: ident: $t: ty)* $([ref] $ref_name: ident -> $ref_val: ident: $ref_t: ty)*) => {
        $(
            #[allow(missing_docs)]
            #[inline]
            pub fn $name(&self) -> $t {
                self.$val
            }
        )*
        $(
            #[allow(missing_docs)]
            #[inline]
            pub fn $ref_name(&self) -> &$ref_t {
                &self.$ref_val
            }
        )*
    }
);

macro_rules! user_data_accessors(
    () => {
        /// Retrieves a reference to the user-defined user-data attached to this object.
        #[inline]
        pub fn user_data(&self) -> Option<&(dyn Any + Send + Sync)> {
            self.user_data.as_ref().map(|d| &**d)
        }

        /// Retrieves a mutable reference to the user-defined user-data attached to this object.
        #[inline]
        pub fn user_data_mut(&mut self) -> Option<&mut (dyn Any + Send + Sync)> {
            self.user_data.as_mut().map(|d| &mut **d)
        }

        /// Sets the user-defined data attached to this object.
        #[inline]
        pub fn set_user_data(&mut self, data: Option<Box<dyn Any + Send + Sync>>) -> Option<Box<dyn Any + Send + Sync>> {
            std::mem::replace(&mut self.user_data, data)
        }

        /// Replace by `None` the user-defined data attached to this object and returns the old value.
        #[inline]
        pub fn take_user_data(&mut self) -> Option<Box<dyn Any + Send + Sync>> {
            self.user_data.take()
        }
    }
);

macro_rules! user_data_desc_accessors(
    () => {
        /// Sets a user-data to be attached to the object being built.
        pub fn user_data(mut self, data: impl UserData) -> Self {
            self.user_data = Some(UserDataBox(Box::new(data) as Box<dyn UserData>));
            self
        }

        /// Sets the user-data to be attached to the object being built.
        pub fn set_user_data(&mut self, data: Option<impl UserData>) -> &mut Self {
            self.user_data = data.map(|data| UserDataBox(Box::new(data) as Box<dyn UserData>));
            self
        }

        /// Reference to the user-data to be attached to the object being built.
        pub fn get_user_data(&self) -> Option<&(dyn Any + Send + Sync)> {
            self.user_data.as_ref().map(|data| data.0.as_any())
        }
    }
);
