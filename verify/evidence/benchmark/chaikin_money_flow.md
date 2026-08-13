# ChaikinMoneyFlow benchmark (`ChaikinMoneyFlow` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.094 | 10.67M | 0.079 | 12.64M | 0.278 | 2.96× | 3.51× |
| 10,000 | 0.648 | 15.44M | 0.689 | 14.51M | 1.509 | 2.33× | 2.19× |
| 100,000 | 6.400 | 15.62M | 6.187 | 16.16M | 13.596 | 2.12× | 2.20× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.113 | 0.269 | 2.38× |
| 1 | 5 | 0.454 | 1.140 | 2.51× |
| 1 | 10 | 0.663 | 2.710 | 4.09× |
| 10 | 1 | 0.079 | 0.222 | 2.81× |
| 10 | 5 | 0.322 | 1.076 | 3.34× |
| 10 | 10 | 0.668 | 2.392 | 3.58× |
| 100 | 1 | 0.093 | 0.226 | 2.43× |
| 100 | 5 | 0.332 | 1.427 | 4.29× |
| 100 | 10 | 0.676 | 2.508 | 3.71× |
| 1,000 | 1 | 0.142 | 0.359 | 2.53× |
| 1,000 | 5 | 0.347 | 2.045 | 5.90× |
| 1,000 | 10 | 0.729 | 3.778 | 5.18× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
