# ParabolicMovingAverageStop benchmark (`pmax` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 50.99M | 0.017 | 59.33M | 2.887 | 147.20× | 171.26× |
| 10,000 | 0.165 | 60.74M | 0.163 | 61.51M | 15.667 | 95.16× | 96.37× |
| 100,000 | 1.657 | 60.35M | 1.551 | 64.49M | 147.270 | 88.87× | 94.97× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.128 | 0.285 | 2.23× |
| 1 | 5 | 0.221 | 1.257 | 5.69× |
| 1 | 10 | 0.409 | 2.274 | 5.56× |
| 10 | 1 | 0.048 | 1.738 | 36.56× |
| 10 | 5 | 0.192 | 8.247 | 42.97× |
| 10 | 10 | 0.424 | 17.710 | 41.74× |
| 100 | 1 | 0.054 | 1.870 | 34.78× |
| 100 | 5 | 0.245 | 9.597 | 39.14× |
| 100 | 10 | 0.579 | 19.763 | 34.10× |
| 1,000 | 1 | 0.068 | 3.287 | 48.22× |
| 1,000 | 5 | 0.219 | 16.412 | 74.96× |
| 1,000 | 10 | 0.450 | 33.162 | 73.74× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
