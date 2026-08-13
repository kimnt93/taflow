# MathSin benchmark (`SIN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.036 | 27.89M | 0.032 | 31.23M | 0.036 | 1.01× | 1.13× |
| 10,000 | 0.283 | 35.34M | 0.276 | 36.19M | 0.171 | 0.60× | 0.62× |
| 100,000 | 2.713 | 36.86M | 2.698 | 37.07M | 1.597 | 0.59× | 0.59× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.106 | 0.124 | 1.16× |
| 1 | 5 | 0.328 | 0.454 | 1.38× |
| 1 | 10 | 0.584 | 0.900 | 1.54× |
| 10 | 1 | 0.065 | 0.109 | 1.67× |
| 10 | 5 | 0.287 | 0.422 | 1.47× |
| 10 | 10 | 0.577 | 0.918 | 1.59× |
| 100 | 1 | 0.066 | 0.087 | 1.31× |
| 100 | 5 | 0.293 | 0.419 | 1.43× |
| 100 | 10 | 0.620 | 0.875 | 1.41× |
| 1,000 | 1 | 0.089 | 0.109 | 1.22× |
| 1,000 | 5 | 0.280 | 0.502 | 1.79× |
| 1,000 | 10 | 0.655 | 1.032 | 1.58× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
