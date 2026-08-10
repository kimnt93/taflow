# RollingAverageDeviation benchmark (`AVGDEV` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 51.83M | 0.019 | 51.76M | 0.043 | 2.25× | 2.25× |
| 10,000 | 0.168 | 59.44M | 0.161 | 62.03M | 0.171 | 1.02× | 1.06× |
| 100,000 | 1.644 | 60.84M | 1.557 | 64.23M | 1.410 | 0.86× | 0.91× |
| 1,000,000 | 15.948 | 62.70M | 15.523 | 64.42M | 13.970 | 0.88× | 0.90× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.143 | 0.169 | 1.18× |
| 1 | 5 | 0.335 | 0.500 | 1.49× |
| 1 | 10 | 0.480 | 0.938 | 1.96× |
| 10 | 1 | 0.049 | 0.086 | 1.77× |
| 10 | 5 | 0.248 | 0.459 | 1.85× |
| 10 | 10 | 0.478 | 0.911 | 1.91× |
| 100 | 1 | 0.050 | 0.098 | 1.96× |
| 100 | 5 | 0.241 | 0.445 | 1.85× |
| 100 | 10 | 0.672 | 0.910 | 1.35× |
| 1,000 | 1 | 0.061 | 0.105 | 1.71× |
| 1,000 | 5 | 0.218 | 0.491 | 2.25× |
| 1,000 | 10 | 0.480 | 1.099 | 2.29× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
