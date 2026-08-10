# OnBalanceVolume benchmark (`OBV` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 125.14M | 0.007 | 138.96M | 0.036 | 4.56× | 5.07× |
| 10,000 | 0.058 | 173.43M | 0.054 | 186.66M | 0.062 | 1.08× | 1.16× |
| 100,000 | 0.564 | 177.46M | 0.516 | 193.82M | 0.370 | 0.66× | 0.72× |
| 1,000,000 | 5.918 | 168.96M | 5.512 | 181.42M | 3.779 | 0.64× | 0.69× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.134 | 0.140 | 1.04× |
| 1 | 5 | 0.288 | 0.442 | 1.54× |
| 1 | 10 | 0.495 | 0.914 | 1.85× |
| 10 | 1 | 0.062 | 0.102 | 1.65× |
| 10 | 5 | 0.242 | 0.435 | 1.79× |
| 10 | 10 | 0.489 | 0.907 | 1.85× |
| 100 | 1 | 0.048 | 0.090 | 1.87× |
| 100 | 5 | 0.214 | 0.447 | 2.09× |
| 100 | 10 | 0.449 | 0.888 | 1.98× |
| 1,000 | 1 | 0.053 | 0.096 | 1.82× |
| 1,000 | 5 | 0.233 | 0.462 | 1.98× |
| 1,000 | 10 | 0.520 | 0.985 | 1.90× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
