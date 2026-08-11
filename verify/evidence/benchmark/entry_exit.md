# EntryExit benchmark (`entry-exit position state` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 151.42M | 0.005 | 186.86M | 0.129 | 19.52× | 24.09× |
| 10,000 | 0.028 | 356.75M | 0.024 | 409.03M | 1.218 | 43.47× | 49.84× |
| 100,000 | 0.232 | 431.05M | 0.203 | 492.38M | 12.206 | 52.61× | 60.10× |
| 1,000,000 | 2.414 | 414.18M | 2.120 | 471.79M | 128.780 | 53.34× | 60.76× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.084 | 0.076 | 0.90× |
| 1 | 5 | 0.278 | 0.363 | 1.30× |
| 1 | 10 | 0.566 | 0.801 | 1.41× |
| 10 | 1 | 0.051 | 0.064 | 1.26× |
| 10 | 5 | 0.274 | 0.387 | 1.41× |
| 10 | 10 | 0.536 | 0.711 | 1.33× |
| 100 | 1 | 0.049 | 0.080 | 1.63× |
| 100 | 5 | 0.225 | 0.361 | 1.60× |
| 100 | 10 | 0.500 | 0.739 | 1.48× |
| 1,000 | 1 | 0.054 | 0.195 | 3.62× |
| 1,000 | 5 | 0.248 | 0.977 | 3.94× |
| 1,000 | 10 | 0.542 | 2.013 | 3.71× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
