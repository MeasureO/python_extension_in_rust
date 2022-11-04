// Для объединения Rust и Python необходимо обеспечить преобразование кода на этих языках
extern crate cpython;

use cpython::{PyResult, Python, py_module_initializer, py_fn};

py_module_initializer!(rust_processing, |py, m| {
        m.add(py, "doc", "Этот модуль реализован на языке Rust")?;
            m.add(py, "return_first", py_fn!(py, return_first(array: Vec<i32>)))?;
            m.add(py, "average", py_fn!(py, average(array: Vec<i32>)))?;
            Ok(())
});

fn return_first(_py: Python, array: Vec<i32>) -> PyResult<i32> {
        Ok(_return_first(&array))
}

fn _return_first(array: &Vec<i32>) -> i32 {
        array[0]
}

fn average(_py: Python, array: Vec<i32>) -> PyResult<f32> {
        Ok(_average(&array))
}

fn _average(array: &Vec<i32>) -> f32 {
        let mut res: f32 = 0_f32;
        for i in 0..array.len() {
                res += array[i] as f32;
        }
        res = res / array.len() as f32;
        res
}