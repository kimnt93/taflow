# CandleBeltHold benchmark (`CDLBELTHOLD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 156.85M | 0.003 | 302.62M | 0.039 | 6.18× | 11.92× |
| 10,000 | 0.096 | 104.48M | 0.090 | 111.57M | 0.121 | 1.27× | 1.35× |
| 100,000 | 0.968 | 103.26M | 0.963 | 103.89M | 0.979 | 1.01× | 1.02× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.057 | 0.107 | 1.86× |
| 1 | 5 | 0.245 | 0.435 | 1.78× |
| 1 | 10 | 0.381 | 0.942 | 2.47× |
| 10 | 1 | 0.042 | 0.085 | 2.03× |
| 10 | 5 | 0.177 | 0.434 | 2.45× |
| 10 | 10 | 0.375 | 0.906 | 2.41× |
| 100 | 1 | 0.041 | 0.093 | 2.25× |
| 100 | 5 | 0.191 | 0.443 | 2.32× |
| 100 | 10 | 0.387 | 0.890 | 2.30× |
| 1,000 | 1 | 0.049 | 0.099 | 2.00× |
| 1,000 | 5 | 0.192 | 0.509 | 2.66× |
| 1,000 | 10 | 0.413 | 0.998 | 2.42× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
