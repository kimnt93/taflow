# RollingKellyCriterion benchmark (`KellyCriterion` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 51.87M | 0.019 | 52.78M | 0.158 | 8.21× | 8.36× |
| 10,000 | 0.168 | 59.58M | 0.170 | 58.90M | 0.656 | 3.91× | 3.87× |
| 100,000 | 1.606 | 62.28M | 1.649 | 60.64M | 5.916 | 3.68× | 3.59× |
| 1,000,000 | 16.920 | 59.10M | 16.440 | 60.83M | 55.237 | 3.26× | 3.36× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.066 | 0.223 | 3.39× |
| 1 | 5 | 0.380 | 0.958 | 2.52× |
| 1 | 10 | 0.466 | 2.048 | 4.40× |
| 10 | 1 | 0.049 | 0.188 | 3.86× |
| 10 | 5 | 0.229 | 0.947 | 4.14× |
| 10 | 10 | 0.462 | 2.081 | 4.50× |
| 100 | 1 | 0.055 | 0.197 | 3.59× |
| 100 | 5 | 0.224 | 0.955 | 4.26× |
| 100 | 10 | 0.489 | 2.116 | 4.33× |
| 1,000 | 1 | 0.075 | 0.252 | 3.35× |
| 1,000 | 5 | 0.233 | 1.244 | 5.34× |
| 1,000 | 10 | 0.562 | 2.840 | 5.05× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
