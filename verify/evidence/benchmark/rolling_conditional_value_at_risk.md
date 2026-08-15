# RollingConditionalValueAtRisk benchmark (`ConditionalValueAtRisk` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.128 | 7.81M | 0.128 | 7.82M | 0.346 | 2.70× | 2.70× |
| 10,000 | 1.321 | 7.57M | 1.328 | 7.53M | 1.774 | 1.34× | 1.34× |
| 100,000 | 13.255 | 7.54M | 12.737 | 7.85M | 16.514 | 1.25× | 1.30× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.068 | 0.307 | 4.51× |
| 1 | 5 | 0.312 | 1.224 | 3.92× |
| 1 | 10 | 0.410 | 2.639 | 6.44× |
| 10 | 1 | 0.043 | 0.236 | 5.50× |
| 10 | 5 | 0.201 | 1.141 | 5.66× |
| 10 | 10 | 0.381 | 2.491 | 6.54× |
| 100 | 1 | 0.057 | 0.250 | 4.42× |
| 100 | 5 | 0.220 | 1.432 | 6.50× |
| 100 | 10 | 0.423 | 2.755 | 6.52× |
| 1,000 | 1 | 0.187 | 0.425 | 2.27× |
| 1,000 | 5 | 0.372 | 2.305 | 6.20× |
| 1,000 | 10 | 0.570 | 4.309 | 7.56× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
