# RateOfChangeRatio benchmark (`ROCR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 173.69M | 0.005 | 218.22M | 0.034 | 5.96× | 7.49× |
| 10,000 | 0.026 | 388.22M | 0.021 | 465.64M | 0.042 | 1.62× | 1.94× |
| 100,000 | 0.209 | 477.42M | 0.186 | 537.44M | 0.149 | 0.71× | 0.80× |
| 1,000,000 | 2.474 | 404.19M | 2.217 | 451.07M | 1.215 | 0.49× | 0.55× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.119 | 0.135 | 1.13× |
| 1 | 5 | 0.277 | 0.539 | 1.95× |
| 1 | 10 | 0.512 | 1.017 | 1.99× |
| 10 | 1 | 0.052 | 0.090 | 1.72× |
| 10 | 5 | 0.247 | 0.502 | 2.03× |
| 10 | 10 | 0.530 | 1.067 | 2.01× |
| 100 | 1 | 0.052 | 0.092 | 1.78× |
| 100 | 5 | 0.245 | 0.453 | 1.85× |
| 100 | 10 | 0.570 | 1.080 | 1.89× |
| 1,000 | 1 | 0.057 | 0.092 | 1.62× |
| 1,000 | 5 | 0.256 | 0.471 | 1.84× |
| 1,000 | 10 | 0.541 | 1.050 | 1.94× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
