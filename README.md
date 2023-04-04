# vptree-rust

This is a Rust library implementing a [VPTree](https://en.wikipedia.org/wiki/Vantage-point_tree) data structure. The algorithm for building a VPTree can be explained by means of a binary tree, which stores the input points in the leaves, and keeps additional information on the internal nodes. The algorithm implemented here differs from the original one: the points are stored, together with other additional information, in a linear structure, taking half the size of the original binary tree approach.


## References

1. Peter N. Yianilos. 1993. Data structures and algorithms for nearest neighbor search in general metric spaces. In Proceedings of the fourth annual ACM-SIAM symposium on Discrete algorithms (SODA '93). Society for Industrial and Applied Mathematics, USA, 311–321.
2. Sergey Brin. 1995. Near Neighbor Search in Large Metric Spaces. In Proceedings of the 21th International Conference on Very Large Data Bases (VLDB '95). Morgan Kaufmann Publishers Inc., San Francisco, CA, USA, 574–584.
