# RelativeMomentumIndex benchmark (`RMI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 96.02M | 0.009 | 106.81M | 0.210 | 20.17× | 22.43× |
| 10,000 | 0.073 | 136.82M | 0.079 | 126.51M | 0.594 | 8.12× | 7.51× |
| 100,000 | 0.781 | 127.99M | 0.677 | 147.71M | 4.331 | 5.54× | 6.40× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.106 | 0.355 | 3.34× |
| 1 | 5 | 0.288 | 1.158 | 4.03× |
| 1 | 10 | 0.497 | 2.913 | 5.86× |
| 10 | 1 | 0.059 | 0.257 | 4.32× |
| 10 | 5 | 0.283 | 1.189 | 4.20× |
| 10 | 10 | 0.528 | 2.755 | 5.21× |
| 100 | 1 | 0.052 | 0.230 | 4.45× |
| 100 | 5 | 0.263 | 1.264 | 4.81× |
| 100 | 10 | 0.515 | 2.850 | 5.54× |
| 1,000 | 1 | 0.063 | 0.284 | 4.51× |
| 1,000 | 5 | 0.279 | 1.381 | 4.95× |
| 1,000 | 10 | 0.525 | 3.211 | 6.12× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
