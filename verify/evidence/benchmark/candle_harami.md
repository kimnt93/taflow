# CandleHarami benchmark (`CDLHARAMI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.024 | 42.22M | 0.019 | 53.57M | 0.037 | 1.57× | 1.99× |
| 10,000 | 0.146 | 68.57M | 0.142 | 70.18M | 0.146 | 1.00× | 1.02× |
| 100,000 | 1.485 | 67.33M | 1.417 | 70.58M | 1.235 | 0.83× | 0.87× |
| 1,000,000 | 15.021 | 66.57M | 14.404 | 69.42M | 11.923 | 0.79× | 0.83× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.162 | 0.130 | 0.80× |
| 1 | 5 | 0.330 | 0.506 | 1.53× |
| 1 | 10 | 0.627 | 0.943 | 1.50× |
| 10 | 1 | 0.067 | 0.097 | 1.45× |
| 10 | 5 | 0.285 | 0.528 | 1.85× |
| 10 | 10 | 0.630 | 1.001 | 1.59× |
| 100 | 1 | 0.064 | 0.089 | 1.38× |
| 100 | 5 | 0.285 | 0.459 | 1.61× |
| 100 | 10 | 0.611 | 1.002 | 1.64× |
| 1,000 | 1 | 0.076 | 0.100 | 1.30× |
| 1,000 | 5 | 0.328 | 0.552 | 1.68× |
| 1,000 | 10 | 0.616 | 1.094 | 1.78× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
