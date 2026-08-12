# OnBalanceVolume benchmark (`OBV` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 111.15M | 0.008 | 130.72M | 0.032 | 3.54× | 4.17× |
| 10,000 | 0.065 | 154.63M | 0.059 | 169.83M | 0.067 | 1.04× | 1.14× |
| 100,000 | 0.656 | 152.43M | 0.579 | 172.85M | 0.402 | 0.61× | 0.70× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.077 | 0.106 | 1.38× |
| 1 | 5 | 0.213 | 0.461 | 2.17× |
| 1 | 10 | 0.483 | 0.972 | 2.01× |
| 10 | 1 | 0.062 | 0.121 | 1.95× |
| 10 | 5 | 0.265 | 0.483 | 1.82× |
| 10 | 10 | 0.500 | 0.988 | 1.98× |
| 100 | 1 | 0.050 | 0.091 | 1.81× |
| 100 | 5 | 0.269 | 0.511 | 1.90× |
| 100 | 10 | 0.469 | 0.931 | 1.99× |
| 1,000 | 1 | 0.061 | 0.096 | 1.58× |
| 1,000 | 5 | 0.235 | 0.453 | 1.93× |
| 1,000 | 10 | 0.591 | 0.974 | 1.65× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
