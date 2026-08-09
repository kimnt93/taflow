# CandleCounterAttack benchmark (`CDLCOUNTERATTACK` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 97.91M | 0.009 | 117.33M | 0.036 | 3.50× | 4.20× |
| 10,000 | 0.071 | 141.57M | 0.070 | 142.15M | 0.137 | 1.94× | 1.94× |
| 100,000 | 0.984 | 101.63M | 0.979 | 102.16M | 1.152 | 1.17× | 1.18× |
| 1,000,000 | 10.234 | 97.71M | 10.000 | 100.00M | 10.612 | 1.04× | 1.06× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.101 | 0.110 | 1.09× |
| 1 | 5 | 0.338 | 0.565 | 1.67× |
| 1 | 10 | 0.509 | 0.932 | 1.83× |
| 10 | 1 | 0.052 | 0.089 | 1.70× |
| 10 | 5 | 0.239 | 0.428 | 1.79× |
| 10 | 10 | 0.518 | 0.914 | 1.77× |
| 100 | 1 | 0.054 | 0.091 | 1.69× |
| 100 | 5 | 0.257 | 0.441 | 1.71× |
| 100 | 10 | 0.528 | 0.927 | 1.76× |
| 1,000 | 1 | 0.061 | 0.101 | 1.64× |
| 1,000 | 5 | 0.263 | 0.509 | 1.93× |
| 1,000 | 10 | 0.541 | 1.065 | 1.97× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
