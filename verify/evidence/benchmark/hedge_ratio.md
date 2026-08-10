# HedgeRatio benchmark (`rolling OLS hedge ratio` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.041 | 24.30M | 0.040 | 24.80M | 0.257 | 6.26× | 6.38× |
| 10,000 | 0.360 | 27.74M | 0.357 | 28.03M | 1.422 | 3.94× | 3.99× |
| 100,000 | 3.884 | 25.74M | 3.934 | 25.42M | 20.207 | 5.20× | 5.14× |
| 1,000,000 | 37.763 | 26.48M | 35.778 | 27.95M | 199.834 | 5.29× | 5.59× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.069 | 0.175 | 2.54× |
| 1 | 5 | 0.232 | 0.681 | 2.93× |
| 1 | 10 | 0.483 | 1.299 | 2.69× |
| 10 | 1 | 0.053 | 0.122 | 2.29× |
| 10 | 5 | 0.221 | 0.633 | 2.86× |
| 10 | 10 | 0.502 | 1.282 | 2.55× |
| 100 | 1 | 0.056 | 0.211 | 3.79× |
| 100 | 5 | 0.228 | 1.194 | 5.24× |
| 100 | 10 | 0.492 | 2.258 | 4.59× |
| 1,000 | 1 | 0.085 | 0.353 | 4.17× |
| 1,000 | 5 | 0.251 | 1.367 | 5.45× |
| 1,000 | 10 | 0.535 | 2.734 | 5.11× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
