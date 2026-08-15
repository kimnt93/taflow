# CandleIdenticalThreeCrows benchmark (`CDLIDENTICAL3CROWS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 152.15M | 0.003 | 315.06M | 0.036 | 5.49× | 11.37× |
| 10,000 | 0.049 | 205.22M | 0.043 | 232.85M | 0.117 | 2.40× | 2.73× |
| 100,000 | 0.607 | 164.85M | 0.574 | 174.15M | 0.922 | 1.52× | 1.61× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.096 | 0.121 | 1.26× |
| 1 | 5 | 0.215 | 0.463 | 2.15× |
| 1 | 10 | 0.407 | 0.931 | 2.29× |
| 10 | 1 | 0.047 | 0.111 | 2.35× |
| 10 | 5 | 0.202 | 0.466 | 2.30× |
| 10 | 10 | 0.446 | 0.905 | 2.03× |
| 100 | 1 | 0.042 | 0.090 | 2.17× |
| 100 | 5 | 0.225 | 0.445 | 1.98× |
| 100 | 10 | 0.434 | 0.980 | 2.26× |
| 1,000 | 1 | 0.050 | 0.101 | 2.02× |
| 1,000 | 5 | 0.210 | 0.533 | 2.54× |
| 1,000 | 10 | 0.415 | 1.067 | 2.57× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
