# MathAtan benchmark (`ATAN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 98.24M | 0.009 | 116.67M | 0.032 | 3.14× | 3.72× |
| 10,000 | 0.076 | 132.04M | 0.065 | 154.43M | 0.082 | 1.08× | 1.27× |
| 100,000 | 0.669 | 149.52M | 0.657 | 152.24M | 0.617 | 0.92× | 0.94× |
| 1,000,000 | 7.604 | 131.50M | 6.937 | 144.16M | 5.680 | 0.75× | 0.82× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.093 | 0.132 | 1.41× |
| 1 | 5 | 0.257 | 0.490 | 1.91× |
| 1 | 10 | 0.535 | 1.081 | 2.02× |
| 10 | 1 | 0.063 | 0.098 | 1.57× |
| 10 | 5 | 0.279 | 0.463 | 1.66× |
| 10 | 10 | 0.462 | 0.890 | 1.93× |
| 100 | 1 | 0.052 | 0.098 | 1.89× |
| 100 | 5 | 0.224 | 0.416 | 1.86× |
| 100 | 10 | 0.481 | 0.893 | 1.86× |
| 1,000 | 1 | 0.056 | 0.098 | 1.73× |
| 1,000 | 5 | 0.245 | 0.456 | 1.86× |
| 1,000 | 10 | 0.495 | 0.966 | 1.95× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
