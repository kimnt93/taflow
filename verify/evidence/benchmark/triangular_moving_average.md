# TriangularMovingAverage benchmark (`TRIMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 158.23M | 0.005 | 192.12M | 0.035 | 5.57× | 6.77× |
| 10,000 | 0.043 | 232.07M | 0.041 | 243.79M | 0.061 | 1.43× | 1.50× |
| 100,000 | 0.412 | 242.67M | 0.391 | 255.49M | 0.317 | 0.77× | 0.81× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.078 | 0.139 | 1.78× |
| 1 | 5 | 0.219 | 0.473 | 2.16× |
| 1 | 10 | 0.399 | 0.933 | 2.34× |
| 10 | 1 | 0.050 | 0.101 | 2.01× |
| 10 | 5 | 0.215 | 0.467 | 2.18× |
| 10 | 10 | 0.389 | 0.964 | 2.48× |
| 100 | 1 | 0.042 | 0.096 | 2.28× |
| 100 | 5 | 0.195 | 0.467 | 2.40× |
| 100 | 10 | 0.445 | 0.944 | 2.12× |
| 1,000 | 1 | 0.054 | 0.094 | 1.75× |
| 1,000 | 5 | 0.201 | 0.457 | 2.27× |
| 1,000 | 10 | 0.438 | 1.087 | 2.48× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
