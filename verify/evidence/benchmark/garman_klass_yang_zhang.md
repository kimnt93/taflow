# GarmanKlassYangZhang benchmark (`annualized Garman-Klass-Yang-Zhang volatility` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.029 | 34.31M | 0.025 | 40.79M | 0.113 | 3.88× | 4.62× |
| 10,000 | 0.219 | 45.57M | 0.217 | 46.18M | 0.446 | 2.03× | 2.06× |
| 100,000 | 2.842 | 35.19M | 2.127 | 47.01M | 3.642 | 1.28× | 1.71× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.060 | 0.193 | 3.20× |
| 1 | 5 | 0.332 | 0.623 | 1.88× |
| 1 | 10 | 0.400 | 1.197 | 2.99× |
| 10 | 1 | 0.044 | 0.118 | 2.65× |
| 10 | 5 | 0.191 | 0.592 | 3.10× |
| 10 | 10 | 0.407 | 1.218 | 3.00× |
| 100 | 1 | 0.050 | 0.177 | 3.54× |
| 100 | 5 | 0.203 | 0.925 | 4.56× |
| 100 | 10 | 0.424 | 1.596 | 3.77× |
| 1,000 | 1 | 0.074 | 0.202 | 2.73× |
| 1,000 | 5 | 0.218 | 1.037 | 4.76× |
| 1,000 | 10 | 0.446 | 2.250 | 5.05× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
