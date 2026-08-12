# LowestSince benchmark (`lowest since condition` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 123.40M | 0.007 | 142.75M | 0.300 | 37.00× | 42.80× |
| 10,000 | 0.042 | 239.36M | 0.039 | 258.13M | 2.754 | 65.91× | 71.08× |
| 100,000 | 0.375 | 266.51M | 0.342 | 292.40M | 31.710 | 84.51× | 92.72× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.078 | 0.082 | 1.06× |
| 1 | 5 | 0.268 | 0.338 | 1.26× |
| 1 | 10 | 0.509 | 0.841 | 1.65× |
| 10 | 1 | 0.057 | 0.079 | 1.38× |
| 10 | 5 | 0.301 | 0.412 | 1.37× |
| 10 | 10 | 0.539 | 0.819 | 1.52× |
| 100 | 1 | 0.062 | 0.141 | 2.28× |
| 100 | 5 | 0.266 | 0.512 | 1.93× |
| 100 | 10 | 0.518 | 0.968 | 1.87× |
| 1,000 | 1 | 0.058 | 0.350 | 6.06× |
| 1,000 | 5 | 0.293 | 2.027 | 6.92× |
| 1,000 | 10 | 0.515 | 3.710 | 7.21× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
