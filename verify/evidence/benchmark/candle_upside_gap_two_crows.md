# CandleUpsideGapTwoCrows benchmark (`CDLUPSIDEGAP2CROWS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 181.25M | 0.002 | 412.01M | 0.035 | 6.26× | 14.24× |
| 10,000 | 0.048 | 206.78M | 0.042 | 240.08M | 0.121 | 2.51× | 2.91× |
| 100,000 | 0.660 | 151.42M | 0.615 | 162.68M | 0.950 | 1.44× | 1.55× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.102 | 0.149 | 1.46× |
| 1 | 5 | 0.269 | 0.447 | 1.66× |
| 1 | 10 | 0.380 | 0.896 | 2.36× |
| 10 | 1 | 0.043 | 0.085 | 2.00× |
| 10 | 5 | 0.173 | 0.424 | 2.45× |
| 10 | 10 | 0.397 | 0.891 | 2.24× |
| 100 | 1 | 0.042 | 0.084 | 2.00× |
| 100 | 5 | 0.177 | 0.408 | 2.30× |
| 100 | 10 | 0.387 | 0.899 | 2.32× |
| 1,000 | 1 | 0.047 | 0.094 | 1.99× |
| 1,000 | 5 | 0.184 | 0.456 | 2.48× |
| 1,000 | 10 | 0.415 | 0.961 | 2.31× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
