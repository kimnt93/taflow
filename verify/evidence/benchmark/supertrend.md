# Supertrend benchmark (`supertrend` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.104 | 9.57M | 0.088 | 11.40M | 1.767 | 16.91× | 20.15× |
| 10,000 | 0.828 | 12.08M | 1.128 | 8.86M | 3.273 | 3.95× | 2.90× |
| 100,000 | 7.567 | 13.22M | 7.500 | 13.33M | 10.651 | 1.41× | 1.42× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.172 | 0.286 | 1.66× |
| 1 | 5 | 0.461 | 1.146 | 2.49× |
| 1 | 10 | 0.652 | 2.172 | 3.33× |
| 10 | 1 | 0.078 | 1.712 | 21.87× |
| 10 | 5 | 0.339 | 8.310 | 24.53× |
| 10 | 10 | 0.694 | 16.400 | 23.64× |
| 100 | 1 | 0.121 | 1.593 | 13.18× |
| 100 | 5 | 0.356 | 8.216 | 23.09× |
| 100 | 10 | 0.677 | 16.222 | 23.96× |
| 1,000 | 1 | 0.168 | 1.716 | 10.21× |
| 1,000 | 5 | 0.336 | 9.015 | 26.83× |
| 1,000 | 10 | 0.736 | 18.344 | 24.94× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
