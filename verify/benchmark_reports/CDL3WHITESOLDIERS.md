# CandleThreeWhiteSoldiers benchmark (`CDL3WHITESOLDIERS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 99.12M | 0.009 | 115.83M | 0.047 | 4.62× | 5.39× |
| 10,000 | 0.073 | 136.54M | 0.071 | 139.92M | 0.184 | 2.51× | 2.57× |
| 100,000 | 0.768 | 130.26M | 0.769 | 130.12M | 1.602 | 2.09× | 2.09× |
| 1,000,000 | 7.542 | 132.60M | 7.595 | 131.66M | 14.892 | 1.97× | 1.96× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.103 | 0.106 | 1.03× |
| 1 | 5 | 0.319 | 0.506 | 1.59× |
| 1 | 10 | 0.516 | 0.915 | 1.77× |
| 10 | 1 | 0.055 | 0.090 | 1.64× |
| 10 | 5 | 0.236 | 0.429 | 1.82× |
| 10 | 10 | 0.511 | 0.929 | 1.82× |
| 100 | 1 | 0.059 | 0.093 | 1.59× |
| 100 | 5 | 0.255 | 0.449 | 1.76× |
| 100 | 10 | 0.544 | 0.947 | 1.74× |
| 1,000 | 1 | 0.071 | 0.109 | 1.55× |
| 1,000 | 5 | 0.281 | 0.542 | 1.93× |
| 1,000 | 10 | 0.555 | 1.120 | 2.02× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
