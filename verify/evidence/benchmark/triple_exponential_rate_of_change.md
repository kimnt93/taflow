# TripleExponentialRateOfChange benchmark (`TRIX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.038 | 26.37M | 0.032 | 30.88M | 0.040 | 1.05× | 1.23× |
| 10,000 | 0.239 | 41.83M | 0.227 | 43.98M | 0.127 | 0.53× | 0.56× |
| 100,000 | 2.194 | 45.57M | 2.257 | 44.30M | 0.945 | 0.43× | 0.42× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.100 | 0.133 | 1.33× |
| 1 | 5 | 0.444 | 0.510 | 1.15× |
| 1 | 10 | 0.653 | 1.003 | 1.54× |
| 10 | 1 | 0.066 | 0.095 | 1.45× |
| 10 | 5 | 0.314 | 0.451 | 1.43× |
| 10 | 10 | 0.620 | 0.922 | 1.49× |
| 100 | 1 | 0.068 | 0.091 | 1.33× |
| 100 | 5 | 0.305 | 0.446 | 1.46× |
| 100 | 10 | 0.671 | 0.951 | 1.42× |
| 1,000 | 1 | 0.099 | 0.098 | 0.99× |
| 1,000 | 5 | 0.307 | 0.501 | 1.63× |
| 1,000 | 10 | 0.654 | 1.076 | 1.65× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
