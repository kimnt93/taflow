# TimeOfDayReturnProfile benchmark (`TimeOfDayReturnProfile` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.052 | 19.21M | 0.043 | 23.04M | 1.669 | 32.06× | 38.46× |
| 10,000 | 0.373 | 26.84M | 0.357 | 27.99M | 21.039 | 56.47× | 58.90× |
| 100,000 | 6.093 | 16.41M | 3.564 | 28.06M | 172.803 | 28.36× | 48.49× |
| 1,000,000 | 160.331 | 6.24M | 81.939 | 12.20M | 1925.997 | 12.01× | 23.51× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.148 | 0.323 | 2.18× |
| 1 | 5 | 0.385 | 1.477 | 3.83× |
| 1 | 10 | 0.550 | 3.114 | 5.67× |
| 10 | 1 | 0.072 | 0.349 | 4.86× |
| 10 | 5 | 0.336 | 1.699 | 5.06× |
| 10 | 10 | 0.608 | 3.339 | 5.49× |
| 100 | 1 | 0.068 | 0.461 | 6.76× |
| 100 | 5 | 0.308 | 2.373 | 7.71× |
| 100 | 10 | 0.612 | 4.350 | 7.10× |
| 1,000 | 1 | 0.094 | 1.995 | 21.26× |
| 1,000 | 5 | 0.331 | 16.175 | 48.89× |
| 1,000 | 10 | 0.786 | 19.678 | 25.04× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
