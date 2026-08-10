# RollingMinimumIndex benchmark (`MININDEX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 141.97M | 0.006 | 154.09M | 0.036 | 5.17× | 5.61× |
| 10,000 | 0.059 | 168.49M | 0.054 | 184.97M | 0.099 | 1.67× | 1.83× |
| 100,000 | 0.611 | 163.73M | 0.634 | 157.75M | 0.746 | 1.22× | 1.18× |
| 1,000,000 | 5.864 | 170.52M | 5.447 | 183.57M | 7.259 | 1.24× | 1.33× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.125 | 0.201 | 1.61× |
| 1 | 5 | 0.340 | 0.452 | 1.33× |
| 1 | 10 | 0.459 | 0.947 | 2.06× |
| 10 | 1 | 0.048 | 0.089 | 1.84× |
| 10 | 5 | 0.220 | 0.428 | 1.95× |
| 10 | 10 | 0.471 | 0.942 | 2.00× |
| 100 | 1 | 0.063 | 0.098 | 1.55× |
| 100 | 5 | 0.225 | 0.455 | 2.02× |
| 100 | 10 | 0.513 | 0.933 | 1.82× |
| 1,000 | 1 | 0.053 | 0.097 | 1.83× |
| 1,000 | 5 | 0.257 | 0.503 | 1.95× |
| 1,000 | 10 | 0.521 | 0.996 | 1.91× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
