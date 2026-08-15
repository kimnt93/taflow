# TripleExponentialRateOfChange benchmark (`TRIX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 173.27M | 0.005 | 202.87M | 0.041 | 7.06× | 8.27× |
| 10,000 | 0.038 | 262.01M | 0.035 | 282.36M | 0.127 | 3.32× | 3.58× |
| 100,000 | 0.367 | 272.31M | 0.339 | 294.73M | 0.943 | 2.57× | 2.78× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.161 | 0.214 | 1.32× |
| 1 | 5 | 0.226 | 0.524 | 2.32× |
| 1 | 10 | 0.421 | 1.035 | 2.46× |
| 10 | 1 | 0.042 | 0.094 | 2.26× |
| 10 | 5 | 0.195 | 0.477 | 2.45× |
| 10 | 10 | 0.425 | 0.983 | 2.31× |
| 100 | 1 | 0.041 | 0.093 | 2.28× |
| 100 | 5 | 0.194 | 0.483 | 2.49× |
| 100 | 10 | 0.449 | 1.035 | 2.31× |
| 1,000 | 1 | 0.049 | 0.102 | 2.07× |
| 1,000 | 5 | 0.214 | 0.549 | 2.57× |
| 1,000 | 10 | 0.446 | 1.115 | 2.50× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
