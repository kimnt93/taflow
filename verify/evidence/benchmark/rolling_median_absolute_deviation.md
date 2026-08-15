# RollingMedianAbsoluteDeviation benchmark (`MedianAbsoluteDeviation` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.326 | 3.07M | 0.303 | 3.30M | 0.500 | 1.54× | 1.65× |
| 10,000 | 3.232 | 3.09M | 3.160 | 3.16M | 3.472 | 1.07× | 1.10× |
| 100,000 | 32.058 | 3.12M | 31.947 | 3.13M | 34.210 | 1.07× | 1.07× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.087 | 0.306 | 3.50× |
| 1 | 5 | 0.292 | 1.244 | 4.27× |
| 1 | 10 | 0.374 | 2.337 | 6.25× |
| 10 | 1 | 0.044 | 0.215 | 4.92× |
| 10 | 5 | 0.205 | 1.319 | 6.43× |
| 10 | 10 | 0.400 | 2.265 | 5.66× |
| 100 | 1 | 0.086 | 0.259 | 3.02× |
| 100 | 5 | 0.211 | 1.376 | 6.53× |
| 100 | 10 | 0.513 | 2.733 | 5.33× |
| 1,000 | 1 | 0.380 | 0.567 | 1.49× |
| 1,000 | 5 | 0.644 | 3.198 | 4.96× |
| 1,000 | 10 | 1.161 | 6.006 | 5.17× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
