# RollingKellyCriterion benchmark (`KellyCriterion` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.168 | 5.94M | 0.173 | 5.77M | 0.167 | 0.99× | 0.96× |
| 10,000 | 1.628 | 6.14M | 1.601 | 6.24M | 0.720 | 0.44× | 0.45× |
| 100,000 | 18.192 | 5.50M | 16.280 | 6.14M | 5.269 | 0.29× | 0.32× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.115 | 0.211 | 1.84× |
| 1 | 5 | 0.460 | 0.982 | 2.14× |
| 1 | 10 | 0.637 | 2.068 | 3.25× |
| 10 | 1 | 0.070 | 0.190 | 2.72× |
| 10 | 5 | 0.299 | 0.965 | 3.23× |
| 10 | 10 | 0.601 | 2.140 | 3.56× |
| 100 | 1 | 0.084 | 0.198 | 2.35× |
| 100 | 5 | 0.305 | 0.962 | 3.15× |
| 100 | 10 | 0.625 | 2.128 | 3.40× |
| 1,000 | 1 | 0.247 | 0.247 | 1.00× |
| 1,000 | 5 | 0.528 | 1.229 | 2.33× |
| 1,000 | 10 | 0.752 | 2.684 | 3.57× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
