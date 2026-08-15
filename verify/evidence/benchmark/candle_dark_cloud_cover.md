# CandleDarkCloudCover benchmark (`CDLDARKCLOUDCOVER` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 176.10M | 0.002 | 422.85M | 0.038 | 6.77× | 16.27× |
| 10,000 | 0.050 | 201.45M | 0.043 | 231.57M | 0.117 | 2.36× | 2.72× |
| 100,000 | 0.773 | 129.31M | 0.735 | 136.04M | 0.838 | 1.08× | 1.14× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.069 | 0.129 | 1.88× |
| 1 | 5 | 0.202 | 0.462 | 2.29× |
| 1 | 10 | 0.398 | 0.965 | 2.43× |
| 10 | 1 | 0.055 | 0.096 | 1.74× |
| 10 | 5 | 0.214 | 0.463 | 2.16× |
| 10 | 10 | 0.389 | 0.964 | 2.48× |
| 100 | 1 | 0.040 | 0.092 | 2.30× |
| 100 | 5 | 0.195 | 0.485 | 2.49× |
| 100 | 10 | 0.426 | 0.986 | 2.32× |
| 1,000 | 1 | 0.047 | 0.106 | 2.25× |
| 1,000 | 5 | 0.205 | 0.569 | 2.77× |
| 1,000 | 10 | 0.414 | 1.127 | 2.72× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
