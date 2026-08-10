# CandleHikkakeModified benchmark (`CDLHIKKAKEMOD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 70.16M | 0.011 | 90.64M | 0.035 | 2.49× | 3.21× |
| 10,000 | 0.066 | 152.05M | 0.062 | 160.81M | 0.083 | 1.26× | 1.33× |
| 100,000 | 0.592 | 168.85M | 0.586 | 170.61M | 0.624 | 1.05× | 1.06× |
| 1,000,000 | 6.469 | 154.57M | 5.963 | 167.70M | 5.894 | 0.91× | 0.99× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.092 | 0.130 | 1.42× |
| 1 | 5 | 0.355 | 0.555 | 1.57× |
| 1 | 10 | 0.557 | 0.954 | 1.71× |
| 10 | 1 | 0.057 | 0.091 | 1.58× |
| 10 | 5 | 0.251 | 0.464 | 1.85× |
| 10 | 10 | 0.616 | 0.968 | 1.57× |
| 100 | 1 | 0.058 | 0.098 | 1.69× |
| 100 | 5 | 0.246 | 0.436 | 1.77× |
| 100 | 10 | 0.531 | 1.062 | 2.00× |
| 1,000 | 1 | 0.070 | 0.094 | 1.34× |
| 1,000 | 5 | 0.288 | 0.518 | 1.80× |
| 1,000 | 10 | 0.626 | 1.526 | 2.44× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
