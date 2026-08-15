# HighLowIndex benchmark (`HighLowIndex` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 124.84M | 0.006 | 158.12M | 8.307 | 1037.03× | 1313.49× |
| 10,000 | 0.067 | 148.18M | 0.067 | 149.34M | 82.509 | 1222.61× | 1232.21× |
| 100,000 | 0.546 | 183.11M | 0.513 | 194.87M | 822.520 | 1506.13× | 1602.84× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.088 | 0.284 | 3.24× |
| 1 | 5 | 0.208 | 1.177 | 5.66× |
| 1 | 10 | 0.422 | 2.660 | 6.31× |
| 10 | 1 | 0.044 | 0.317 | 7.26× |
| 10 | 5 | 0.192 | 1.608 | 8.35× |
| 10 | 10 | 0.433 | 3.361 | 7.76× |
| 100 | 1 | 0.049 | 1.074 | 21.80× |
| 100 | 5 | 0.206 | 5.494 | 26.65× |
| 100 | 10 | 0.422 | 11.241 | 26.65× |
| 1,000 | 1 | 0.051 | 8.657 | 170.42× |
| 1,000 | 5 | 0.316 | 45.444 | 143.97× |
| 1,000 | 10 | 0.601 | 98.039 | 162.99× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
