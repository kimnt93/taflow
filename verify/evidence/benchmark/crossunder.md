# Crossunder benchmark (`causal crossunder` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 184.48M | 0.004 | 242.59M | 0.017 | 3.11× | 4.09× |
| 10,000 | 0.034 | 292.12M | 0.031 | 322.46M | 0.029 | 0.83× | 0.92× |
| 100,000 | 0.338 | 295.85M | 0.298 | 335.41M | 0.132 | 0.39× | 0.44× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.053 | 0.089 | 1.70× |
| 1 | 5 | 0.295 | 0.321 | 1.09× |
| 1 | 10 | 0.393 | 0.674 | 1.71× |
| 10 | 1 | 0.041 | 0.063 | 1.53× |
| 10 | 5 | 0.176 | 0.307 | 1.75× |
| 10 | 10 | 0.366 | 0.664 | 1.81× |
| 100 | 1 | 0.048 | 0.074 | 1.54× |
| 100 | 5 | 0.194 | 0.332 | 1.72× |
| 100 | 10 | 0.385 | 0.695 | 1.80× |
| 1,000 | 1 | 0.054 | 0.066 | 1.22× |
| 1,000 | 5 | 0.212 | 0.374 | 1.76× |
| 1,000 | 10 | 0.422 | 0.874 | 2.07× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
