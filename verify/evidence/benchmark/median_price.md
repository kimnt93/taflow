# MedianPrice benchmark (`MEDPRICE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 191.50M | 0.004 | 247.53M | 0.030 | 5.84× | 7.55× |
| 10,000 | 0.019 | 529.07M | 0.016 | 642.56M | 0.032 | 1.68× | 2.04× |
| 100,000 | 0.156 | 641.50M | 0.129 | 775.25M | 0.067 | 0.43× | 0.52× |
| 1,000,000 | 1.953 | 512.16M | 1.532 | 652.89M | 0.877 | 0.45× | 0.57× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.073 | 0.125 | 1.70× |
| 1 | 5 | 0.330 | 0.466 | 1.41× |
| 1 | 10 | 0.516 | 0.912 | 1.77× |
| 10 | 1 | 0.051 | 0.082 | 1.61× |
| 10 | 5 | 0.248 | 0.446 | 1.80× |
| 10 | 10 | 0.489 | 0.869 | 1.78× |
| 100 | 1 | 0.051 | 0.088 | 1.73× |
| 100 | 5 | 0.303 | 0.443 | 1.46× |
| 100 | 10 | 0.513 | 0.877 | 1.71× |
| 1,000 | 1 | 0.056 | 0.095 | 1.69× |
| 1,000 | 5 | 0.227 | 0.409 | 1.81× |
| 1,000 | 10 | 0.521 | 0.894 | 1.72× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
