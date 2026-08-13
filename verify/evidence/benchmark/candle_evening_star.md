# CandleEveningStar benchmark (`CDLEVENINGSTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.121 | 8.28M | 0.106 | 9.40M | 0.036 | 0.30× | 0.34× |
| 10,000 | 1.034 | 9.67M | 0.959 | 10.43M | 0.105 | 0.10× | 0.11× |
| 100,000 | 9.936 | 10.06M | 9.652 | 10.36M | 0.796 | 0.08× | 0.08× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.076 | 0.127 | 1.67× |
| 1 | 5 | 0.456 | 0.546 | 1.20× |
| 1 | 10 | 0.648 | 0.943 | 1.46× |
| 10 | 1 | 0.069 | 0.093 | 1.35× |
| 10 | 5 | 0.320 | 0.459 | 1.44× |
| 10 | 10 | 0.640 | 0.959 | 1.50× |
| 100 | 1 | 0.081 | 0.091 | 1.13× |
| 100 | 5 | 0.308 | 0.453 | 1.47× |
| 100 | 10 | 0.678 | 0.994 | 1.47× |
| 1,000 | 1 | 0.180 | 0.108 | 0.60× |
| 1,000 | 5 | 0.435 | 0.505 | 1.16× |
| 1,000 | 10 | 0.710 | 1.036 | 1.46× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
