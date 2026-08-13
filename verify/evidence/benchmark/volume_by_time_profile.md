# VolumeByTimeProfile benchmark (`VolumeByTimeProfile` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.218 | 4.59M | 0.194 | 5.16M | 1.530 | 7.02× | 7.90× |
| 10,000 | 2.005 | 4.99M | 1.828 | 5.47M | 14.292 | 7.13× | 7.82× |
| 100,000 | 19.117 | 5.23M | 17.647 | 5.67M | 188.195 | 9.84× | 10.66× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.175 | 0.301 | 1.72× |
| 1 | 5 | 0.425 | 7.623 | 17.94× |
| 1 | 10 | 0.789 | 2.717 | 3.44× |
| 10 | 1 | 0.086 | 0.262 | 3.04× |
| 10 | 5 | 0.339 | 1.513 | 4.46× |
| 10 | 10 | 0.725 | 2.842 | 3.92× |
| 100 | 1 | 0.102 | 0.398 | 3.89× |
| 100 | 5 | 0.336 | 2.140 | 6.37× |
| 100 | 10 | 0.740 | 4.261 | 5.76× |
| 1,000 | 1 | 0.271 | 1.946 | 7.18× |
| 1,000 | 5 | 0.509 | 9.383 | 18.43× |
| 1,000 | 10 | 0.894 | 19.114 | 21.37× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
