# CandleOnNeck benchmark (`CDLONNECK` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 161.16M | 0.003 | 336.67M | 0.034 | 5.45× | 11.38× |
| 10,000 | 0.058 | 171.49M | 0.052 | 192.22M | 0.121 | 2.07× | 2.32× |
| 100,000 | 0.843 | 118.61M | 0.961 | 104.08M | 1.352 | 1.60× | 1.41× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.073 | 0.149 | 2.05× |
| 1 | 5 | 0.270 | 0.732 | 2.71× |
| 1 | 10 | 0.670 | 1.218 | 1.82× |
| 10 | 1 | 0.042 | 0.085 | 2.03× |
| 10 | 5 | 0.196 | 0.444 | 2.26× |
| 10 | 10 | 0.436 | 1.210 | 2.77× |
| 100 | 1 | 0.049 | 0.090 | 1.85× |
| 100 | 5 | 0.258 | 0.572 | 2.22× |
| 100 | 10 | 0.479 | 1.009 | 2.11× |
| 1,000 | 1 | 0.049 | 0.097 | 2.00× |
| 1,000 | 5 | 0.201 | 0.506 | 2.51× |
| 1,000 | 10 | 0.435 | 0.993 | 2.28× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
