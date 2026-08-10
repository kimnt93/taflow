# CandleTakuri benchmark (`CDLTAKURI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.017 | 57.19M | 0.014 | 71.59M | 0.035 | 2.00× | 2.51× |
| 10,000 | 0.109 | 91.61M | 0.107 | 93.31M | 0.101 | 0.93× | 0.94× |
| 100,000 | 1.087 | 91.99M | 1.041 | 96.06M | 0.797 | 0.73× | 0.77× |
| 1,000,000 | 11.600 | 86.21M | 11.172 | 89.51M | 7.747 | 0.67× | 0.69× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.087 | 0.131 | 1.52× |
| 1 | 5 | 0.367 | 0.458 | 1.25× |
| 1 | 10 | 0.523 | 0.910 | 1.74× |
| 10 | 1 | 0.053 | 0.093 | 1.77× |
| 10 | 5 | 0.245 | 0.424 | 1.73× |
| 10 | 10 | 0.536 | 0.965 | 1.80× |
| 100 | 1 | 0.056 | 0.101 | 1.83× |
| 100 | 5 | 0.269 | 0.459 | 1.71× |
| 100 | 10 | 0.572 | 0.962 | 1.68× |
| 1,000 | 1 | 0.065 | 0.097 | 1.50× |
| 1,000 | 5 | 0.263 | 0.472 | 1.79× |
| 1,000 | 10 | 0.561 | 1.030 | 1.83× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
