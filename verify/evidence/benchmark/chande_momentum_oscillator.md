# ChandeMomentumOscillator benchmark (`CMO` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 110.96M | 0.008 | 124.79M | 0.040 | 4.49× | 5.04× |
| 10,000 | 0.060 | 165.91M | 0.059 | 168.29M | 0.096 | 1.60× | 1.62× |
| 100,000 | 0.579 | 172.83M | 0.565 | 177.01M | 0.614 | 1.06× | 1.09× |
| 1,000,000 | 6.656 | 150.24M | 5.819 | 171.84M | 6.102 | 0.92× | 1.05× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.108 | 0.123 | 1.14× |
| 1 | 5 | 0.238 | 0.483 | 2.03× |
| 1 | 10 | 0.469 | 0.989 | 2.11× |
| 10 | 1 | 0.047 | 0.097 | 2.07× |
| 10 | 5 | 0.237 | 0.452 | 1.91× |
| 10 | 10 | 0.469 | 1.005 | 2.14× |
| 100 | 1 | 0.048 | 0.105 | 2.18× |
| 100 | 5 | 0.233 | 0.463 | 1.98× |
| 100 | 10 | 0.480 | 1.042 | 2.17× |
| 1,000 | 1 | 0.055 | 0.108 | 1.95× |
| 1,000 | 5 | 0.228 | 0.543 | 2.38× |
| 1,000 | 10 | 0.499 | 1.058 | 2.12× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
