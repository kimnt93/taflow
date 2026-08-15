# RoofingFilter benchmark (`RoofingFilter` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 157.03M | 0.005 | 190.33M | 0.201 | 31.57× | 38.26× |
| 10,000 | 0.046 | 218.20M | 0.046 | 217.90M | 0.518 | 11.31× | 11.29× |
| 100,000 | 0.434 | 230.40M | 0.400 | 250.19M | 3.802 | 8.76× | 9.51× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.069 | 0.284 | 4.12× |
| 1 | 5 | 0.206 | 1.340 | 6.51× |
| 1 | 10 | 0.422 | 2.855 | 6.76× |
| 10 | 1 | 0.048 | 0.234 | 4.85× |
| 10 | 5 | 0.190 | 1.377 | 7.26× |
| 10 | 10 | 0.431 | 2.647 | 6.14× |
| 100 | 1 | 0.047 | 0.240 | 5.12× |
| 100 | 5 | 0.201 | 1.440 | 7.15× |
| 100 | 10 | 0.486 | 2.824 | 5.81× |
| 1,000 | 1 | 0.056 | 0.315 | 5.64× |
| 1,000 | 5 | 0.226 | 1.735 | 7.67× |
| 1,000 | 10 | 0.448 | 3.125 | 6.98× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
