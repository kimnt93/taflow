# CandleTasukiGap benchmark (`CDLTASUKIGAP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 83.24M | 0.009 | 109.03M | 0.046 | 3.85× | 5.04× |
| 10,000 | 0.078 | 127.72M | 0.075 | 132.74M | 0.192 | 2.46× | 2.55× |
| 100,000 | 0.804 | 124.41M | 0.714 | 140.08M | 1.488 | 1.85× | 2.08× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.072 | 0.123 | 1.69× |
| 1 | 5 | 0.282 | 0.481 | 1.70× |
| 1 | 10 | 0.403 | 0.937 | 2.33× |
| 10 | 1 | 0.051 | 0.088 | 1.71× |
| 10 | 5 | 0.200 | 0.448 | 2.24× |
| 10 | 10 | 0.391 | 0.909 | 2.33× |
| 100 | 1 | 0.045 | 0.085 | 1.89× |
| 100 | 5 | 0.182 | 0.482 | 2.65× |
| 100 | 10 | 0.405 | 0.908 | 2.24× |
| 1,000 | 1 | 0.046 | 0.105 | 2.26× |
| 1,000 | 5 | 0.188 | 0.509 | 2.70× |
| 1,000 | 10 | 0.453 | 1.101 | 2.43× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
