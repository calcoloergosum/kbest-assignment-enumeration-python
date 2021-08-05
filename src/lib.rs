use all_lap_rust::bipartite as bp;
use all_lap_rust::contains::Contains;
use kbest_lap as kl;
use numpy::{PyReadonlyArray2};
use pyo3::prelude::*;
use pyo3::PyIterProtocol;
use std::ops::DerefMut;

#[pyclass]
#[derive(Clone)]
struct Node {
    inner: bp::Node,
}

#[pymethods]
impl Node {
    #[new]
    fn __new__(lr: bool, index: usize) -> Self {
        let nodegroup = match lr {
            false => bp::NodeGroup::Left,
            true => bp::NodeGroup::Right,
        };
        let inner = bp::Node::new(nodegroup, index);
        Self { inner }
    }
}

#[pyclass]
#[derive(Clone)]
struct Matching {
    inner: kl::Matching,
}

#[pymethods]
impl Matching {
    #[new]
    fn new(v: Vec<Option<usize>>) -> Self {
        kl::Matching::new(v).into()
    }
    fn as_l2r(&self) -> PyResult<Vec<Option<usize>>> {
        Ok(self.inner.l2r.clone())
    }

    fn as_sparse(&self) -> PyResult<Vec<(usize, usize)>> {
        Ok(self.inner.iter_pairs().collect())
    }
}

impl From<kl::Matching> for Matching {
    fn from(val: kl::Matching) -> Self {
        Self { inner: val }
    }
}

#[pyclass]
#[derive(Clone)]
struct NodeSet {
    inner: bp::NodeSet,
}

#[pymethods]
impl NodeSet {
    #[new]
    fn __new__(nodes: Vec<Node>, lsize: usize) -> Self {
        let hashset = nodes.into_iter().map(|x| x.inner).collect();
        Self {
            inner: bp::NodeSet::new(hashset, lsize),
        }
    }
}

impl Contains<bp::Node> for NodeSet {
    fn contains_node(&self, item: &bp::Node) -> bool {
        self.inner.contains_node(item)
    }
}

impl Contains<usize> for NodeSet {
    fn contains_node(&self, item: &usize) -> bool {
        self.inner.contains_node(item)
    }
}

#[pyclass]
struct SortedMatchingIterator {
    inner: kl::SortedMatchingCalculator,
    allowed_start_nodes: NodeSet,
}

#[pymethods]
impl SortedMatchingIterator {
    #[new]
    fn new(m: PyReadonlyArray2<f64>, allowed_start_nodes: NodeSet) -> Self {
        let costs = m.as_array().to_owned();
        let inner = kl::SortedMatchingCalculator::from_costs(costs);
        Self {
            inner,
            allowed_start_nodes,
        }
    }
}

#[pyproto]
impl PyIterProtocol for SortedMatchingIterator {
    fn __iter__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<Self>) -> Option<Matching> {
        let _self = slf.deref_mut();
        let m = _self.inner.next_item(&_self.allowed_start_nodes)?;
        Some(m.into())
    }
}

#[pymodule]
fn rust_ext(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<Matching>()?;
    m.add_class::<Node>()?;
    m.add_class::<NodeSet>()?;
    m.add_class::<SortedMatchingIterator>()?;

    Ok(())
}
