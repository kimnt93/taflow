# MathCeil benchmark (`CEIL` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 164.40M | 0.005 | 199.40M | 0.034 | 5.61× | 6.81× |
| 10,000 | 0.030 | 332.49M | 0.028 | 362.71M | 0.046 | 1.53× | 1.67× |
| 100,000 | 0.261 | 383.82M | 0.253 | 395.27M | 0.214 | 0.82× | 0.84× |
| 1,000,000 | 3.446 | 290.15M | 2.496 | 400.66M | 1.899 | 0.55× | 0.76× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.082 | 0.119 | 1.45× |
| 1 | 5 | 0.301 | 0.455 | 1.51× |
| 1 | 10 | 0.535 | 1.087 | 2.03× |
| 10 | 1 | 0.057 | 0.086 | 1.53× |
| 10 | 5 | 0.220 | 0.440 | 2.00× |
| 10 | 10 | 0.464 | 1.101 | 2.37× |
| 100 | 1 | 0.067 | 0.108 | 1.62× |
| 100 | 5 | 0.506 | 0.454 | 0.90× |
| 100 | 10 | 0.484 | 0.993 | 2.05× |
| 1,000 | 1 | 0.075 | 0.099 | 1.33× |
| 1,000 | 5 | 0.278 | 0.507 | 1.82× |
| 1,000 | 10 | 0.530 | 1.001 | 1.89× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
