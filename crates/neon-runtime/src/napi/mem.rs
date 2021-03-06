use crate::raw::{Env, Local};
use crate::napi::bindings as napi;

pub unsafe extern "C" fn strict_equals(env: Env, lhs: Local, rhs: Local) -> bool {
    let mut result = false;
    assert_eq!(napi::strict_equals(env, lhs, rhs, &mut result as *mut _), napi::Status::Ok);
    result
}
