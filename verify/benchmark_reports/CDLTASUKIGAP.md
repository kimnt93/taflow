# CandleTasukiGap benchmark (`CDLTASUKIGAP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 64.83M | 0.014 | 73.56M | 0.045 | 2.90× | 3.30× |
| 10,000 | 0.088 | 113.92M | 0.088 | 113.91M | 0.185 | 2.11× | 2.11× |
| 100,000 | 0.816 | 122.54M | 0.814 | 122.78M | 1.472 | 1.80× | 1.81× |
| 1,000,000 | 9.277 | 107.80M | 8.448 | 118.37M | 14.597 | 1.57× | 1.73× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.148 | 0.130 | 0.88× |
| 1 | 5 | 0.291 | 0.521 | 1.79× |
| 1 | 10 | 0.510 | 0.939 | 1.84× |
| 10 | 1 | 0.055 | 0.093 | 1.70× |
| 10 | 5 | 0.241 | 0.444 | 1.84× |
| 10 | 10 | 0.526 | 0.929 | 1.77× |
| 100 | 1 | 0.060 | 0.093 | 1.55× |
| 100 | 5 | 0.264 | 0.450 | 1.70× |
| 100 | 10 | 0.536 | 0.955 | 1.78× |
| 1,000 | 1 | 0.060 | 0.106 | 1.76× |
| 1,000 | 5 | 0.277 | 0.547 | 1.97× |
| 1,000 | 10 | 0.549 | 1.155 | 2.10× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
