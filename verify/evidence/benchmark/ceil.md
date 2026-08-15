# MathCeil benchmark (`CEIL` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.002 | 524.72M | 0.001 | 1.13G | 0.031 | 16.40× | 35.31× |
| 10,000 | 0.006 | 1.72G | 0.003 | 3.38G | 0.044 | 7.52× | 14.78× |
| 100,000 | 0.057 | 1.74G | 0.030 | 3.35G | 0.163 | 2.84× | 5.46× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.070 | 0.136 | 1.94× |
| 1 | 5 | 0.305 | 0.432 | 1.41× |
| 1 | 10 | 0.372 | 0.885 | 2.38× |
| 10 | 1 | 0.042 | 0.085 | 2.04× |
| 10 | 5 | 0.207 | 0.446 | 2.16× |
| 10 | 10 | 0.418 | 0.952 | 2.28× |
| 100 | 1 | 0.040 | 0.082 | 2.05× |
| 100 | 5 | 0.181 | 0.442 | 2.44× |
| 100 | 10 | 0.432 | 0.928 | 2.15× |
| 1,000 | 1 | 0.042 | 0.091 | 2.15× |
| 1,000 | 5 | 0.218 | 0.447 | 2.05× |
| 1,000 | 10 | 0.416 | 0.985 | 2.36× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
