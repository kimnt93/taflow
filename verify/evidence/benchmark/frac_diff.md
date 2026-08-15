# FracDiff benchmark (`fixed-width fractional differencing` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.072 | 13.90M | 0.072 | 13.95M | 0.294 | 4.08× | 4.09× |
| 10,000 | 7.366 | 1.36M | 7.589 | 1.32M | 7.849 | 1.07× | 1.03× |
| 100,000 | 81.522 | 1.23M | 82.288 | 1.22M | 93.408 | 1.15× | 1.14× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.090 | 0.365 | 4.07× |
| 1 | 5 | 0.351 | 1.466 | 4.18× |
| 1 | 10 | 0.466 | 2.879 | 6.18× |
| 10 | 1 | 0.052 | 0.283 | 5.44× |
| 10 | 5 | 0.229 | 1.386 | 6.05× |
| 10 | 10 | 0.481 | 2.900 | 6.03× |
| 100 | 1 | 0.055 | 0.276 | 5.05× |
| 100 | 5 | 0.245 | 1.446 | 5.89× |
| 100 | 10 | 0.487 | 2.795 | 5.74× |
| 1,000 | 1 | 0.119 | 0.418 | 3.52× |
| 1,000 | 5 | 0.265 | 1.910 | 7.20× |
| 1,000 | 10 | 0.513 | 3.952 | 7.70× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
