# Falling benchmark (`period-over-period falling` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.002 | 531.73M | 0.001 | 957.59M | 0.029 | 15.33× | 27.61× |
| 10,000 | 0.008 | 1.28G | 0.005 | 2.04G | 0.037 | 4.77× | 7.57× |
| 100,000 | 0.070 | 1.43G | 0.046 | 2.18G | 0.133 | 1.91× | 2.91× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.106 | 0.098 | 0.92× |
| 1 | 5 | 0.293 | 0.464 | 1.58× |
| 1 | 10 | 0.361 | 0.905 | 2.50× |
| 10 | 1 | 0.040 | 0.093 | 2.33× |
| 10 | 5 | 0.180 | 0.441 | 2.45× |
| 10 | 10 | 0.375 | 0.889 | 2.37× |
| 100 | 1 | 0.044 | 0.088 | 2.00× |
| 100 | 5 | 0.189 | 0.434 | 2.30× |
| 100 | 10 | 0.384 | 0.890 | 2.32× |
| 1,000 | 1 | 0.040 | 0.090 | 2.23× |
| 1,000 | 5 | 0.178 | 0.493 | 2.77× |
| 1,000 | 10 | 0.388 | 1.098 | 2.83× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
