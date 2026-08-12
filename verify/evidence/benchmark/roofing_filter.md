# RoofingFilter benchmark (`RoofingFilter` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 74.09M | 0.008 | 130.17M | 0.197 | 14.63× | 25.71× |
| 10,000 | 0.053 | 187.65M | 0.050 | 198.41M | 0.532 | 9.99× | 10.56× |
| 100,000 | 0.478 | 209.03M | 0.452 | 221.09M | 3.855 | 8.06× | 8.52× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.129 | 0.267 | 2.07× |
| 1 | 5 | 0.247 | 1.344 | 5.43× |
| 1 | 10 | 0.471 | 2.667 | 5.66× |
| 10 | 1 | 0.053 | 0.250 | 4.74× |
| 10 | 5 | 0.238 | 1.496 | 6.30× |
| 10 | 10 | 0.513 | 2.658 | 5.18× |
| 100 | 1 | 0.055 | 0.259 | 4.72× |
| 100 | 5 | 0.249 | 1.488 | 5.98× |
| 100 | 10 | 0.514 | 2.754 | 5.36× |
| 1,000 | 1 | 0.059 | 0.275 | 4.63× |
| 1,000 | 5 | 0.256 | 1.579 | 6.16× |
| 1,000 | 10 | 0.539 | 2.933 | 5.45× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
