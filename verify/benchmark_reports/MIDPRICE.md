# RollingMidprice benchmark (`MIDPRICE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 94.75M | 0.008 | 119.53M | 0.040 | 3.75× | 4.73× |
| 10,000 | 0.080 | 125.38M | 0.078 | 127.76M | 0.108 | 1.35× | 1.38× |
| 100,000 | 0.777 | 128.67M | 0.772 | 129.57M | 0.745 | 0.96× | 0.97× |
| 1,000,000 | 9.783 | 102.22M | 8.884 | 112.57M | 7.221 | 0.74× | 0.81× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.070 | 0.127 | 1.81× |
| 1 | 5 | 0.301 | 0.546 | 1.82× |
| 1 | 10 | 0.559 | 1.053 | 1.88× |
| 10 | 1 | 0.053 | 0.092 | 1.74× |
| 10 | 5 | 0.237 | 0.465 | 1.96× |
| 10 | 10 | 0.560 | 1.014 | 1.81× |
| 100 | 1 | 0.052 | 0.098 | 1.87× |
| 100 | 5 | 0.258 | 0.486 | 1.89× |
| 100 | 10 | 0.539 | 1.006 | 1.87× |
| 1,000 | 1 | 0.061 | 0.102 | 1.68× |
| 1,000 | 5 | 0.265 | 0.531 | 2.01× |
| 1,000 | 10 | 0.530 | 1.121 | 2.11× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
