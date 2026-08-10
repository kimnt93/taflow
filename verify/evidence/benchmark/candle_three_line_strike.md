# CandleThreeLineStrike benchmark (`CDL3LINESTRIKE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 80.33M | 0.013 | 76.42M | 0.043 | 3.45× | 3.29× |
| 10,000 | 0.070 | 143.45M | 0.068 | 147.55M | 0.113 | 1.63× | 1.67× |
| 100,000 | 0.775 | 129.10M | 0.780 | 128.21M | 0.789 | 1.02× | 1.01× |
| 1,000,000 | 8.498 | 117.67M | 8.070 | 123.92M | 8.373 | 0.99× | 1.04× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.103 | 0.101 | 0.98× |
| 1 | 5 | 0.272 | 0.461 | 1.70× |
| 1 | 10 | 0.523 | 0.908 | 1.74× |
| 10 | 1 | 0.063 | 0.091 | 1.43× |
| 10 | 5 | 0.257 | 0.433 | 1.69× |
| 10 | 10 | 0.533 | 0.893 | 1.68× |
| 100 | 1 | 0.056 | 0.094 | 1.67× |
| 100 | 5 | 0.269 | 0.427 | 1.59× |
| 100 | 10 | 0.557 | 0.920 | 1.65× |
| 1,000 | 1 | 0.068 | 0.099 | 1.45× |
| 1,000 | 5 | 0.285 | 0.481 | 1.69× |
| 1,000 | 10 | 0.547 | 0.997 | 1.82× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
