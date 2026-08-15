# CandleThreeOutside benchmark (`CDL3OUTSIDE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 160.61M | 0.003 | 317.65M | 0.031 | 4.92× | 9.73× |
| 10,000 | 0.028 | 354.17M | 0.023 | 425.66M | 0.083 | 2.93× | 3.52× |
| 100,000 | 0.235 | 426.00M | 0.224 | 446.72M | 0.570 | 2.43× | 2.55× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.090 | 0.103 | 1.14× |
| 1 | 5 | 0.240 | 0.431 | 1.80× |
| 1 | 10 | 0.395 | 0.900 | 2.28× |
| 10 | 1 | 0.041 | 0.088 | 2.16× |
| 10 | 5 | 0.200 | 0.441 | 2.20× |
| 10 | 10 | 0.398 | 0.929 | 2.33× |
| 100 | 1 | 0.049 | 0.097 | 1.99× |
| 100 | 5 | 0.183 | 0.438 | 2.39× |
| 100 | 10 | 0.412 | 0.920 | 2.23× |
| 1,000 | 1 | 0.044 | 0.096 | 2.18× |
| 1,000 | 5 | 0.196 | 0.491 | 2.50× |
| 1,000 | 10 | 0.420 | 0.982 | 2.33× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
