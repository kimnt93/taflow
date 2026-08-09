# CandleHikkake benchmark (`CDLHIKKAKE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 117.55M | 0.007 | 150.97M | 0.030 | 3.48× | 4.47× |
| 10,000 | 0.057 | 174.79M | 0.055 | 182.15M | 0.075 | 1.31× | 1.37× |
| 100,000 | 0.575 | 173.87M | 0.565 | 176.99M | 0.476 | 0.83× | 0.84× |
| 1,000,000 | 6.182 | 161.76M | 6.264 | 159.64M | 4.770 | 0.77× | 0.76× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.102 | 0.108 | 1.06× |
| 1 | 5 | 0.316 | 0.501 | 1.58× |
| 1 | 10 | 0.506 | 0.929 | 1.84× |
| 10 | 1 | 0.055 | 0.092 | 1.68× |
| 10 | 5 | 0.237 | 0.446 | 1.88× |
| 10 | 10 | 0.499 | 0.931 | 1.87× |
| 100 | 1 | 0.052 | 0.096 | 1.84× |
| 100 | 5 | 0.245 | 0.438 | 1.79× |
| 100 | 10 | 0.514 | 0.915 | 1.78× |
| 1,000 | 1 | 0.063 | 0.096 | 1.52× |
| 1,000 | 5 | 0.263 | 0.471 | 1.79× |
| 1,000 | 10 | 0.658 | 1.057 | 1.61× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
