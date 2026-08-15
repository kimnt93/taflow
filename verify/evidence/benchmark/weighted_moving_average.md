# WeightedMovingAverage benchmark (`WMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 206.76M | 0.004 | 261.56M | 0.034 | 7.12× | 9.01× |
| 10,000 | 0.032 | 308.19M | 0.034 | 294.92M | 0.052 | 1.61× | 1.54× |
| 100,000 | 0.306 | 326.96M | 0.298 | 335.98M | 0.223 | 0.73× | 0.75× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.072 | 0.158 | 2.21× |
| 1 | 5 | 0.212 | 0.460 | 2.17× |
| 1 | 10 | 0.399 | 0.971 | 2.43× |
| 10 | 1 | 0.056 | 0.099 | 1.77× |
| 10 | 5 | 0.195 | 0.450 | 2.31× |
| 10 | 10 | 0.385 | 0.952 | 2.48× |
| 100 | 1 | 0.042 | 0.103 | 2.44× |
| 100 | 5 | 0.201 | 0.490 | 2.45× |
| 100 | 10 | 0.393 | 0.949 | 2.42× |
| 1,000 | 1 | 0.044 | 0.097 | 2.21× |
| 1,000 | 5 | 0.189 | 0.452 | 2.39× |
| 1,000 | 10 | 0.405 | 0.991 | 2.45× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
