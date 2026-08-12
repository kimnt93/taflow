# BreadthThrust benchmark (`BreadthThrust` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 99.94M | 0.009 | 117.48M | 9.172 | 916.69× | 1077.59× |
| 10,000 | 0.058 | 171.13M | 0.091 | 109.99M | 87.916 | 1504.55× | 966.97× |
| 100,000 | 0.532 | 188.04M | 0.527 | 189.93M | 874.080 | 1643.60× | 1660.15× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.196 | 0.346 | 1.76× |
| 1 | 5 | 0.315 | 1.244 | 3.94× |
| 1 | 10 | 0.547 | 2.656 | 4.85× |
| 10 | 1 | 0.059 | 0.334 | 5.63× |
| 10 | 5 | 0.236 | 1.652 | 6.99× |
| 10 | 10 | 0.506 | 3.620 | 7.15× |
| 100 | 1 | 0.055 | 1.156 | 21.02× |
| 100 | 5 | 0.237 | 5.851 | 24.73× |
| 100 | 10 | 0.581 | 11.932 | 20.55× |
| 1,000 | 1 | 0.068 | 9.159 | 135.29× |
| 1,000 | 5 | 0.265 | 46.761 | 176.59× |
| 1,000 | 10 | 0.658 | 96.203 | 146.19× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
