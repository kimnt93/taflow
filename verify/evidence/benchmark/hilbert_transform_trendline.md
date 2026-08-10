# HilbertTransformTrendline benchmark (`HT_TRENDLINE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.083 | 12.11M | 0.075 | 13.31M | 0.084 | 1.02× | 1.12× |
| 10,000 | 0.762 | 13.13M | 0.750 | 13.33M | 0.643 | 0.84× | 0.86× |
| 100,000 | 7.457 | 13.41M | 7.409 | 13.50M | 6.226 | 0.83× | 0.84× |
| 1,000,000 | 76.932 | 13.00M | 75.614 | 13.23M | 68.663 | 0.89× | 0.91× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.079 | 0.132 | 1.67× |
| 1 | 5 | 0.282 | 0.553 | 1.96× |
| 1 | 10 | 0.584 | 1.027 | 1.76× |
| 10 | 1 | 0.051 | 0.092 | 1.83× |
| 10 | 5 | 0.245 | 0.531 | 2.17× |
| 10 | 10 | 0.538 | 1.111 | 2.07× |
| 100 | 1 | 0.063 | 0.114 | 1.81× |
| 100 | 5 | 0.288 | 0.563 | 1.95× |
| 100 | 10 | 0.655 | 1.240 | 1.89× |
| 1,000 | 1 | 0.131 | 0.159 | 1.21× |
| 1,000 | 5 | 0.345 | 0.990 | 2.87× |
| 1,000 | 10 | 0.804 | 1.764 | 2.20× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
