# MovingAverageConvergenceDivergenceExtended benchmark (`MACDEXT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 72.96M | 0.011 | 87.35M | 0.057 | 4.15× | 4.97× |
| 10,000 | 0.108 | 92.94M | 0.102 | 98.33M | 0.124 | 1.16× | 1.22× |
| 100,000 | 2.143 | 46.66M | 0.967 | 103.44M | 1.406 | 0.66× | 1.45× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.078 | 0.128 | 1.63× |
| 1 | 5 | 0.258 | 0.590 | 2.28× |
| 1 | 10 | 0.445 | 1.208 | 2.71× |
| 10 | 1 | 0.048 | 0.110 | 2.27× |
| 10 | 5 | 0.228 | 0.611 | 2.68× |
| 10 | 10 | 0.496 | 1.146 | 2.31× |
| 100 | 1 | 0.045 | 0.115 | 2.53× |
| 100 | 5 | 0.208 | 0.544 | 2.62× |
| 100 | 10 | 0.448 | 1.194 | 2.67× |
| 1,000 | 1 | 0.071 | 0.126 | 1.77× |
| 1,000 | 5 | 0.225 | 0.583 | 2.60× |
| 1,000 | 10 | 0.458 | 1.274 | 2.78× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
