# CandleGravestoneDoji benchmark (`CDLGRAVESTONEDOJI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 126.68M | 0.005 | 213.91M | 0.034 | 4.35× | 7.34× |
| 10,000 | 0.063 | 159.96M | 0.059 | 169.22M | 0.099 | 1.59× | 1.68× |
| 100,000 | 0.718 | 139.27M | 0.694 | 144.17M | 0.732 | 1.02× | 1.06× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.058 | 0.139 | 2.38× |
| 1 | 5 | 0.307 | 0.443 | 1.44× |
| 1 | 10 | 0.382 | 0.915 | 2.39× |
| 10 | 1 | 0.050 | 0.104 | 2.09× |
| 10 | 5 | 0.207 | 0.458 | 2.22× |
| 10 | 10 | 0.378 | 0.889 | 2.35× |
| 100 | 1 | 0.040 | 0.086 | 2.15× |
| 100 | 5 | 0.180 | 0.413 | 2.30× |
| 100 | 10 | 0.409 | 0.926 | 2.26× |
| 1,000 | 1 | 0.049 | 0.095 | 1.95× |
| 1,000 | 5 | 0.190 | 0.452 | 2.38× |
| 1,000 | 10 | 0.380 | 1.012 | 2.66× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
