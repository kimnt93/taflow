# CandleHikkakeModified benchmark (`CDLHIKKAKEMOD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 125.25M | 0.005 | 197.94M | 0.035 | 4.37× | 6.91× |
| 10,000 | 0.061 | 163.03M | 0.057 | 176.17M | 0.083 | 1.36× | 1.47× |
| 100,000 | 0.610 | 163.90M | 0.587 | 170.27M | 0.556 | 0.91× | 0.95× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.057 | 0.129 | 2.26× |
| 1 | 5 | 0.247 | 0.453 | 1.83× |
| 1 | 10 | 0.389 | 0.945 | 2.43× |
| 10 | 1 | 0.043 | 0.091 | 2.10× |
| 10 | 5 | 0.207 | 0.468 | 2.27× |
| 10 | 10 | 0.427 | 0.908 | 2.12× |
| 100 | 1 | 0.043 | 0.088 | 2.02× |
| 100 | 5 | 0.176 | 0.446 | 2.54× |
| 100 | 10 | 0.462 | 0.915 | 1.98× |
| 1,000 | 1 | 0.050 | 0.089 | 1.78× |
| 1,000 | 5 | 0.191 | 0.454 | 2.38× |
| 1,000 | 10 | 0.413 | 1.015 | 2.46× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
