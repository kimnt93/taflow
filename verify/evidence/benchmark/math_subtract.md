# MathSubtract benchmark (`SUB` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.029 | 34.16M | 0.022 | 45.36M | 0.029 | 0.99× | 1.31× |
| 10,000 | 0.160 | 62.37M | 0.147 | 67.94M | 0.032 | 0.20× | 0.22× |
| 100,000 | 1.464 | 68.31M | 1.396 | 71.65M | 0.065 | 0.04× | 0.05× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.127 | 0.110 | 0.86× |
| 1 | 5 | 0.447 | 0.465 | 1.04× |
| 1 | 10 | 0.613 | 0.990 | 1.62× |
| 10 | 1 | 0.129 | 0.116 | 0.90× |
| 10 | 5 | 0.303 | 0.426 | 1.40× |
| 10 | 10 | 0.568 | 0.867 | 1.53× |
| 100 | 1 | 0.063 | 0.088 | 1.39× |
| 100 | 5 | 0.284 | 0.423 | 1.49× |
| 100 | 10 | 0.580 | 0.937 | 1.62× |
| 1,000 | 1 | 0.076 | 0.090 | 1.18× |
| 1,000 | 5 | 0.284 | 0.424 | 1.49× |
| 1,000 | 10 | 0.640 | 0.916 | 1.43× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
