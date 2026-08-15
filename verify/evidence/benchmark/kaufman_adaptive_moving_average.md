# KaufmanAdaptiveMovingAverage benchmark (`KAMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 213.83M | 0.004 | 248.81M | 0.036 | 7.78× | 9.06× |
| 10,000 | 0.031 | 317.60M | 0.029 | 340.84M | 0.064 | 2.03× | 2.18× |
| 100,000 | 0.305 | 327.43M | 0.291 | 343.64M | 0.338 | 1.11× | 1.16× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.091 | 0.116 | 1.27× |
| 1 | 5 | 0.294 | 0.491 | 1.67× |
| 1 | 10 | 0.416 | 0.968 | 2.33× |
| 10 | 1 | 0.043 | 0.093 | 2.19× |
| 10 | 5 | 0.192 | 0.461 | 2.40× |
| 10 | 10 | 0.399 | 0.986 | 2.47× |
| 100 | 1 | 0.046 | 0.095 | 2.06× |
| 100 | 5 | 0.185 | 0.445 | 2.40× |
| 100 | 10 | 0.381 | 0.940 | 2.47× |
| 1,000 | 1 | 0.050 | 0.100 | 2.00× |
| 1,000 | 5 | 0.203 | 0.461 | 2.27× |
| 1,000 | 10 | 0.436 | 0.963 | 2.21× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
