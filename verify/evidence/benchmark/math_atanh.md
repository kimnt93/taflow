# MathAtanh benchmark (`numpy.arctanh` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 74.72M | 0.013 | 76.64M | 0.027 | 1.99× | 2.04× |
| 10,000 | 0.114 | 87.35M | 0.110 | 90.68M | 0.152 | 1.33× | 1.38× |
| 100,000 | 1.096 | 91.28M | 1.080 | 92.59M | 1.386 | 1.27× | 1.28× |
| 1,000,000 | 12.114 | 82.55M | 10.929 | 91.50M | 12.808 | 1.06× | 1.17× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.115 | 0.077 | 0.67× |
| 1 | 5 | 0.281 | 0.294 | 1.04× |
| 1 | 10 | 0.487 | 0.560 | 1.15× |
| 10 | 1 | 0.051 | 0.060 | 1.18× |
| 10 | 5 | 0.210 | 0.266 | 1.27× |
| 10 | 10 | 0.475 | 0.556 | 1.17× |
| 100 | 1 | 0.046 | 0.054 | 1.18× |
| 100 | 5 | 0.212 | 0.272 | 1.28× |
| 100 | 10 | 0.487 | 0.594 | 1.22× |
| 1,000 | 1 | 0.064 | 0.073 | 1.15× |
| 1,000 | 5 | 0.243 | 0.322 | 1.33× |
| 1,000 | 10 | 0.506 | 0.824 | 1.63× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
