// INTERNAL IMPORTS
extern crate yarrow_validator;
use yarrow_validator::{yarrow, ptr_to_buffer, buffer_to_ptr};

mod base;
mod utilities;
mod components;
mod statistics;

#[repr(C)]
#[allow(dead_code)]
struct ByteBuffer {
    len: i64,
    data: *mut u8,
}

#[no_mangle]
pub extern "C" fn compute_release(
    dataset_ptr: *const u8, dataset_length: i32,
    analysis_ptr: *const u8, analysis_length: i32,
    release_ptr: *const u8, release_length: i32
) -> ffi_support::ByteBuffer {

    let dataset_buffer = unsafe {ptr_to_buffer(dataset_ptr, dataset_length)};
    let dataset: yarrow::Dataset = prost::Message::decode(dataset_buffer).unwrap();

    let analysis_buffer = unsafe {ptr_to_buffer(analysis_ptr, analysis_length)};
    let analysis: yarrow::Analysis = prost::Message::decode(analysis_buffer).unwrap();

    let release_buffer = unsafe {ptr_to_buffer(release_ptr, release_length)};
    let release: yarrow::Release = prost::Message::decode(release_buffer).unwrap();

    let response_release = base::compute_release(&analysis, &release, &dataset);
    buffer_to_ptr(response_release)
}

//ffi_support::implement_into_ffi_by_protobuf!(yarrow::Release);
//ffi_support::define_bytebuffer_destructor!(dp_runtime_destroy_bytebuffer);
