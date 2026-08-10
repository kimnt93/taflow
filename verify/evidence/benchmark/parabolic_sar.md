# ParabolicSar benchmark (`SAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 75.10M | 0.014 | 71.10M | 0.065 | 4.84× | 4.59× |
| 10,000 | 0.131 | 76.15M | 0.135 | 74.29M | 0.109 | 0.83× | 0.81× |
| 100,000 | 1.310 | 76.33M | 1.348 | 74.17M | 0.730 | 0.56× | 0.54× |
| 1,000,000 | 12.639 | 79.12M | 11.631 | 85.98M | 6.646 | 0.53× | 0.57× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.066 | 0.140 | 2.12× |
| 1 | 5 | 0.322 | 0.534 | 1.66× |
| 1 | 10 | 0.524 | 1.126 | 2.15× |
| 10 | 1 | 0.054 | 0.100 | 1.86× |
| 10 | 5 | 0.252 | 0.480 | 1.91× |
| 10 | 10 | 0.554 | 1.055 | 1.90× |
| 100 | 1 | 0.059 | 0.098 | 1.66× |
| 100 | 5 | 0.254 | 0.502 | 1.98× |
| 100 | 10 | 0.535 | 1.090 | 2.04× |
| 1,000 | 1 | 0.063 | 0.104 | 1.65× |
| 1,000 | 5 | 0.285 | 0.540 | 1.90× |
| 1,000 | 10 | 0.524 | 1.177 | 2.25× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
