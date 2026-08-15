# RollingMidpoint benchmark (`MIDPOINT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 159.58M | 0.005 | 186.64M | 0.035 | 5.64× | 6.60× |
| 10,000 | 0.045 | 222.45M | 0.042 | 237.54M | 0.098 | 2.17× | 2.32× |
| 100,000 | 0.434 | 230.39M | 0.404 | 247.72M | 0.719 | 1.66× | 1.78× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.071 | 0.144 | 2.01× |
| 1 | 5 | 0.295 | 0.518 | 1.75× |
| 1 | 10 | 0.443 | 0.923 | 2.08× |
| 10 | 1 | 0.045 | 0.089 | 1.99× |
| 10 | 5 | 0.196 | 0.437 | 2.23× |
| 10 | 10 | 0.407 | 1.036 | 2.54× |
| 100 | 1 | 0.045 | 0.091 | 2.04× |
| 100 | 5 | 0.225 | 0.486 | 2.16× |
| 100 | 10 | 0.456 | 0.992 | 2.18× |
| 1,000 | 1 | 0.058 | 0.101 | 1.72× |
| 1,000 | 5 | 0.236 | 0.499 | 2.11× |
| 1,000 | 10 | 0.465 | 1.108 | 2.38× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
