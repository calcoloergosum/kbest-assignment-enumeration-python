"""
Naive approach; find by enumeration of perfect matching
"""
import heapq
from typing import Callable, Iterator, List, Tuple, TypeVar

try:
    import networkx as nx
    import networkx.algorithms.bipartite as bprt
except ImportError:
    print("Networkx not found. Regression test is not available")

T = TypeVar("T")

Matching = List[Tuple[T, T]]

MAX_HEAPSIZE = 100

def enumerate_naive(edges: List[Tuple[T, T, float]]) -> Iterator[Matching
]:
    """Enumerate best matchings"""
    graph = nx.Graph()
    for node1, node2, weight in edges:
        graph.add_node(node1)
        graph.add_node(node2)
        graph.add_edge(node1, node2, weight=weight)

    if not bprt.is_bipartite(graph):
        raise RuntimeError("Not bipartite")

    left_set, right_set = nx.bipartite.sets(graph)
    lefts = sorted(left_set)
    rights = sorted(right_set)
    heap = []
    for matching in _naive(lefts, rights, is_valid=lambda x, y: (x, y) in graph.edges):
        score = sum(graph.edges[(n1, n2)]['weight'] for n1, n2 in matching)
        if len(heap) <= MAX_HEAPSIZE:
            heapq.heappush(heap, (score, matching))
        else:
            heapq.heappushpop(heap, (score, matching))

    while heap != []:
        score, matching = heapq.heappop(heap)
        yield matching


def _naive(lefts: List[T], rights: List[T], is_valid: Callable[[T, T], bool]) -> Iterator[Matching[T]]:
    if lefts == [] or rights == []:
        raise RuntimeError("Exhausted")

    if len(lefts) != len(rights):
        raise RuntimeError("Left and right has different length")

    if len(lefts) == 1:
        yield from [[(lefts[0], rights[0])]]
        return

    head, *tail = lefts
    for i, right in enumerate(rights):
        pair = (head, right)
        if not is_valid(*pair):
            continue

        for matching in _naive(tail, rights[:i] + rights[i + 1:], is_valid=is_valid):
            yield [pair] + matching
