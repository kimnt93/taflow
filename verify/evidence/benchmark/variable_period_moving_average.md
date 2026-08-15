# VariablePeriodMovingAverage benchmark (`MAVP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.085 | 11.74M | 0.083 | 12.12M | 0.107 | 1.25× | 1.29× |
| 10,000 | 0.795 | 12.58M | 0.802 | 12.47M | 0.743 | 0.93× | 0.93× |
| 100,000 | 8.585 | 11.65M | 8.519 | 11.74M | 7.078 | 0.82× | 0.83× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.073 | 0.125 | 1.72× |
| 1 | 5 | 0.385 | 0.521 | 1.35× |
| 1 | 10 | 0.455 | 1.015 | 2.23× |
| 10 | 1 | 0.045 | 0.098 | 2.16× |
| 10 | 5 | 0.213 | 0.475 | 2.23× |
| 10 | 10 | 0.437 | 1.000 | 2.29× |
| 100 | 1 | 0.059 | 0.103 | 1.76× |
| 100 | 5 | 0.238 | 0.519 | 2.18× |
| 100 | 10 | 0.451 | 1.066 | 2.36× |
| 1,000 | 1 | 0.128 | 0.190 | 1.48× |
| 1,000 | 5 | 0.289 | 0.892 | 3.09× |
| 1,000 | 10 | 0.540 | 1.866 | 3.45× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
