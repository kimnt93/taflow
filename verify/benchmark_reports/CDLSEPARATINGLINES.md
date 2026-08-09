# CandleSeparatingLines benchmark (`CDLSEPARATINGLINES` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 85.46M | 0.011 | 94.91M | 0.036 | 3.07× | 3.40× |
| 10,000 | 0.071 | 140.25M | 0.070 | 143.29M | 0.131 | 1.84× | 1.88× |
| 100,000 | 0.761 | 131.36M | 0.743 | 134.64M | 0.994 | 1.31× | 1.34× |
| 1,000,000 | 7.561 | 132.26M | 7.947 | 125.83M | 9.761 | 1.29× | 1.23× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.125 | 0.107 | 0.85× |
| 1 | 5 | 0.407 | 0.584 | 1.43× |
| 1 | 10 | 0.564 | 0.994 | 1.76× |
| 10 | 1 | 0.053 | 0.088 | 1.66× |
| 10 | 5 | 0.240 | 0.433 | 1.81× |
| 10 | 10 | 0.513 | 0.922 | 1.80× |
| 100 | 1 | 0.055 | 0.091 | 1.64× |
| 100 | 5 | 0.247 | 0.437 | 1.77× |
| 100 | 10 | 0.509 | 0.921 | 1.81× |
| 1,000 | 1 | 0.063 | 0.104 | 1.66× |
| 1,000 | 5 | 0.264 | 0.495 | 1.88× |
| 1,000 | 10 | 0.549 | 1.040 | 1.89× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
