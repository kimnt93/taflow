# CypherPattern benchmark (`Cypher` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.058 | 17.23M | 0.048 | 20.74M | 0.222 | 3.82× | 4.60× |
| 10,000 | 0.396 | 25.24M | 0.411 | 24.32M | 1.336 | 3.37× | 3.25× |
| 100,000 | 3.925 | 25.48M | 3.792 | 26.37M | 12.381 | 3.15× | 3.27× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.165 | 0.194 | 1.18× |
| 1 | 5 | 0.444 | 0.842 | 1.89× |
| 1 | 10 | 0.665 | 1.647 | 2.48× |
| 10 | 1 | 0.079 | 0.161 | 2.05× |
| 10 | 5 | 0.320 | 1.122 | 3.50× |
| 10 | 10 | 0.647 | 1.692 | 2.61× |
| 100 | 1 | 0.078 | 0.179 | 2.30× |
| 100 | 5 | 0.324 | 1.150 | 3.55× |
| 100 | 10 | 0.676 | 1.768 | 2.61× |
| 1,000 | 1 | 0.107 | 0.294 | 2.74× |
| 1,000 | 5 | 0.313 | 1.758 | 5.61× |
| 1,000 | 10 | 0.686 | 2.943 | 4.29× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
