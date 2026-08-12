# FibonacciProjection benchmark (`FibProjection` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 53.25M | 0.016 | 62.99M | 0.535 | 28.48× | 33.69× |
| 10,000 | 0.142 | 70.44M | 0.130 | 76.82M | 4.429 | 31.19× | 34.02× |
| 100,000 | 1.476 | 67.74M | 1.324 | 75.53M | 47.198 | 31.97× | 35.65× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.079 | 0.205 | 2.58× |
| 1 | 5 | 0.328 | 0.845 | 2.57× |
| 1 | 10 | 0.519 | 1.943 | 3.75× |
| 10 | 1 | 0.052 | 0.179 | 3.44× |
| 10 | 5 | 0.305 | 0.891 | 2.92× |
| 10 | 10 | 0.501 | 1.982 | 3.96× |
| 100 | 1 | 0.064 | 0.216 | 3.36× |
| 100 | 5 | 0.246 | 1.058 | 4.30× |
| 100 | 10 | 0.515 | 2.400 | 4.66× |
| 1,000 | 1 | 0.068 | 0.786 | 11.59× |
| 1,000 | 5 | 0.245 | 10.351 | 42.18× |
| 1,000 | 10 | 0.566 | 6.898 | 12.19× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
