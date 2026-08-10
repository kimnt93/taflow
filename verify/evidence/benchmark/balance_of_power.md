# BalanceOfPower benchmark (`BOP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 120.86M | 0.007 | 149.92M | 0.027 | 3.25× | 4.03× |
| 10,000 | 0.031 | 323.95M | 0.027 | 371.91M | 0.036 | 1.17× | 1.34× |
| 100,000 | 0.240 | 416.66M | 0.297 | 337.01M | 0.127 | 0.53× | 0.43× |
| 1,000,000 | 3.116 | 320.90M | 2.641 | 378.61M | 1.778 | 0.57× | 0.67× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.094 | 0.098 | 1.04× |
| 1 | 5 | 0.301 | 0.442 | 1.47× |
| 1 | 10 | 0.590 | 0.979 | 1.66× |
| 10 | 1 | 0.056 | 0.095 | 1.69× |
| 10 | 5 | 0.270 | 0.451 | 1.67× |
| 10 | 10 | 0.521 | 0.871 | 1.67× |
| 100 | 1 | 0.052 | 0.081 | 1.56× |
| 100 | 5 | 0.217 | 0.420 | 1.93× |
| 100 | 10 | 0.480 | 0.895 | 1.87× |
| 1,000 | 1 | 0.051 | 0.083 | 1.63× |
| 1,000 | 5 | 0.240 | 0.417 | 1.74× |
| 1,000 | 10 | 0.485 | 0.879 | 1.81× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
