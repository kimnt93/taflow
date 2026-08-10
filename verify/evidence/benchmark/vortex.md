# Vortex benchmark (`Vortex` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 56.27M | 0.015 | 65.00M | 0.504 | 28.36× | 32.76× |
| 10,000 | 0.107 | 93.16M | 0.105 | 95.39M | 3.587 | 33.42× | 34.22× |
| 100,000 | 1.050 | 95.24M | 1.031 | 96.96M | 40.707 | 38.77× | 39.47× |
| 1,000,000 | 11.298 | 88.52M | 10.493 | 95.30M | 449.919 | 39.82× | 42.88× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.083 | 0.267 | 3.20× |
| 1 | 5 | 0.332 | 1.217 | 3.66× |
| 1 | 10 | 0.487 | 2.245 | 4.61× |
| 10 | 1 | 0.059 | 0.218 | 3.70× |
| 10 | 5 | 0.239 | 1.322 | 5.53× |
| 10 | 10 | 0.521 | 2.287 | 4.39× |
| 100 | 1 | 0.059 | 0.252 | 4.29× |
| 100 | 5 | 0.261 | 1.536 | 5.89× |
| 100 | 10 | 0.558 | 2.685 | 4.81× |
| 1,000 | 1 | 0.065 | 0.819 | 12.68× |
| 1,000 | 5 | 0.255 | 3.388 | 13.28× |
| 1,000 | 10 | 0.586 | 6.890 | 11.76× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
