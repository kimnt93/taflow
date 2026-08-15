# AverageDirectionalIndexRating benchmark (`ADXR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 75.46M | 0.011 | 93.09M | 0.043 | 3.27× | 4.04× |
| 10,000 | 0.099 | 100.99M | 0.094 | 106.76M | 0.129 | 1.30× | 1.37× |
| 100,000 | 0.947 | 105.55M | 0.893 | 112.01M | 1.048 | 1.11× | 1.17× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.087 | 0.147 | 1.69× |
| 1 | 5 | 0.323 | 0.493 | 1.53× |
| 1 | 10 | 0.399 | 0.942 | 2.36× |
| 10 | 1 | 0.043 | 0.116 | 2.72× |
| 10 | 5 | 0.227 | 0.488 | 2.15× |
| 10 | 10 | 0.394 | 0.950 | 2.41× |
| 100 | 1 | 0.043 | 0.097 | 2.23× |
| 100 | 5 | 0.207 | 0.484 | 2.34× |
| 100 | 10 | 0.490 | 1.077 | 2.20× |
| 1,000 | 1 | 0.064 | 0.116 | 1.81× |
| 1,000 | 5 | 0.226 | 0.525 | 2.32× |
| 1,000 | 10 | 0.385 | 1.196 | 3.11× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
