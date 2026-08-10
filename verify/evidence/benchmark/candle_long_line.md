# CandleLongLine benchmark (`CDLLONGLINE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 52.95M | 0.016 | 64.20M | 0.036 | 1.91× | 2.32× |
| 10,000 | 0.159 | 63.02M | 0.154 | 64.89M | 0.178 | 1.12× | 1.16× |
| 100,000 | 1.530 | 65.37M | 1.493 | 67.00M | 1.563 | 1.02× | 1.05× |
| 1,000,000 | 15.487 | 64.57M | 15.225 | 65.68M | 15.363 | 0.99× | 1.01× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.115 | 0.108 | 0.94× |
| 1 | 5 | 0.276 | 0.478 | 1.73× |
| 1 | 10 | 0.541 | 0.918 | 1.70× |
| 10 | 1 | 0.056 | 0.093 | 1.66× |
| 10 | 5 | 0.240 | 0.413 | 1.72× |
| 10 | 10 | 0.514 | 0.972 | 1.89× |
| 100 | 1 | 0.057 | 0.095 | 1.69× |
| 100 | 5 | 0.259 | 0.425 | 1.64× |
| 100 | 10 | 0.523 | 0.907 | 1.73× |
| 1,000 | 1 | 0.075 | 0.112 | 1.50× |
| 1,000 | 5 | 0.308 | 0.526 | 1.71× |
| 1,000 | 10 | 0.569 | 1.064 | 1.87× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
