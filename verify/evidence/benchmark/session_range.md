# SessionRange benchmark (`SessionRange` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.030 | 33.86M | 0.026 | 38.52M | 0.736 | 24.92× | 28.35× |
| 10,000 | 0.169 | 59.16M | 0.156 | 64.06M | 5.600 | 33.13× | 35.87× |
| 100,000 | 1.874 | 53.35M | 1.769 | 56.52M | 58.294 | 31.10× | 32.95× |
| 1,000,000 | 18.597 | 53.77M | 16.634 | 60.12M | 634.305 | 34.11× | 38.13× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.101 | 0.287 | 2.85× |
| 1 | 5 | 0.461 | 1.325 | 2.87× |
| 1 | 10 | 0.643 | 2.575 | 4.00× |
| 10 | 1 | 0.060 | 0.244 | 4.04× |
| 10 | 5 | 0.293 | 1.387 | 4.74× |
| 10 | 10 | 0.597 | 2.681 | 4.49× |
| 100 | 1 | 0.065 | 0.296 | 4.58× |
| 100 | 5 | 0.322 | 1.786 | 5.55× |
| 100 | 10 | 0.619 | 3.116 | 5.04× |
| 1,000 | 1 | 0.083 | 0.963 | 11.61× |
| 1,000 | 5 | 0.278 | 4.305 | 15.47× |
| 1,000 | 10 | 0.590 | 15.041 | 25.50× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
