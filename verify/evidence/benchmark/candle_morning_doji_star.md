# CandleMorningDojiStar benchmark (`CDLMORNINGDOJISTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 140.06M | 0.004 | 266.69M | 0.040 | 5.56× | 10.58× |
| 10,000 | 0.060 | 165.56M | 0.060 | 167.38M | 0.130 | 2.15× | 2.18× |
| 100,000 | 0.906 | 110.40M | 0.907 | 110.28M | 0.890 | 0.98× | 0.98× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.079 | 0.160 | 2.03× |
| 1 | 5 | 0.272 | 0.492 | 1.81× |
| 1 | 10 | 0.410 | 1.001 | 2.44× |
| 10 | 1 | 0.055 | 0.093 | 1.71× |
| 10 | 5 | 0.191 | 0.547 | 2.87× |
| 10 | 10 | 0.465 | 1.033 | 2.22× |
| 100 | 1 | 0.048 | 0.101 | 2.11× |
| 100 | 5 | 0.176 | 0.450 | 2.55× |
| 100 | 10 | 0.459 | 1.053 | 2.29× |
| 1,000 | 1 | 0.067 | 0.121 | 1.82× |
| 1,000 | 5 | 0.217 | 0.519 | 2.39× |
| 1,000 | 10 | 0.423 | 1.175 | 2.77× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
