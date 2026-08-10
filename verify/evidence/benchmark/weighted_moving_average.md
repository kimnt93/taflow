# WeightedMovingAverage benchmark (`WMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 142.85M | 0.007 | 143.77M | 0.040 | 5.70× | 5.74× |
| 10,000 | 0.044 | 228.14M | 0.041 | 244.95M | 0.057 | 1.30× | 1.39× |
| 100,000 | 0.377 | 265.18M | 0.394 | 254.01M | 0.266 | 0.71× | 0.68× |
| 1,000,000 | 4.509 | 221.76M | 3.897 | 256.58M | 2.327 | 0.52× | 0.60× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.097 | 0.156 | 1.61× |
| 1 | 5 | 0.318 | 0.546 | 1.72× |
| 1 | 10 | 0.549 | 0.997 | 1.82× |
| 10 | 1 | 0.049 | 0.094 | 1.91× |
| 10 | 5 | 0.229 | 0.463 | 2.03× |
| 10 | 10 | 0.572 | 1.017 | 1.78× |
| 100 | 1 | 0.050 | 0.097 | 1.94× |
| 100 | 5 | 0.227 | 0.457 | 2.01× |
| 100 | 10 | 0.523 | 1.186 | 2.27× |
| 1,000 | 1 | 0.402 | 0.107 | 0.27× |
| 1,000 | 5 | 0.275 | 0.472 | 1.72× |
| 1,000 | 10 | 0.523 | 1.157 | 2.21× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
