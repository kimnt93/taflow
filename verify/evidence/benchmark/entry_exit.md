# EntryExit benchmark (`entry-exit position state` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.037 | 27.39M | 0.028 | 35.79M | 0.135 | 3.71× | 4.84× |
| 10,000 | 0.206 | 48.48M | 0.195 | 51.15M | 1.264 | 6.13× | 6.47× |
| 100,000 | 1.910 | 52.36M | 1.855 | 53.91M | 12.625 | 6.61× | 6.81× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.095 | 0.096 | 1.01× |
| 1 | 5 | 0.338 | 0.318 | 0.94× |
| 1 | 10 | 0.589 | 0.623 | 1.06× |
| 10 | 1 | 0.060 | 0.067 | 1.12× |
| 10 | 5 | 0.282 | 0.312 | 1.11× |
| 10 | 10 | 0.580 | 0.624 | 1.08× |
| 100 | 1 | 0.067 | 0.077 | 1.15× |
| 100 | 5 | 0.295 | 0.353 | 1.20× |
| 100 | 10 | 0.624 | 0.735 | 1.18× |
| 1,000 | 1 | 0.088 | 0.187 | 2.13× |
| 1,000 | 5 | 0.283 | 0.942 | 3.32× |
| 1,000 | 10 | 0.620 | 1.925 | 3.11× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
