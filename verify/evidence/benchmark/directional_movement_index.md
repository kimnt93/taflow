# DirectionalMovementIndex benchmark (`DX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 53.62M | 0.016 | 61.82M | 0.043 | 2.33× | 2.69× |
| 10,000 | 0.114 | 87.44M | 0.108 | 92.78M | 0.124 | 1.09× | 1.15× |
| 100,000 | 1.094 | 91.39M | 1.055 | 94.81M | 0.919 | 0.84× | 0.87× |
| 1,000,000 | 11.355 | 88.07M | 10.687 | 93.58M | 9.205 | 0.81× | 0.86× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.070 | 0.117 | 1.67× |
| 1 | 5 | 0.335 | 0.516 | 1.54× |
| 1 | 10 | 0.615 | 1.089 | 1.77× |
| 10 | 1 | 0.055 | 0.103 | 1.86× |
| 10 | 5 | 0.242 | 0.468 | 1.93× |
| 10 | 10 | 0.535 | 0.986 | 1.84× |
| 100 | 1 | 0.056 | 0.103 | 1.84× |
| 100 | 5 | 0.267 | 0.503 | 1.88× |
| 100 | 10 | 0.541 | 0.951 | 1.76× |
| 1,000 | 1 | 0.067 | 0.102 | 1.51× |
| 1,000 | 5 | 0.264 | 0.526 | 1.99× |
| 1,000 | 10 | 0.584 | 1.067 | 1.83× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
