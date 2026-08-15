# EntryExit benchmark (`entry-exit position state` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 225.09M | 0.003 | 332.07M | 0.139 | 31.25× | 46.11× |
| 10,000 | 0.022 | 459.26M | 0.018 | 551.08M | 1.306 | 59.99× | 71.99× |
| 100,000 | 0.209 | 477.44M | 0.176 | 567.75M | 12.554 | 59.94× | 71.27× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.082 | 0.077 | 0.94× |
| 1 | 5 | 0.261 | 0.324 | 1.24× |
| 1 | 10 | 0.379 | 0.669 | 1.77× |
| 10 | 1 | 0.042 | 0.069 | 1.62× |
| 10 | 5 | 0.193 | 0.298 | 1.55× |
| 10 | 10 | 0.392 | 0.631 | 1.61× |
| 100 | 1 | 0.045 | 0.072 | 1.59× |
| 100 | 5 | 0.201 | 0.419 | 2.09× |
| 100 | 10 | 0.410 | 0.761 | 1.86× |
| 1,000 | 1 | 0.045 | 0.189 | 4.18× |
| 1,000 | 5 | 0.189 | 0.943 | 5.00× |
| 1,000 | 10 | 0.451 | 2.015 | 4.47× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
