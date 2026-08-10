# AveragePrice benchmark (`AVGPRICE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 90.78M | 0.009 | 113.03M | 0.034 | 3.13× | 3.89× |
| 10,000 | 0.045 | 221.61M | 0.031 | 321.16M | 0.041 | 0.92× | 1.33× |
| 100,000 | 0.306 | 326.82M | 0.261 | 383.53M | 0.115 | 0.38× | 0.44× |
| 1,000,000 | 4.171 | 239.76M | 3.254 | 307.30M | 2.161 | 0.52× | 0.66× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.098 | 0.144 | 1.46× |
| 1 | 5 | 0.326 | 0.512 | 1.57× |
| 1 | 10 | 0.530 | 1.018 | 1.92× |
| 10 | 1 | 0.053 | 0.120 | 2.25× |
| 10 | 5 | 0.319 | 0.509 | 1.60× |
| 10 | 10 | 0.608 | 0.984 | 1.62× |
| 100 | 1 | 0.051 | 0.087 | 1.73× |
| 100 | 5 | 0.287 | 0.529 | 1.84× |
| 100 | 10 | 0.615 | 0.988 | 1.61× |
| 1,000 | 1 | 0.058 | 0.102 | 1.76× |
| 1,000 | 5 | 0.265 | 0.487 | 1.84× |
| 1,000 | 10 | 0.624 | 1.093 | 1.75× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
