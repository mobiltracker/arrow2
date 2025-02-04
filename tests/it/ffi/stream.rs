use std::sync::Arc;

use arrow2::array::*;
use arrow2::datatypes::Field;
use arrow2::{error::Result, ffi};

fn _test_round_trip(arrays: Vec<Arc<dyn Array>>) -> Result<()> {
    let field = Field::new("a", arrays[0].data_type().clone(), true);
    let iter = Box::new(arrays.clone().into_iter().map(Ok)) as _;

    let mut stream = Box::new(ffi::ArrowArrayStream::empty());

    unsafe { ffi::export_iterator(iter, field.clone(), &mut *stream) }

    let mut stream = unsafe { ffi::ArrowArrayStreamReader::try_new(stream)? };

    let mut produced_arrays: Vec<Arc<dyn Array>> = vec![];
    while let Some(array) = unsafe { stream.next() } {
        produced_arrays.push(array?.into());
    }

    assert_eq!(produced_arrays, arrays);
    assert_eq!(stream.field(), &field);
    Ok(())
}

#[test]
fn round_trip() -> Result<()> {
    let array = Int32Array::from(&[Some(2), None, Some(1), None]);
    let array: Arc<dyn Array> = Arc::new(array);

    _test_round_trip(vec![array.clone(), array.clone(), array])
}
