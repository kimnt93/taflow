# SessionExtrema benchmark (`explicit-session extrema` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 115.48M | 0.006 | 158.39M | 0.501 | 57.81× | 79.29× |
| 10,000 | 0.057 | 176.70M | 0.048 | 206.63M | 4.924 | 87.01× | 101.75× |
| 100,000 | 0.513 | 194.76M | 0.447 | 223.54M | 49.481 | 96.37× | 110.61× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.079 | 0.099 | 1.26× |
| 1 | 5 | 0.202 | 0.335 | 1.66× |
| 1 | 10 | 0.367 | 0.604 | 1.65× |
| 10 | 1 | 0.042 | 0.072 | 1.72× |
| 10 | 5 | 0.183 | 0.320 | 1.75× |
| 10 | 10 | 0.397 | 0.694 | 1.75× |
| 100 | 1 | 0.042 | 0.123 | 2.94× |
| 100 | 5 | 0.189 | 0.558 | 2.95× |
| 100 | 10 | 0.431 | 1.164 | 2.70× |
| 1,000 | 1 | 0.052 | 0.584 | 11.28× |
| 1,000 | 5 | 0.203 | 2.966 | 14.64× |
| 1,000 | 10 | 0.432 | 5.780 | 13.38× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
