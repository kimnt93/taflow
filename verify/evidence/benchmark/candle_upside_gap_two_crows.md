# CandleUpsideGapTwoCrows benchmark (`CDLUPSIDEGAP2CROWS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 163.49M | 0.003 | 372.50M | 0.033 | 5.42× | 12.35× |
| 10,000 | 0.048 | 206.70M | 0.041 | 241.67M | 0.121 | 2.51× | 2.93× |
| 100,000 | 0.671 | 149.02M | 0.670 | 149.30M | 0.977 | 1.46× | 1.46× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.134 | 0.117 | 0.87× |
| 1 | 5 | 0.221 | 0.461 | 2.08× |
| 1 | 10 | 0.406 | 0.934 | 2.30× |
| 10 | 1 | 0.043 | 0.085 | 1.94× |
| 10 | 5 | 0.184 | 0.419 | 2.28× |
| 10 | 10 | 0.421 | 1.000 | 2.38× |
| 100 | 1 | 0.047 | 0.097 | 2.08× |
| 100 | 5 | 0.181 | 0.461 | 2.55× |
| 100 | 10 | 0.397 | 0.937 | 2.36× |
| 1,000 | 1 | 0.055 | 0.106 | 1.93× |
| 1,000 | 5 | 0.201 | 0.490 | 2.44× |
| 1,000 | 10 | 0.432 | 1.028 | 2.38× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
