# FracDiff benchmark (`fixed-width fractional differencing` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.071 | 14.04M | 0.070 | 14.31M | 0.275 | 3.86× | 3.94× |
| 10,000 | 7.113 | 1.41M | 7.065 | 1.42M | 7.746 | 1.09× | 1.10× |
| 100,000 | 78.480 | 1.27M | 82.035 | 1.22M | 83.897 | 1.07× | 1.02× |
| 1,000,000 | 782.482 | 1.28M | 781.021 | 1.28M | 801.979 | 1.02× | 1.03× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.237 | 0.356 | 1.51× |
| 1 | 5 | 0.416 | 1.453 | 3.49× |
| 1 | 10 | 0.564 | 2.810 | 4.99× |
| 10 | 1 | 0.054 | 0.279 | 5.13× |
| 10 | 5 | 0.263 | 1.354 | 5.15× |
| 10 | 10 | 0.521 | 2.722 | 5.23× |
| 100 | 1 | 0.055 | 0.267 | 4.82× |
| 100 | 5 | 0.262 | 1.345 | 5.13× |
| 100 | 10 | 0.545 | 2.693 | 4.94× |
| 1,000 | 1 | 0.131 | 0.377 | 2.88× |
| 1,000 | 5 | 0.309 | 1.857 | 6.00× |
| 1,000 | 10 | 0.572 | 3.748 | 6.56× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
