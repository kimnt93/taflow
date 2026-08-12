# MoneyFlowIndex benchmark (`MFI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 72.56M | 0.012 | 86.38M | 0.036 | 2.63× | 3.13× |
| 10,000 | 0.066 | 152.64M | 0.062 | 162.05M | 0.107 | 1.64× | 1.74× |
| 100,000 | 0.642 | 155.83M | 0.566 | 176.59M | 0.932 | 1.45× | 1.65× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.184 | 0.111 | 0.60× |
| 1 | 5 | 0.289 | 0.477 | 1.65× |
| 1 | 10 | 0.500 | 1.009 | 2.02× |
| 10 | 1 | 0.059 | 0.096 | 1.61× |
| 10 | 5 | 0.239 | 0.454 | 1.90× |
| 10 | 10 | 0.493 | 0.952 | 1.93× |
| 100 | 1 | 0.050 | 0.091 | 1.81× |
| 100 | 5 | 0.246 | 0.500 | 2.03× |
| 100 | 10 | 0.519 | 0.954 | 1.84× |
| 1,000 | 1 | 0.061 | 0.101 | 1.67× |
| 1,000 | 5 | 0.236 | 0.496 | 2.10× |
| 1,000 | 10 | 0.586 | 1.032 | 1.76× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
