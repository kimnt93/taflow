# CandleEveningDojiStar benchmark (`CDLEVENINGDOJISTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.021 | 47.58M | 0.019 | 53.67M | 0.043 | 2.04× | 2.30× |
| 10,000 | 0.166 | 60.19M | 0.165 | 60.73M | 0.128 | 0.77× | 0.78× |
| 100,000 | 1.724 | 58.00M | 1.690 | 59.17M | 0.945 | 0.55× | 0.56× |
| 1,000,000 | 17.341 | 57.67M | 16.741 | 59.73M | 9.191 | 0.53× | 0.55× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.083 | 0.140 | 1.69× |
| 1 | 5 | 0.324 | 0.522 | 1.61× |
| 1 | 10 | 0.585 | 1.098 | 1.88× |
| 10 | 1 | 0.057 | 0.093 | 1.64× |
| 10 | 5 | 0.303 | 0.513 | 1.70× |
| 10 | 10 | 0.542 | 1.013 | 1.87× |
| 100 | 1 | 0.071 | 0.107 | 1.52× |
| 100 | 5 | 0.287 | 0.520 | 1.81× |
| 100 | 10 | 0.607 | 1.013 | 1.67× |
| 1,000 | 1 | 0.078 | 0.104 | 1.33× |
| 1,000 | 5 | 0.320 | 0.576 | 1.80× |
| 1,000 | 10 | 0.652 | 1.198 | 1.84× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
