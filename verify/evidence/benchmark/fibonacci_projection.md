# FibonacciProjection benchmark (`FibProjection` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 56.91M | 0.015 | 64.57M | 0.529 | 30.11× | 34.16× |
| 10,000 | 0.140 | 71.52M | 0.128 | 78.20M | 4.206 | 30.08× | 32.89× |
| 100,000 | 1.337 | 74.78M | 1.244 | 80.42M | 43.643 | 32.64× | 35.10× |
| 1,000,000 | 16.452 | 60.78M | 13.037 | 76.71M | 488.566 | 29.70× | 37.48× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.083 | 0.231 | 2.78× |
| 1 | 5 | 0.322 | 0.875 | 2.72× |
| 1 | 10 | 0.477 | 1.829 | 3.84× |
| 10 | 1 | 0.051 | 0.177 | 3.45× |
| 10 | 5 | 0.226 | 0.838 | 3.72× |
| 10 | 10 | 0.491 | 1.947 | 3.97× |
| 100 | 1 | 0.058 | 0.223 | 3.86× |
| 100 | 5 | 0.246 | 1.060 | 4.30× |
| 100 | 10 | 0.453 | 2.261 | 5.00× |
| 1,000 | 1 | 0.070 | 0.697 | 9.99× |
| 1,000 | 5 | 0.250 | 3.344 | 13.36× |
| 1,000 | 10 | 0.515 | 6.635 | 12.88× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
