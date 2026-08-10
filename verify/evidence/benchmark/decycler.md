# Decycler benchmark (`Decycler` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 97.39M | 0.009 | 111.18M | 0.146 | 14.23× | 16.25× |
| 10,000 | 0.070 | 142.87M | 0.070 | 142.14M | 0.488 | 6.98× | 6.94× |
| 100,000 | 0.673 | 148.59M | 0.633 | 157.86M | 3.858 | 5.73× | 6.09× |
| 1,000,000 | 7.312 | 136.76M | 6.398 | 156.30M | 38.461 | 5.26× | 6.01× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.107 | 0.244 | 2.28× |
| 1 | 5 | 0.352 | 0.977 | 2.78× |
| 1 | 10 | 0.505 | 2.069 | 4.10× |
| 10 | 1 | 0.063 | 0.228 | 3.62× |
| 10 | 5 | 0.231 | 0.941 | 4.08× |
| 10 | 10 | 0.485 | 2.079 | 4.29× |
| 100 | 1 | 0.054 | 0.191 | 3.53× |
| 100 | 5 | 0.242 | 0.989 | 4.09× |
| 100 | 10 | 0.496 | 2.117 | 4.27× |
| 1,000 | 1 | 0.059 | 0.231 | 3.89× |
| 1,000 | 5 | 0.242 | 1.161 | 4.79× |
| 1,000 | 10 | 0.516 | 2.457 | 4.77× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
