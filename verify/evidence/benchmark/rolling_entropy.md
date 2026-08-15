# RollingEntropy benchmark (`rolling Shannon entropy` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.711 | 1.41M | 0.714 | 1.40M | 0.051 | 0.07× | 0.07× |
| 10,000 | 7.425 | 1.35M | 7.381 | 1.35M | 0.122 | 0.02× | 0.02× |
| 100,000 | 73.948 | 1.35M | 73.601 | 1.36M | 1.096 | 0.01× | 0.01× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.120 | 0.115 | 0.96× |
| 1 | 5 | 0.252 | 0.431 | 1.71× |
| 1 | 10 | 0.411 | 0.858 | 2.09× |
| 10 | 1 | 0.043 | 0.098 | 2.31× |
| 10 | 5 | 0.189 | 0.420 | 2.22× |
| 10 | 10 | 0.436 | 0.861 | 1.97× |
| 100 | 1 | 0.115 | 0.123 | 1.07× |
| 100 | 5 | 0.223 | 0.566 | 2.54× |
| 100 | 10 | 0.455 | 1.149 | 2.52× |
| 1,000 | 1 | 0.858 | 0.126 | 0.15× |
| 1,000 | 5 | 0.964 | 0.703 | 0.73× |
| 1,000 | 10 | 1.526 | 1.440 | 0.94× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
