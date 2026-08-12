# HedgeRatio benchmark (`rolling OLS hedge ratio` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.043 | 23.45M | 0.042 | 23.96M | 0.274 | 6.42× | 6.56× |
| 10,000 | 0.392 | 25.48M | 0.372 | 26.87M | 1.768 | 4.50× | 4.75× |
| 100,000 | 4.015 | 24.91M | 3.863 | 25.88M | 19.092 | 4.76× | 4.94× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.134 | 0.190 | 1.42× |
| 1 | 5 | 0.292 | 0.663 | 2.27× |
| 1 | 10 | 0.547 | 1.322 | 2.42× |
| 10 | 1 | 0.050 | 0.140 | 2.77× |
| 10 | 5 | 0.251 | 0.649 | 2.59× |
| 10 | 10 | 0.487 | 1.305 | 2.68× |
| 100 | 1 | 0.057 | 0.214 | 3.76× |
| 100 | 5 | 0.274 | 1.147 | 4.19× |
| 100 | 10 | 0.503 | 2.278 | 4.53× |
| 1,000 | 1 | 0.095 | 0.366 | 3.84× |
| 1,000 | 5 | 0.254 | 1.365 | 5.37× |
| 1,000 | 10 | 0.535 | 2.912 | 5.45× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
