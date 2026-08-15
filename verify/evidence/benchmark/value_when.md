# ValueWhen benchmark (`last value when condition` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 235.32M | 0.003 | 314.66M | 0.146 | 34.27× | 45.82× |
| 10,000 | 0.024 | 417.04M | 0.020 | 500.78M | 1.369 | 57.07× | 68.53× |
| 100,000 | 0.280 | 356.58M | 0.190 | 527.57M | 13.970 | 49.81× | 73.70× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.126 | 0.107 | 0.85× |
| 1 | 5 | 0.211 | 0.352 | 1.67× |
| 1 | 10 | 0.419 | 0.658 | 1.57× |
| 10 | 1 | 0.044 | 0.066 | 1.48× |
| 10 | 5 | 0.196 | 0.320 | 1.63× |
| 10 | 10 | 0.399 | 0.717 | 1.80× |
| 100 | 1 | 0.049 | 0.081 | 1.64× |
| 100 | 5 | 0.184 | 0.389 | 2.12× |
| 100 | 10 | 0.396 | 0.777 | 1.96× |
| 1,000 | 1 | 0.046 | 0.208 | 4.56× |
| 1,000 | 5 | 0.187 | 1.143 | 6.12× |
| 1,000 | 10 | 0.427 | 2.095 | 4.91× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
