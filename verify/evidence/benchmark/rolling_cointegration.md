# RollingCointegration benchmark (`Cointegration` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.331 | 3.02M | 0.332 | 3.01M | 3.255 | 9.84× | 9.81× |
| 10,000 | 3.178 | 3.15M | 3.223 | 3.10M | 31.479 | 9.91× | 9.77× |
| 100,000 | 38.874 | 2.57M | 38.230 | 2.62M | 355.701 | 9.15× | 9.30× |
| 1,000,000 | 334.396 | 2.99M | 329.395 | 3.04M | 3233.381 | 9.67× | 9.82× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.072 | 0.290 | 4.02× |
| 1 | 5 | 0.324 | 1.429 | 4.41× |
| 1 | 10 | 0.518 | 2.565 | 4.96× |
| 10 | 1 | 0.055 | 0.257 | 4.65× |
| 10 | 5 | 0.287 | 1.486 | 5.17× |
| 10 | 10 | 0.520 | 2.761 | 5.31× |
| 100 | 1 | 0.086 | 0.490 | 5.69× |
| 100 | 5 | 0.259 | 2.676 | 10.35× |
| 100 | 10 | 0.579 | 5.221 | 9.02× |
| 1,000 | 1 | 0.402 | 3.720 | 9.27× |
| 1,000 | 5 | 0.837 | 18.408 | 22.00× |
| 1,000 | 10 | 1.093 | 36.967 | 33.82× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
