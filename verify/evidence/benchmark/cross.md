# Cross benchmark (`causal cross event` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.003 | 353.37M | 0.002 | 629.31M | 0.023 | 8.15× | 14.52× |
| 10,000 | 0.013 | 793.51M | 0.009 | 1.12G | 0.052 | 4.13× | 5.84× |
| 100,000 | 0.102 | 982.62M | 0.077 | 1.30G | 0.296 | 2.91× | 3.84× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.070 | 0.124 | 1.78× |
| 1 | 5 | 0.268 | 0.385 | 1.44× |
| 1 | 10 | 0.389 | 0.714 | 1.83× |
| 10 | 1 | 0.041 | 0.078 | 1.92× |
| 10 | 5 | 0.178 | 0.335 | 1.89× |
| 10 | 10 | 0.422 | 0.780 | 1.85× |
| 100 | 1 | 0.045 | 0.068 | 1.51× |
| 100 | 5 | 0.209 | 0.337 | 1.61× |
| 100 | 10 | 0.388 | 0.761 | 1.96× |
| 1,000 | 1 | 0.048 | 0.087 | 1.81× |
| 1,000 | 5 | 0.195 | 0.682 | 3.49× |
| 1,000 | 10 | 0.404 | 1.229 | 3.04× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
