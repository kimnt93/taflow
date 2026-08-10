# CandlePiercing benchmark (`CDLPIERCING` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.017 | 58.23M | 0.014 | 70.76M | 0.035 | 2.02× | 2.46× |
| 10,000 | 0.125 | 80.12M | 0.120 | 83.21M | 0.117 | 0.93× | 0.97× |
| 100,000 | 1.222 | 81.80M | 1.176 | 85.04M | 0.956 | 0.78× | 0.81× |
| 1,000,000 | 13.103 | 76.32M | 12.322 | 81.15M | 9.401 | 0.72× | 0.76× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.101 | 0.126 | 1.25× |
| 1 | 5 | 0.395 | 0.445 | 1.13× |
| 1 | 10 | 0.491 | 0.906 | 1.84× |
| 10 | 1 | 0.055 | 0.088 | 1.61× |
| 10 | 5 | 0.248 | 0.416 | 1.68× |
| 10 | 10 | 0.519 | 0.902 | 1.74× |
| 100 | 1 | 0.053 | 0.093 | 1.74× |
| 100 | 5 | 0.276 | 0.460 | 1.67× |
| 100 | 10 | 0.570 | 0.946 | 1.66× |
| 1,000 | 1 | 0.065 | 0.098 | 1.49× |
| 1,000 | 5 | 0.282 | 0.497 | 1.76× |
| 1,000 | 10 | 0.605 | 1.034 | 1.71× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
