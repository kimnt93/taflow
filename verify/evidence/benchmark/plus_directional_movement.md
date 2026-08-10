# PlusDirectionalMovement benchmark (`PLUS_DM` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 107.93M | 0.008 | 130.30M | 0.036 | 3.88× | 4.68× |
| 10,000 | 0.057 | 176.93M | 0.053 | 187.73M | 0.080 | 1.42× | 1.50× |
| 100,000 | 0.511 | 195.82M | 0.492 | 203.05M | 0.514 | 1.01× | 1.04× |
| 1,000,000 | 5.526 | 180.97M | 4.896 | 204.24M | 5.073 | 0.92× | 1.04× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.136 | 0.127 | 0.94× |
| 1 | 5 | 0.264 | 0.488 | 1.85× |
| 1 | 10 | 0.502 | 1.021 | 2.03× |
| 10 | 1 | 0.060 | 0.101 | 1.68× |
| 10 | 5 | 0.230 | 0.453 | 1.97× |
| 10 | 10 | 0.472 | 0.927 | 1.96× |
| 100 | 1 | 0.048 | 0.095 | 1.97× |
| 100 | 5 | 0.213 | 0.451 | 2.11× |
| 100 | 10 | 0.503 | 0.978 | 1.94× |
| 1,000 | 1 | 0.054 | 0.092 | 1.71× |
| 1,000 | 5 | 0.236 | 0.479 | 2.02× |
| 1,000 | 10 | 0.473 | 1.004 | 2.12× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
