"""Type aliases for semantic naming"""
from typing import Callable, Iterator, List, Optional, Tuple, TypeVar

import numpy as np

from .rust_ext import Iter, get_costs_reduced

T = TypeVar("T")
Matching = List[Tuple[T, T]]
MatchingIndices = Tuple[np.ndarray, np.ndarray]
Edge = Tuple[T, T, float]

EnumerationAlgorithm = Callable[[List[Edge[T]]], Iterator[Matching[T]]]
CostMatrix = np.ndarray

def enumerate_kbest(
    cost_matrix: CostMatrix, *,
    yield_iter: Optional[Callable[[List[Tuple[int, int]], MatchingIndices], MatchingIndices]] = None
) -> Iterator[MatchingIndices]:
    """
    When `ignore_same_value` is set to True, yield only one matching for each cost.
    Otherwise, return all possible matchings, even if some of them has the same value.

    Iterate through triplets of (matching cost, row indices of a solution, column indices of a solution)
    """
    for state in Iter(cost_matrix):
        a_solution = np.arange(len(state.a_solution)), state.a_solution
        if yield_iter is None:
            yield a_solution
        else:
            costs = get_costs_reduced(state)
            rows, cols = np.nonzero(np.isclose(costs, 0.))
            yield from yield_iter(list(zip(rows, cols)), a_solution)
