# ArnaudLegouxMovingAverage benchmark (`ALMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 53.66M | 0.017 | 59.65M | 0.247 | 13.25× | 14.73× |
| 10,000 | 0.143 | 69.84M | 0.142 | 70.34M | 0.604 | 4.22× | 4.25× |
| 100,000 | 1.381 | 72.40M | 1.350 | 74.05M | 4.289 | 3.11× | 3.18× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.123 | 0.285 | 2.32× |
| 1 | 5 | 0.301 | 1.498 | 4.97× |
| 1 | 10 | 0.473 | 3.040 | 6.42× |
| 10 | 1 | 0.059 | 0.279 | 4.76× |
| 10 | 5 | 0.229 | 1.477 | 6.44× |
| 10 | 10 | 0.493 | 3.088 | 6.27× |
| 100 | 1 | 0.058 | 0.276 | 4.77× |
| 100 | 5 | 0.250 | 1.539 | 6.17× |
| 100 | 10 | 0.500 | 3.240 | 6.48× |
| 1,000 | 1 | 0.072 | 0.319 | 4.41× |
| 1,000 | 5 | 0.251 | 1.693 | 6.75× |
| 1,000 | 10 | 0.550 | 3.552 | 6.46× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
