# MathFloor benchmark (`FLOOR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 186.66M | 0.005 | 215.10M | 0.029 | 5.43× | 6.26× |
| 10,000 | 0.028 | 358.48M | 0.025 | 403.47M | 0.042 | 1.50× | 1.69× |
| 100,000 | 0.246 | 406.68M | 0.224 | 446.42M | 0.168 | 0.68× | 0.75× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.070 | 0.113 | 1.60× |
| 1 | 5 | 0.238 | 0.487 | 2.04× |
| 1 | 10 | 0.482 | 0.893 | 1.85× |
| 10 | 1 | 0.049 | 0.087 | 1.80× |
| 10 | 5 | 0.215 | 0.421 | 1.96× |
| 10 | 10 | 0.463 | 0.920 | 1.99× |
| 100 | 1 | 0.052 | 0.082 | 1.59× |
| 100 | 5 | 0.232 | 0.441 | 1.90× |
| 100 | 10 | 0.649 | 0.878 | 1.35× |
| 1,000 | 1 | 0.068 | 0.097 | 1.43× |
| 1,000 | 5 | 0.247 | 0.422 | 1.71× |
| 1,000 | 10 | 0.532 | 0.970 | 1.82× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
