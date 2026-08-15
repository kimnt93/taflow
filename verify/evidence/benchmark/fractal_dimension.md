# FractalDimension benchmark (`two-chunk rescaled-range dimension` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.197 | 5.07M | 0.200 | 4.99M | 0.809 | 4.10× | 4.04× |
| 10,000 | 2.037 | 4.91M | 1.930 | 5.18M | 6.062 | 2.98× | 3.14× |
| 100,000 | 20.673 | 4.84M | 20.019 | 5.00M | 66.074 | 3.20× | 3.30× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.128 | 0.138 | 1.08× |
| 1 | 5 | 0.264 | 0.439 | 1.66× |
| 1 | 10 | 0.388 | 1.128 | 2.91× |
| 10 | 1 | 0.054 | 0.095 | 1.76× |
| 10 | 5 | 0.188 | 0.407 | 2.17× |
| 10 | 10 | 0.403 | 0.858 | 2.13× |
| 100 | 1 | 0.064 | 0.383 | 6.03× |
| 100 | 5 | 0.238 | 2.124 | 8.93× |
| 100 | 10 | 0.457 | 4.336 | 9.49× |
| 1,000 | 1 | 0.253 | 0.890 | 3.52× |
| 1,000 | 5 | 0.408 | 2.954 | 7.24× |
| 1,000 | 10 | 0.630 | 6.619 | 10.50× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
