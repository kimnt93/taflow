# VariablePeriodMovingAverage benchmark (`MAVP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.123 | 8.10M | 0.124 | 8.05M | 0.120 | 0.97× | 0.97× |
| 10,000 | 1.193 | 8.38M | 1.201 | 8.33M | 0.769 | 0.64× | 0.64× |
| 100,000 | 11.685 | 8.56M | 12.952 | 7.72M | 7.672 | 0.66× | 0.59× |
| 1,000,000 | 123.953 | 8.07M | 120.478 | 8.30M | 91.568 | 0.74× | 0.76× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.117 | 0.121 | 1.04× |
| 1 | 5 | 0.375 | 0.510 | 1.36× |
| 1 | 10 | 0.520 | 1.040 | 2.00× |
| 10 | 1 | 0.056 | 0.099 | 1.76× |
| 10 | 5 | 0.246 | 0.496 | 2.01× |
| 10 | 10 | 0.524 | 1.086 | 2.07× |
| 100 | 1 | 0.065 | 0.112 | 1.73× |
| 100 | 5 | 0.266 | 0.531 | 2.00× |
| 100 | 10 | 0.534 | 1.083 | 2.03× |
| 1,000 | 1 | 0.195 | 0.212 | 1.09× |
| 1,000 | 5 | 0.373 | 0.910 | 2.44× |
| 1,000 | 10 | 0.649 | 1.799 | 2.77× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
