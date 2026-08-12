# RollingProfitFactor benchmark (`ProfitFactor` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.029 | 34.30M | 0.038 | 26.46M | 0.187 | 6.41× | 4.94× |
| 10,000 | 0.265 | 37.75M | 0.255 | 39.16M | 0.677 | 2.56× | 2.65× |
| 100,000 | 2.494 | 40.10M | 2.460 | 40.65M | 5.567 | 2.23× | 2.26× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.090 | 0.254 | 2.83× |
| 1 | 5 | 0.280 | 1.133 | 4.04× |
| 1 | 10 | 0.542 | 2.358 | 4.35× |
| 10 | 1 | 0.053 | 0.206 | 3.91× |
| 10 | 5 | 0.289 | 1.152 | 3.99× |
| 10 | 10 | 0.545 | 2.582 | 4.74× |
| 100 | 1 | 0.069 | 0.205 | 2.97× |
| 100 | 5 | 0.280 | 1.082 | 3.86× |
| 100 | 10 | 0.559 | 2.499 | 4.47× |
| 1,000 | 1 | 0.082 | 0.269 | 3.28× |
| 1,000 | 5 | 0.273 | 1.382 | 5.06× |
| 1,000 | 10 | 0.604 | 3.075 | 5.09× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
