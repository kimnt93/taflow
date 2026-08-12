# HeikinAshi benchmark (`HeikinAshi` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 68.82M | 0.011 | 88.96M | 0.556 | 38.24× | 49.43× |
| 10,000 | 0.084 | 119.50M | 0.073 | 136.39M | 4.540 | 54.26× | 61.93× |
| 100,000 | 0.808 | 123.79M | 0.846 | 118.24M | 51.219 | 63.41× | 60.56× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.081 | 0.233 | 2.87× |
| 1 | 5 | 0.332 | 0.988 | 2.98× |
| 1 | 10 | 0.497 | 2.031 | 4.09× |
| 10 | 1 | 0.064 | 0.195 | 3.06× |
| 10 | 5 | 0.250 | 0.934 | 3.74× |
| 10 | 10 | 0.512 | 2.196 | 4.29× |
| 100 | 1 | 0.051 | 0.225 | 4.41× |
| 100 | 5 | 0.250 | 1.146 | 4.58× |
| 100 | 10 | 0.515 | 2.589 | 5.02× |
| 1,000 | 1 | 0.067 | 0.791 | 11.83× |
| 1,000 | 5 | 0.238 | 3.672 | 15.45× |
| 1,000 | 10 | 0.532 | 7.379 | 13.88× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
