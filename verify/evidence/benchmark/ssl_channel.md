# SmoothedTrendChannel benchmark (`smoothed trend channel` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.017 | 58.97M | 0.014 | 69.16M | 0.572 | 33.71× | 39.54× |
| 10,000 | 0.128 | 78.24M | 0.121 | 82.83M | 5.295 | 41.43× | 43.86× |
| 100,000 | 1.237 | 80.84M | 1.181 | 84.65M | 49.893 | 40.33× | 42.23× |
| 1,000,000 | 12.664 | 78.96M | 12.030 | 83.13M | 501.306 | 39.58× | 41.67× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.116 | 0.147 | 1.26× |
| 1 | 5 | 0.335 | 0.561 | 1.68× |
| 1 | 10 | 0.519 | 1.117 | 2.15× |
| 10 | 1 | 0.055 | 0.186 | 3.39× |
| 10 | 5 | 0.233 | 0.862 | 3.71× |
| 10 | 10 | 0.533 | 1.736 | 3.26× |
| 100 | 1 | 0.057 | 0.213 | 3.72× |
| 100 | 5 | 0.254 | 1.105 | 4.35× |
| 100 | 10 | 0.545 | 2.201 | 4.03× |
| 1,000 | 1 | 0.071 | 0.682 | 9.57× |
| 1,000 | 5 | 0.271 | 3.325 | 12.25× |
| 1,000 | 10 | 0.555 | 6.793 | 12.25× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
