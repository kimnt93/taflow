# Vortex benchmark (`Vortex` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 50.46M | 0.017 | 60.37M | 0.615 | 31.01× | 37.10× |
| 10,000 | 0.131 | 76.37M | 0.113 | 88.34M | 3.888 | 29.70× | 34.35× |
| 100,000 | 1.041 | 96.11M | 1.389 | 72.01M | 43.654 | 41.95× | 31.44× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.098 | 0.268 | 2.72× |
| 1 | 5 | 0.363 | 1.114 | 3.07× |
| 1 | 10 | 0.535 | 4.326 | 8.09× |
| 10 | 1 | 0.092 | 0.370 | 4.01× |
| 10 | 5 | 0.371 | 2.327 | 6.27× |
| 10 | 10 | 0.729 | 3.486 | 4.78× |
| 100 | 1 | 0.062 | 0.319 | 5.15× |
| 100 | 5 | 0.276 | 2.274 | 8.25× |
| 100 | 10 | 0.667 | 2.849 | 4.27× |
| 1,000 | 1 | 0.066 | 0.829 | 12.59× |
| 1,000 | 5 | 0.279 | 3.540 | 12.68× |
| 1,000 | 10 | 0.615 | 7.119 | 11.57× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
