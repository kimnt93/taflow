# CandleStickSandwich benchmark (`CDLSTICKSANDWICH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 150.38M | 0.003 | 298.97M | 0.034 | 5.17× | 10.28× |
| 10,000 | 0.044 | 227.01M | 0.039 | 253.93M | 0.090 | 2.04× | 2.29× |
| 100,000 | 0.563 | 177.52M | 0.574 | 174.31M | 0.611 | 1.09× | 1.07× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.153 | 0.167 | 1.09× |
| 1 | 5 | 0.218 | 0.454 | 2.09× |
| 1 | 10 | 0.386 | 0.922 | 2.39× |
| 10 | 1 | 0.044 | 0.087 | 2.01× |
| 10 | 5 | 0.208 | 0.484 | 2.32× |
| 10 | 10 | 0.403 | 0.907 | 2.25× |
| 100 | 1 | 0.043 | 0.092 | 2.16× |
| 100 | 5 | 0.183 | 0.422 | 2.30× |
| 100 | 10 | 0.419 | 0.947 | 2.26× |
| 1,000 | 1 | 0.052 | 0.093 | 1.79× |
| 1,000 | 5 | 0.185 | 0.458 | 2.48× |
| 1,000 | 10 | 0.430 | 0.991 | 2.30× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
