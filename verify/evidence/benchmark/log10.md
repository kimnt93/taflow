# MathLog10 benchmark (`LOG10` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.034 | 29.43M | 0.029 | 34.35M | 0.034 | 1.01× | 1.18× |
| 10,000 | 0.229 | 43.67M | 0.258 | 38.82M | 0.103 | 0.45× | 0.40× |
| 100,000 | 2.220 | 45.04M | 2.213 | 45.19M | 0.784 | 0.35× | 0.35× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.098 | 0.123 | 1.25× |
| 1 | 5 | 0.357 | 0.426 | 1.19× |
| 1 | 10 | 0.573 | 0.899 | 1.57× |
| 10 | 1 | 0.063 | 0.089 | 1.41× |
| 10 | 5 | 0.275 | 0.404 | 1.47× |
| 10 | 10 | 0.688 | 0.883 | 1.28× |
| 100 | 1 | 0.064 | 0.090 | 1.42× |
| 100 | 5 | 0.273 | 0.420 | 1.54× |
| 100 | 10 | 0.623 | 0.860 | 1.38× |
| 1,000 | 1 | 0.088 | 0.092 | 1.05× |
| 1,000 | 5 | 0.307 | 0.462 | 1.51× |
| 1,000 | 10 | 0.638 | 0.958 | 1.50× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
