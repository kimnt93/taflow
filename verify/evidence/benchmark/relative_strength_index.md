# RelativeStrengthIndex benchmark (`RSI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.042 | 24.08M | 0.037 | 26.76M | 0.038 | 0.91× | 1.02× |
| 10,000 | 0.305 | 32.75M | 0.298 | 33.51M | 0.083 | 0.27× | 0.28× |
| 100,000 | 2.994 | 33.40M | 2.809 | 35.60M | 0.558 | 0.19× | 0.20× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.108 | 0.130 | 1.20× |
| 1 | 5 | 0.522 | 0.476 | 0.91× |
| 1 | 10 | 0.552 | 0.957 | 1.73× |
| 10 | 1 | 0.070 | 0.088 | 1.25× |
| 10 | 5 | 0.289 | 0.444 | 1.54× |
| 10 | 10 | 0.588 | 0.898 | 1.53× |
| 100 | 1 | 0.064 | 0.090 | 1.41× |
| 100 | 5 | 0.284 | 0.445 | 1.56× |
| 100 | 10 | 0.614 | 0.932 | 1.52× |
| 1,000 | 1 | 0.093 | 0.097 | 1.04× |
| 1,000 | 5 | 0.287 | 0.464 | 1.62× |
| 1,000 | 10 | 0.587 | 0.979 | 1.67× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
