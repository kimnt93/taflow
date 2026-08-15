# RelativeMomentumIndex benchmark (`RMI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 114.34M | 0.007 | 136.73M | 0.173 | 19.81× | 23.69× |
| 10,000 | 0.070 | 142.29M | 0.068 | 146.92M | 0.515 | 7.33× | 7.56× |
| 100,000 | 0.686 | 145.87M | 0.657 | 152.29M | 3.926 | 5.73× | 5.98× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.175 | 0.327 | 1.87× |
| 1 | 5 | 0.333 | 1.151 | 3.46× |
| 1 | 10 | 0.378 | 2.457 | 6.50× |
| 10 | 1 | 0.054 | 0.215 | 3.99× |
| 10 | 5 | 0.187 | 1.091 | 5.84× |
| 10 | 10 | 0.392 | 2.442 | 6.24× |
| 100 | 1 | 0.046 | 0.208 | 4.51× |
| 100 | 5 | 0.180 | 1.064 | 5.90× |
| 100 | 10 | 0.380 | 2.436 | 6.42× |
| 1,000 | 1 | 0.048 | 0.253 | 5.32× |
| 1,000 | 5 | 0.188 | 1.260 | 6.70× |
| 1,000 | 10 | 0.476 | 2.908 | 6.11× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
