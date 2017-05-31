#![feature(proc_macro, specialization)]
#![allow(dead_code, unused_variables)]

extern crate pyo3;

use pyo3::*;


#[test]
fn test_basics() {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let v = PySlice::new(py, 1, 10, 2);
    let indices = v.indices(100).unwrap();
    assert_eq!(1, indices.start);
    assert_eq!(10, indices.stop);
    assert_eq!(2, indices.step);
    assert_eq!(5, indices.slicelength);
}


#[py::class]
struct Test {}

#[py::proto]
impl<'p> PyMappingProtocol<'p> for Test
{
    fn __getitem__(&self, py: Python, idx: PyObject<'p>) -> PyResult<pptr> {
        if let Ok(slice) = idx.cast_as::<PySlice>() {
            let indices = slice.indices(1000)?;
            if indices.start == 100 && indices.stop == 200 && indices.step == 1 {
                return Ok("slice".into_object(py))
            }
        }
        else if let Ok(idx) = idx.extract::<isize>() {
            if idx == 1 {
                return Ok("int".into_object(py))
            }
        }
        Err(PyErr::new::<exc::ValueError, _>(py, "error"))
    }
}

#[test]
fn test_cls_impl() {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let ob = py.with_token(|e| Test{});
    let d = PyDict::new(py);
    d.set_item("ob", ob).unwrap();

    py.run("assert ob[1] == 'int'", None, Some(&d)).unwrap();
    py.run("assert ob[100:200:1] == 'slice'", None, Some(&d)).unwrap();
}
