# NormalizedAverageTrueRange benchmark (`NATR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.017 | 60.58M | 0.011 | 88.95M | 0.043 | 2.63× | 3.86× |
| 10,000 | 0.085 | 117.48M | 0.068 | 146.61M | 0.095 | 1.11× | 1.39× |
| 100,000 | 0.696 | 143.70M | 0.661 | 151.28M | 0.612 | 0.88× | 0.93× |
| 1,000,000 | 7.402 | 135.10M | 7.002 | 142.81M | 6.757 | 0.91× | 0.96× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.110 | 0.124 | 1.13× |
| 1 | 5 | 0.272 | 0.534 | 1.97× |
| 1 | 10 | 0.498 | 0.979 | 1.96× |
| 10 | 1 | 0.052 | 0.095 | 1.83× |
| 10 | 5 | 0.273 | 0.520 | 1.90× |
| 10 | 10 | 0.546 | 1.007 | 1.84× |
| 100 | 1 | 0.053 | 0.102 | 1.92× |
| 100 | 5 | 0.265 | 0.502 | 1.90× |
| 100 | 10 | 0.548 | 1.071 | 1.95× |
| 1,000 | 1 | 0.062 | 0.102 | 1.63× |
| 1,000 | 5 | 0.253 | 0.494 | 1.95× |
| 1,000 | 10 | 0.557 | 1.135 | 2.04× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
