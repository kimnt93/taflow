# WeightedMovingAverage benchmark (`WMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 126.55M | 0.007 | 134.12M | 0.039 | 4.94× | 5.24× |
| 10,000 | 0.043 | 231.27M | 0.038 | 259.97M | 0.055 | 1.28× | 1.43× |
| 100,000 | 0.386 | 259.33M | 0.345 | 289.76M | 0.234 | 0.61× | 0.68× |
| 1,000,000 | 4.062 | 246.20M | 3.486 | 286.85M | 2.117 | 0.52× | 0.61× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.130 | 0.120 | 0.93× |
| 1 | 5 | 0.286 | 0.502 | 1.76× |
| 1 | 10 | 0.494 | 1.049 | 2.12× |
| 10 | 1 | 0.064 | 0.112 | 1.76× |
| 10 | 5 | 0.251 | 0.470 | 1.87× |
| 10 | 10 | 0.501 | 1.002 | 2.00× |
| 100 | 1 | 0.051 | 0.098 | 1.90× |
| 100 | 5 | 0.263 | 0.543 | 2.07× |
| 100 | 10 | 0.503 | 1.022 | 2.03× |
| 1,000 | 1 | 0.058 | 0.095 | 1.64× |
| 1,000 | 5 | 0.255 | 0.519 | 2.04× |
| 1,000 | 10 | 0.568 | 1.032 | 1.82× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
