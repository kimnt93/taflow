# MathCosh benchmark (`COSH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 111.73M | 0.008 | 124.25M | 0.035 | 3.86× | 4.30× |
| 10,000 | 0.063 | 158.35M | 0.060 | 166.90M | 0.089 | 1.40× | 1.48× |
| 100,000 | 0.627 | 159.41M | 0.639 | 156.61M | 0.657 | 1.05× | 1.03× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.083 | 0.126 | 1.51× |
| 1 | 5 | 0.339 | 0.505 | 1.49× |
| 1 | 10 | 0.508 | 0.890 | 1.75× |
| 10 | 1 | 0.048 | 0.085 | 1.75× |
| 10 | 5 | 0.208 | 0.415 | 2.00× |
| 10 | 10 | 0.559 | 0.883 | 1.58× |
| 100 | 1 | 0.052 | 0.086 | 1.65× |
| 100 | 5 | 0.224 | 0.445 | 1.99× |
| 100 | 10 | 0.465 | 1.038 | 2.23× |
| 1,000 | 1 | 0.059 | 0.096 | 1.62× |
| 1,000 | 5 | 0.231 | 0.466 | 2.02× |
| 1,000 | 10 | 0.502 | 0.990 | 1.97× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
