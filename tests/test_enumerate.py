import itertools as it

import numpy as np
import pytest

import kbest_lap


@pytest.mark.parametrize('size', [5,] * 10)
def test_linear_sum_assignment(size: int) -> None:
    cost_matrix = np.random.random((size, size))
    nodes = [kbest_lap.Node(bool(b), i) for b in (0, 1) for i in range(size)]
    nodes = kbest_lap.NodeSet(nodes, size)
    solutions = list(kbest_lap.SortedMatchingIterator(cost_matrix, nodes))

    # solution count
    assert len(solutions) == np.math.factorial(size)

    get_cost = lambda s: cost_matrix[tuple(zip(*s.as_sparse()))].sum()

    # solution sort
    solution_costs = list(map(get_cost, solutions))
    solutions_sorted = [x[1] for x in sorted(enumerate(solutions), key=lambda i_s: solution_costs[i_s[0]])]
    assert solutions == solutions_sorted

    # regression with brute-force
    bf_costs = sorted(cost_matrix[range(size), js].sum() for js in it.permutations(range(size)))
    solution_costs = [get_cost(s) for s in solutions]

    np.testing.assert_allclose(bf_costs, solution_costs)
