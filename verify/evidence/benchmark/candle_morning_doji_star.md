# CandleMorningDojiStar benchmark (`CDLMORNINGDOJISTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 147.29M | 0.004 | 263.41M | 0.040 | 5.90× | 10.56× |
| 10,000 | 0.075 | 132.83M | 0.073 | 136.76M | 0.113 | 1.49× | 1.54× |
| 100,000 | 0.859 | 116.43M | 0.824 | 121.40M | 0.808 | 0.94× | 0.98× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.119 | 0.117 | 0.98× |
| 1 | 5 | 0.361 | 0.495 | 1.37× |
| 1 | 10 | 0.376 | 0.936 | 2.49× |
| 10 | 1 | 0.040 | 0.097 | 2.46× |
| 10 | 5 | 0.176 | 0.447 | 2.54× |
| 10 | 10 | 0.382 | 0.956 | 2.50× |
| 100 | 1 | 0.047 | 0.100 | 2.12× |
| 100 | 5 | 0.174 | 0.453 | 2.60× |
| 100 | 10 | 0.371 | 0.934 | 2.52× |
| 1,000 | 1 | 0.051 | 0.102 | 2.01× |
| 1,000 | 5 | 0.182 | 0.516 | 2.84× |
| 1,000 | 10 | 0.408 | 1.002 | 2.46× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
