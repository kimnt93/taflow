# RollingStandardError benchmark (`StandardError` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.035 | 28.86M | 0.033 | 30.66M | 0.198 | 5.71× | 6.07× |
| 10,000 | 0.328 | 30.47M | 0.312 | 32.03M | 0.725 | 2.21× | 2.32× |
| 100,000 | 3.147 | 31.77M | 3.071 | 32.57M | 6.216 | 1.97× | 2.02× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.096 | 0.258 | 2.69× |
| 1 | 5 | 0.243 | 1.053 | 4.34× |
| 1 | 10 | 0.503 | 2.519 | 5.01× |
| 10 | 1 | 0.053 | 0.206 | 3.87× |
| 10 | 5 | 0.240 | 1.349 | 5.62× |
| 10 | 10 | 0.497 | 2.313 | 4.66× |
| 100 | 1 | 0.052 | 0.214 | 4.12× |
| 100 | 5 | 0.278 | 1.430 | 5.15× |
| 100 | 10 | 0.517 | 2.422 | 4.69× |
| 1,000 | 1 | 0.094 | 0.312 | 3.34× |
| 1,000 | 5 | 0.261 | 1.552 | 5.94× |
| 1,000 | 10 | 0.525 | 3.004 | 5.72× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
