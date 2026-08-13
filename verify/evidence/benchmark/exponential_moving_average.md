# ExponentialMovingAverage benchmark (`EMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.034 | 29.72M | 0.028 | 35.56M | 0.036 | 1.06× | 1.26× |
| 10,000 | 0.216 | 46.32M | 0.206 | 48.45M | 0.067 | 0.31× | 0.33× |
| 100,000 | 2.013 | 49.67M | 1.979 | 50.54M | 0.300 | 0.15× | 0.15× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.180 | 0.121 | 0.67× |
| 1 | 5 | 0.509 | 0.477 | 0.94× |
| 1 | 10 | 0.622 | 0.940 | 1.51× |
| 10 | 1 | 0.076 | 0.101 | 1.33× |
| 10 | 5 | 0.304 | 0.441 | 1.45× |
| 10 | 10 | 0.581 | 0.914 | 1.57× |
| 100 | 1 | 0.075 | 0.091 | 1.22× |
| 100 | 5 | 0.306 | 0.448 | 1.46× |
| 100 | 10 | 0.631 | 0.917 | 1.45× |
| 1,000 | 1 | 0.093 | 0.093 | 1.00× |
| 1,000 | 5 | 0.298 | 0.449 | 1.51× |
| 1,000 | 10 | 0.623 | 0.958 | 1.54× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
