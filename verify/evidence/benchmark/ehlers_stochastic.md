# EhlersStochastic benchmark (`EhlersStochastic` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.050 | 19.84M | 0.042 | 23.55M | 0.200 | 3.96× | 4.70× |
| 10,000 | 0.488 | 20.47M | 0.464 | 21.54M | 0.818 | 1.67× | 1.76× |
| 100,000 | 3.647 | 27.42M | 3.943 | 25.36M | 9.752 | 2.67× | 2.47× |
| 1,000,000 | 39.755 | 25.15M | 39.465 | 25.34M | 77.494 | 1.95× | 1.96× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.096 | 0.276 | 2.87× |
| 1 | 5 | 0.316 | 1.016 | 3.22× |
| 1 | 10 | 0.492 | 2.261 | 4.60× |
| 10 | 1 | 0.057 | 0.202 | 3.57× |
| 10 | 5 | 0.229 | 0.952 | 4.15× |
| 10 | 10 | 0.513 | 2.513 | 4.90× |
| 100 | 1 | 0.060 | 0.196 | 3.29× |
| 100 | 5 | 0.240 | 1.013 | 4.22× |
| 100 | 10 | 0.543 | 2.570 | 4.73× |
| 1,000 | 1 | 0.111 | 0.337 | 3.04× |
| 1,000 | 5 | 0.327 | 1.542 | 4.72× |
| 1,000 | 10 | 0.605 | 3.341 | 5.52× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
