# PercentAboveMovingAverage benchmark (`PercentAboveMa` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 206.72M | 0.004 | 279.57M | 12.923 | 2671.50× | 3612.89× |
| 10,000 | 0.036 | 280.20M | 0.029 | 349.47M | 117.759 | 3299.57× | 4115.26× |
| 100,000 | 0.251 | 399.10M | 0.239 | 418.94M | 1152.960 | 4601.48× | 4830.15× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.078 | 0.316 | 4.06× |
| 1 | 5 | 0.264 | 1.477 | 5.59× |
| 1 | 10 | 0.407 | 2.342 | 5.76× |
| 10 | 1 | 0.048 | 0.316 | 6.57× |
| 10 | 5 | 0.183 | 1.571 | 8.57× |
| 10 | 10 | 0.416 | 3.401 | 8.17× |
| 100 | 1 | 0.048 | 1.366 | 28.65× |
| 100 | 5 | 0.223 | 6.930 | 31.01× |
| 100 | 10 | 0.402 | 14.184 | 35.29× |
| 1,000 | 1 | 0.052 | 11.998 | 229.63× |
| 1,000 | 5 | 0.368 | 59.452 | 161.77× |
| 1,000 | 10 | 0.574 | 117.753 | 205.06× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
