# AwesomeOscillator benchmark (`AwesomeOscillator` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.026 | 38.16M | 0.026 | 39.00M | 0.235 | 8.95× | 9.15× |
| 10,000 | 0.244 | 40.96M | 0.251 | 39.77M | 0.899 | 3.68× | 3.58× |
| 100,000 | 2.384 | 41.94M | 2.335 | 42.82M | 7.278 | 3.05× | 3.12× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.076 | 0.276 | 3.64× |
| 1 | 5 | 0.295 | 1.311 | 4.45× |
| 1 | 10 | 0.411 | 2.662 | 6.48× |
| 10 | 1 | 0.044 | 0.257 | 5.86× |
| 10 | 5 | 0.196 | 1.369 | 7.00× |
| 10 | 10 | 0.439 | 2.534 | 5.77× |
| 100 | 1 | 0.047 | 0.246 | 5.20× |
| 100 | 5 | 0.200 | 1.448 | 7.23× |
| 100 | 10 | 0.448 | 2.702 | 6.03× |
| 1,000 | 1 | 0.077 | 0.334 | 4.36× |
| 1,000 | 5 | 0.221 | 1.738 | 7.87× |
| 1,000 | 10 | 0.473 | 3.261 | 6.90× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
