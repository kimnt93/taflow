# HilbertDominantCycle benchmark (`HilbertDominantCycle` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.067 | 14.87M | 0.061 | 16.30M | 0.204 | 3.03× | 3.32× |
| 10,000 | 0.636 | 15.74M | 0.627 | 15.95M | 1.074 | 1.69× | 1.71× |
| 100,000 | 6.361 | 15.72M | 6.145 | 16.27M | 9.546 | 1.50× | 1.55× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.110 | 0.237 | 2.16× |
| 1 | 5 | 0.220 | 1.200 | 5.46× |
| 1 | 10 | 0.451 | 1.981 | 4.40× |
| 10 | 1 | 0.045 | 0.173 | 3.88× |
| 10 | 5 | 0.196 | 0.864 | 4.41× |
| 10 | 10 | 0.441 | 1.958 | 4.44× |
| 100 | 1 | 0.053 | 0.182 | 3.41× |
| 100 | 5 | 0.210 | 0.876 | 4.18× |
| 100 | 10 | 0.466 | 2.015 | 4.32× |
| 1,000 | 1 | 0.108 | 0.267 | 2.48× |
| 1,000 | 5 | 0.231 | 1.322 | 5.71× |
| 1,000 | 10 | 0.477 | 2.666 | 5.59× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
