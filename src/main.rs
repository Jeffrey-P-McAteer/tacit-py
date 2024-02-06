use pyo3::prelude::*;

fn main() -> PyResult<()> {
    std::env::set_var("PYTHONPATH", "pyembedded/stdlib");
    unsafe {
        pyo3::with_embedded_python_interpreter(|py| {
            py.run("print('hello, world')", None, None)
        })
    }
}
