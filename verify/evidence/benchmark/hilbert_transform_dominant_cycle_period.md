# HilbertTransformDominantCyclePeriod benchmark (`HT_DCPERIOD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.045 | 22.23M | 0.044 | 22.68M | 0.072 | 1.59× | 1.62× |
| 10,000 | 0.454 | 22.02M | 0.457 | 21.91M | 0.463 | 1.02× | 1.01× |
| 100,000 | 4.517 | 22.14M | 4.468 | 22.38M | 4.414 | 0.98× | 0.99× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.164 | 0.132 | 0.80× |
| 1 | 5 | 0.236 | 0.446 | 1.89× |
| 1 | 10 | 0.402 | 0.953 | 2.37× |
| 10 | 1 | 0.045 | 0.093 | 2.05× |
| 10 | 5 | 0.177 | 0.420 | 2.37× |
| 10 | 10 | 0.376 | 0.953 | 2.54× |
| 100 | 1 | 0.056 | 0.096 | 1.71× |
| 100 | 5 | 0.192 | 0.432 | 2.26× |
| 100 | 10 | 0.396 | 0.920 | 2.32× |
| 1,000 | 1 | 0.096 | 0.151 | 1.57× |
| 1,000 | 5 | 0.214 | 0.653 | 3.05× |
| 1,000 | 10 | 0.486 | 1.375 | 2.83× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
