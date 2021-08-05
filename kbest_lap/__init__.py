"""K-th best matching enumeration"""
from .rust_ext import Node, NodeSet, SortedMatchingIterator

LEFT = True
RIGHT = False

__all__ = [
    "SortedMatchingIterator", "NodeSet", "Node"
]
