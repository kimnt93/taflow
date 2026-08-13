# CandleLadderBottom benchmark (`CDLLADDERBOTTOM` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.102 | 9.76M | 0.091 | 11.01M | 0.032 | 0.31× | 0.35× |
| 10,000 | 0.831 | 12.03M | 0.845 | 11.83M | 0.082 | 0.10× | 0.10× |
| 100,000 | 8.606 | 11.62M | 7.940 | 12.59M | 0.566 | 0.07× | 0.07× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.090 | 0.109 | 1.21× |
| 1 | 5 | 0.347 | 0.474 | 1.37× |
| 1 | 10 | 0.659 | 0.896 | 1.36× |
| 10 | 1 | 0.069 | 0.085 | 1.24× |
| 10 | 5 | 0.331 | 0.453 | 1.37× |
| 10 | 10 | 0.662 | 0.917 | 1.38× |
| 100 | 1 | 0.080 | 0.088 | 1.10× |
| 100 | 5 | 0.327 | 0.455 | 1.39× |
| 100 | 10 | 0.661 | 0.914 | 1.38× |
| 1,000 | 1 | 0.151 | 0.098 | 0.65× |
| 1,000 | 5 | 0.338 | 0.471 | 1.39× |
| 1,000 | 10 | 0.682 | 1.029 | 1.51× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
