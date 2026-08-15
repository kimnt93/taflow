# CandleLongLine benchmark (`CDLLONGLINE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 77.14M | 0.010 | 95.89M | 0.037 | 2.86× | 3.55× |
| 10,000 | 0.152 | 65.88M | 0.137 | 73.00M | 0.175 | 1.15× | 1.28× |
| 100,000 | 1.457 | 68.64M | 1.427 | 70.09M | 1.478 | 1.01× | 1.04× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.068 | 0.099 | 1.46× |
| 1 | 5 | 0.279 | 0.440 | 1.57× |
| 1 | 10 | 0.415 | 0.956 | 2.31× |
| 10 | 1 | 0.045 | 0.094 | 2.10× |
| 10 | 5 | 0.192 | 0.422 | 2.20× |
| 10 | 10 | 0.416 | 0.956 | 2.30× |
| 100 | 1 | 0.047 | 0.088 | 1.87× |
| 100 | 5 | 0.197 | 0.441 | 2.24× |
| 100 | 10 | 0.417 | 0.904 | 2.17× |
| 1,000 | 1 | 0.061 | 0.101 | 1.67× |
| 1,000 | 5 | 0.216 | 0.561 | 2.60× |
| 1,000 | 10 | 0.441 | 1.084 | 2.46× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
