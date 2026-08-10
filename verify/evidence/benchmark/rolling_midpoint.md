# RollingMidpoint benchmark (`MIDPOINT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 134.24M | 0.007 | 149.23M | 0.035 | 4.66× | 5.18× |
| 10,000 | 0.074 | 135.67M | 0.073 | 137.69M | 0.102 | 1.39× | 1.41× |
| 100,000 | 0.734 | 136.18M | 0.713 | 140.24M | 0.653 | 0.89× | 0.92× |
| 1,000,000 | 8.636 | 115.80M | 9.915 | 100.85M | 6.814 | 0.79× | 0.69× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.081 | 0.123 | 1.52× |
| 1 | 5 | 0.296 | 0.502 | 1.70× |
| 1 | 10 | 0.461 | 0.896 | 1.94× |
| 10 | 1 | 0.050 | 0.092 | 1.85× |
| 10 | 5 | 0.223 | 0.439 | 1.97× |
| 10 | 10 | 0.444 | 0.913 | 2.06× |
| 100 | 1 | 0.048 | 0.099 | 2.04× |
| 100 | 5 | 0.223 | 0.447 | 2.00× |
| 100 | 10 | 0.464 | 0.900 | 1.94× |
| 1,000 | 1 | 0.057 | 0.098 | 1.72× |
| 1,000 | 5 | 0.248 | 0.478 | 1.93× |
| 1,000 | 10 | 0.486 | 1.001 | 2.06× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
