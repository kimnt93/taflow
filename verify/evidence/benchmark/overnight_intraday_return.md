# OvernightIntradayReturn benchmark (`OvernightIntradayReturn` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 50.31M | 0.015 | 67.82M | 0.681 | 34.27× | 46.19× |
| 10,000 | 0.081 | 123.35M | 0.081 | 123.47M | 5.418 | 66.84× | 66.90× |
| 100,000 | 0.714 | 140.04M | 0.649 | 154.01M | 52.241 | 73.16× | 80.46× |
| 1,000,000 | 7.948 | 125.82M | 7.271 | 137.53M | 580.133 | 72.99× | 79.79× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.082 | 0.319 | 3.87× |
| 1 | 5 | 0.312 | 1.224 | 3.92× |
| 1 | 10 | 0.594 | 2.581 | 4.34× |
| 10 | 1 | 0.065 | 0.245 | 3.77× |
| 10 | 5 | 0.275 | 1.411 | 5.12× |
| 10 | 10 | 0.575 | 2.664 | 4.63× |
| 100 | 1 | 0.064 | 0.308 | 4.82× |
| 100 | 5 | 0.311 | 1.542 | 4.96× |
| 100 | 10 | 0.604 | 3.092 | 5.12× |
| 1,000 | 1 | 0.073 | 0.962 | 13.22× |
| 1,000 | 5 | 0.308 | 4.227 | 13.71× |
| 1,000 | 10 | 0.622 | 8.250 | 13.26× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
