# RollingCointegration benchmark (`Cointegration` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.350 | 2.86M | 0.347 | 2.88M | 3.394 | 9.70× | 9.77× |
| 10,000 | 3.609 | 2.77M | 3.791 | 2.64M | 31.304 | 8.67× | 8.26× |
| 100,000 | 36.043 | 2.77M | 36.170 | 2.76M | 320.616 | 8.90× | 8.86× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.078 | 0.305 | 3.90× |
| 1 | 5 | 0.310 | 1.304 | 4.21× |
| 1 | 10 | 0.464 | 2.619 | 5.65× |
| 10 | 1 | 0.053 | 0.235 | 4.48× |
| 10 | 5 | 0.229 | 1.465 | 6.40× |
| 10 | 10 | 0.415 | 2.733 | 6.58× |
| 100 | 1 | 0.077 | 0.534 | 6.91× |
| 100 | 5 | 0.218 | 2.616 | 11.98× |
| 100 | 10 | 0.495 | 5.293 | 10.70× |
| 1,000 | 1 | 0.422 | 3.743 | 8.87× |
| 1,000 | 5 | 0.815 | 19.142 | 23.49× |
| 1,000 | 10 | 1.145 | 36.244 | 31.67× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
