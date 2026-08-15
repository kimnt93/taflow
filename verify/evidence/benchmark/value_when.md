# ValueWhen benchmark (`last value when condition` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 239.29M | 0.003 | 331.21M | 0.158 | 37.71× | 52.20× |
| 10,000 | 0.024 | 413.19M | 0.021 | 475.95M | 1.970 | 81.42× | 93.78× |
| 100,000 | 0.213 | 468.53M | 0.184 | 542.42M | 14.561 | 68.22× | 78.98× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.077 | 0.103 | 1.33× |
| 1 | 5 | 0.285 | 0.343 | 1.20× |
| 1 | 10 | 0.392 | 0.669 | 1.70× |
| 10 | 1 | 0.042 | 0.064 | 1.52× |
| 10 | 5 | 0.193 | 0.324 | 1.68× |
| 10 | 10 | 0.396 | 0.679 | 1.72× |
| 100 | 1 | 0.046 | 0.082 | 1.78× |
| 100 | 5 | 0.184 | 0.380 | 2.06× |
| 100 | 10 | 0.374 | 0.786 | 2.10× |
| 1,000 | 1 | 0.047 | 0.215 | 4.57× |
| 1,000 | 5 | 0.194 | 1.035 | 5.35× |
| 1,000 | 10 | 0.434 | 2.167 | 4.99× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
