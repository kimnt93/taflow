# GapUp benchmark (`gap up relation` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.046 | 21.55M | 0.040 | 24.73M | 0.023 | 0.48× | 0.56× |
| 10,000 | 0.337 | 29.68M | 0.345 | 28.98M | 0.044 | 0.13× | 0.13× |
| 100,000 | 3.242 | 30.85M | 3.153 | 31.72M | 0.219 | 0.07× | 0.07× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.128 | 0.097 | 0.76× |
| 1 | 5 | 0.472 | 0.365 | 0.77× |
| 1 | 10 | 0.583 | 0.706 | 1.21× |
| 10 | 1 | 0.063 | 0.072 | 1.15× |
| 10 | 5 | 0.297 | 0.353 | 1.19× |
| 10 | 10 | 0.569 | 0.745 | 1.31× |
| 100 | 1 | 0.067 | 0.076 | 1.13× |
| 100 | 5 | 0.312 | 0.346 | 1.11× |
| 100 | 10 | 0.592 | 0.754 | 1.27× |
| 1,000 | 1 | 0.099 | 0.081 | 0.83× |
| 1,000 | 5 | 0.300 | 0.486 | 1.62× |
| 1,000 | 10 | 0.636 | 1.156 | 1.82× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
