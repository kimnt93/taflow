# MathExp benchmark (`EXP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.028 | 35.56M | 0.025 | 40.11M | 0.030 | 1.07× | 1.21× |
| 10,000 | 0.191 | 52.49M | 0.200 | 49.97M | 0.068 | 0.36× | 0.34× |
| 100,000 | 1.645 | 60.79M | 1.775 | 56.34M | 0.450 | 0.27× | 0.25× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.128 | 0.109 | 0.86× |
| 1 | 5 | 0.419 | 0.429 | 1.03× |
| 1 | 10 | 0.565 | 0.847 | 1.50× |
| 10 | 1 | 0.069 | 0.085 | 1.24× |
| 10 | 5 | 0.276 | 0.413 | 1.50× |
| 10 | 10 | 0.583 | 0.863 | 1.48× |
| 100 | 1 | 0.062 | 0.091 | 1.47× |
| 100 | 5 | 0.280 | 0.432 | 1.54× |
| 100 | 10 | 0.597 | 0.881 | 1.48× |
| 1,000 | 1 | 0.078 | 0.089 | 1.13× |
| 1,000 | 5 | 0.276 | 0.426 | 1.54× |
| 1,000 | 10 | 0.604 | 0.910 | 1.51× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
