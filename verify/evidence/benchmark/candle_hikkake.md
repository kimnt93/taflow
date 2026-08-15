# CandleHikkake benchmark (`CDLHIKKAKE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 160.85M | 0.003 | 343.89M | 0.032 | 5.16× | 11.03× |
| 10,000 | 0.056 | 178.14M | 0.050 | 200.68M | 0.077 | 1.37× | 1.54× |
| 100,000 | 0.588 | 170.05M | 0.580 | 172.32M | 0.514 | 0.87× | 0.89× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.176 | 0.118 | 0.67× |
| 1 | 5 | 0.224 | 0.435 | 1.94× |
| 1 | 10 | 0.435 | 0.952 | 2.19× |
| 10 | 1 | 0.040 | 0.085 | 2.11× |
| 10 | 5 | 0.217 | 0.553 | 2.55× |
| 10 | 10 | 0.399 | 0.938 | 2.35× |
| 100 | 1 | 0.047 | 0.091 | 1.95× |
| 100 | 5 | 0.211 | 0.446 | 2.12× |
| 100 | 10 | 0.433 | 1.004 | 2.32× |
| 1,000 | 1 | 0.053 | 0.096 | 1.80× |
| 1,000 | 5 | 0.213 | 0.464 | 2.18× |
| 1,000 | 10 | 0.460 | 1.044 | 2.27× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
