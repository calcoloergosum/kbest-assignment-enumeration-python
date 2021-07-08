// use numpy::{PyReadonlyArray2};
use numpy::{IntoPyArray, PyArrayDyn, PyReadonlyArray2};
use pyo3::prelude::*;
use pyo3::PyIterProtocol;

type Matrix = ndarray::Array2<f64>;

#[pyclass]
struct State {
    #[pyo3(get)]
    cost_solution: f64,
    costs_reduced: Matrix,
    #[pyo3(get)]
    a_solution: Vec<usize>,
}

impl From<kbest_lap::State<f64>> for State {
    fn from(inner: kbest_lap::State<f64>) -> Self {
        State {
            cost_solution: *inner.cost_solution,
            costs_reduced: inner.costs_reduced,
            a_solution: inner.a_solution.0,
        }
    }
}

#[pyclass]
struct Iter {
    inner: kbest_lap::KBestEnumeration<f64>,
}

#[pyproto]
impl PyIterProtocol for Iter {
    fn __iter__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<Self>) -> Option<State> {
        let s = slf.inner.next()?;
        Some(s.into())
    }
}

#[pymethods]
impl Iter {
    #[new]
    fn new(m: PyReadonlyArray2<f64>) -> Iter {
        let arr = m.as_array().to_owned();
        Iter {
            inner: kbest_lap::KBestEnumeration::<f64>::new(arr).unwrap(),
        }
    }
}

#[pymodule]
fn rust_ext(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    // wrapper of `Iter`
    m.add_class::<Iter>()?;
    m.add_class::<State>()?;

    #[pyfn(m)]
    #[pyo3(name = "get_costs_reduced")]
    fn costs_reduced<'py>(py: Python<'py>, state: &State) -> &'py PyArrayDyn<f64> {
        let dynmat = state.costs_reduced.clone().into_dyn();
        dynmat.into_pyarray(py)
    }

    Ok(())
}
