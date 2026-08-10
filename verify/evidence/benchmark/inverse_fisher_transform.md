# InverseFisherTransform benchmark (`InverseFisherTransform` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 157.26M | 0.006 | 170.01M | 0.163 | 25.67× | 27.76× |
| 10,000 | 0.038 | 263.31M | 0.036 | 275.25M | 0.442 | 11.65× | 12.17× |
| 100,000 | 0.356 | 280.85M | 0.325 | 308.08M | 3.317 | 9.32× | 10.22× |
| 1,000,000 | 3.596 | 278.12M | 3.250 | 307.69M | 33.650 | 9.36× | 10.35× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.170 | 0.283 | 1.67× |
| 1 | 5 | 0.288 | 1.207 | 4.19× |
| 1 | 10 | 0.503 | 2.200 | 4.37× |
| 10 | 1 | 0.048 | 0.208 | 4.34× |
| 10 | 5 | 0.224 | 1.203 | 5.37× |
| 10 | 10 | 0.462 | 2.197 | 4.76× |
| 100 | 1 | 0.050 | 0.213 | 4.23× |
| 100 | 5 | 0.225 | 1.202 | 5.34× |
| 100 | 10 | 0.486 | 2.233 | 4.60× |
| 1,000 | 1 | 0.055 | 0.240 | 4.34× |
| 1,000 | 5 | 0.238 | 1.364 | 5.73× |
| 1,000 | 10 | 0.484 | 2.538 | 5.25× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
