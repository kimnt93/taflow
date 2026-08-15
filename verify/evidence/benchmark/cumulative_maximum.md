# CumulativeMaximum benchmark (`numpy.maximum.accumulate` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 239.76M | 0.003 | 288.69M | 0.015 | 3.70× | 4.46× |
| 10,000 | 0.031 | 326.20M | 0.028 | 356.30M | 0.040 | 1.30× | 1.42× |
| 100,000 | 0.292 | 342.42M | 0.266 | 375.90M | 0.292 | 1.00× | 1.10× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.056 | 0.071 | 1.25× |
| 1 | 5 | 0.316 | 0.321 | 1.02× |
| 1 | 10 | 0.389 | 0.653 | 1.68× |
| 10 | 1 | 0.045 | 0.056 | 1.26× |
| 10 | 5 | 0.188 | 0.290 | 1.54× |
| 10 | 10 | 0.406 | 0.585 | 1.44× |
| 100 | 1 | 0.042 | 0.059 | 1.41× |
| 100 | 5 | 0.178 | 0.294 | 1.65× |
| 100 | 10 | 0.395 | 0.598 | 1.51× |
| 1,000 | 1 | 0.043 | 0.066 | 1.52× |
| 1,000 | 5 | 0.190 | 0.313 | 1.64× |
| 1,000 | 10 | 0.445 | 0.744 | 1.67× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
