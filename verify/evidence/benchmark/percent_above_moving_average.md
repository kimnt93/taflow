# PercentAboveMovingAverage benchmark (`PercentAboveMa` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 144.03M | 0.006 | 178.97M | 12.447 | 1792.76× | 2227.68× |
| 10,000 | 0.030 | 332.28M | 0.029 | 347.58M | 120.245 | 3995.50× | 4179.49× |
| 100,000 | 0.250 | 399.76M | 0.241 | 414.24M | 1232.240 | 4925.96× | 5104.41× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.115 | 0.283 | 2.47× |
| 1 | 5 | 0.296 | 1.066 | 3.60× |
| 1 | 10 | 0.518 | 2.204 | 4.26× |
| 10 | 1 | 0.053 | 0.324 | 6.14× |
| 10 | 5 | 0.246 | 1.985 | 8.08× |
| 10 | 10 | 0.504 | 3.355 | 6.65× |
| 100 | 1 | 0.051 | 1.423 | 27.71× |
| 100 | 5 | 0.241 | 7.592 | 31.50× |
| 100 | 10 | 0.531 | 15.345 | 28.88× |
| 1,000 | 1 | 0.060 | 12.464 | 206.79× |
| 1,000 | 5 | 0.446 | 63.420 | 142.33× |
| 1,000 | 10 | 0.568 | 128.097 | 225.53× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
