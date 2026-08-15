# MesaAdaptiveMovingAverage benchmark (`MAMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.054 | 18.57M | 0.052 | 19.26M | 0.086 | 1.60× | 1.66× |
| 10,000 | 0.521 | 19.18M | 0.497 | 20.13M | 0.564 | 1.08× | 1.14× |
| 100,000 | 5.781 | 17.30M | 5.150 | 19.42M | 5.141 | 0.89× | 1.00× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.119 | 0.113 | 0.95× |
| 1 | 5 | 0.248 | 0.517 | 2.08× |
| 1 | 10 | 0.392 | 1.050 | 2.68× |
| 10 | 1 | 0.040 | 0.099 | 2.47× |
| 10 | 5 | 0.191 | 0.472 | 2.48× |
| 10 | 10 | 0.404 | 0.956 | 2.37× |
| 100 | 1 | 0.051 | 0.124 | 2.44× |
| 100 | 5 | 0.215 | 0.515 | 2.40× |
| 100 | 10 | 0.402 | 1.017 | 2.53× |
| 1,000 | 1 | 0.098 | 0.151 | 1.53× |
| 1,000 | 5 | 0.195 | 0.784 | 4.03× |
| 1,000 | 10 | 0.441 | 1.553 | 3.52× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
