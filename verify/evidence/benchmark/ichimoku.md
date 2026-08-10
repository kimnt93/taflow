# Ichimoku benchmark (`causal ichimoku components` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.096 | 10.41M | 0.089 | 11.29M | 0.451 | 4.69× | 5.09× |
| 10,000 | 0.927 | 10.79M | 0.886 | 11.29M | 2.572 | 2.78× | 2.90× |
| 100,000 | 9.048 | 11.05M | 9.173 | 10.90M | 23.666 | 2.62× | 2.58× |
| 1,000,000 | 118.271 | 8.46M | 90.280 | 11.08M | 244.248 | 2.07× | 2.71× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.086 | 0.185 | 2.16× |
| 1 | 5 | 0.346 | 0.681 | 1.97× |
| 1 | 10 | 0.499 | 1.361 | 2.73× |
| 10 | 1 | 0.055 | 0.223 | 4.04× |
| 10 | 5 | 0.245 | 1.033 | 4.22× |
| 10 | 10 | 0.526 | 2.114 | 4.02× |
| 100 | 1 | 0.066 | 0.337 | 5.08× |
| 100 | 5 | 0.280 | 1.793 | 6.40× |
| 100 | 10 | 0.546 | 3.715 | 6.80× |
| 1,000 | 1 | 0.156 | 0.576 | 3.70× |
| 1,000 | 5 | 0.353 | 2.199 | 6.23× |
| 1,000 | 10 | 0.608 | 4.497 | 7.40× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
