# AverageDailyDollarValue benchmark (`rolling average dollar volume` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 142.39M | 0.006 | 171.15M | 0.082 | 11.67× | 14.03× |
| 10,000 | 0.051 | 197.88M | 0.048 | 207.60M | 0.269 | 5.33× | 5.59× |
| 100,000 | 0.475 | 210.57M | 0.452 | 221.46M | 2.212 | 4.66× | 4.90× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.085 | 0.143 | 1.68× |
| 1 | 5 | 0.288 | 0.531 | 1.84× |
| 1 | 10 | 0.377 | 1.040 | 2.76× |
| 10 | 1 | 0.047 | 0.105 | 2.25× |
| 10 | 5 | 0.203 | 0.517 | 2.55× |
| 10 | 10 | 0.390 | 1.080 | 2.77× |
| 100 | 1 | 0.049 | 0.144 | 2.91× |
| 100 | 5 | 0.188 | 0.700 | 3.73× |
| 100 | 10 | 0.421 | 1.466 | 3.48× |
| 1,000 | 1 | 0.047 | 0.164 | 3.47× |
| 1,000 | 5 | 0.194 | 0.810 | 4.18× |
| 1,000 | 10 | 0.439 | 1.678 | 3.82× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
