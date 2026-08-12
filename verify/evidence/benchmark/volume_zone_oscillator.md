# VolumeZoneOscillator benchmark (`VZO` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.016 | 61.01M | 0.016 | 61.37M | 0.227 | 13.87× | 13.95× |
| 10,000 | 0.128 | 78.17M | 0.125 | 79.96M | 0.911 | 7.12× | 7.28× |
| 100,000 | 1.200 | 83.35M | 1.188 | 84.15M | 7.672 | 6.39× | 6.46× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.122 | 0.271 | 2.22× |
| 1 | 5 | 0.330 | 1.075 | 3.25× |
| 1 | 10 | 0.534 | 2.420 | 4.53× |
| 10 | 1 | 0.052 | 0.212 | 4.06× |
| 10 | 5 | 0.249 | 1.311 | 5.26× |
| 10 | 10 | 0.563 | 2.380 | 4.22× |
| 100 | 1 | 0.059 | 0.231 | 3.89× |
| 100 | 5 | 0.254 | 1.445 | 5.69× |
| 100 | 10 | 0.548 | 2.437 | 4.45× |
| 1,000 | 1 | 0.072 | 0.310 | 4.31× |
| 1,000 | 5 | 0.289 | 1.712 | 5.93× |
| 1,000 | 10 | 0.562 | 3.261 | 5.81× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
