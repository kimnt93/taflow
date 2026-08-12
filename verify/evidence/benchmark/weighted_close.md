# WeightedClose benchmark (`WCLPRICE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 157.78M | 0.005 | 199.87M | 0.027 | 4.31× | 5.46× |
| 10,000 | 0.022 | 455.87M | 0.019 | 534.76M | 0.033 | 1.51× | 1.77× |
| 100,000 | 0.157 | 637.66M | 0.132 | 756.68M | 0.082 | 0.52× | 0.62× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.087 | 0.099 | 1.13× |
| 1 | 5 | 0.372 | 0.480 | 1.29× |
| 1 | 10 | 0.502 | 0.931 | 1.85× |
| 10 | 1 | 0.053 | 0.089 | 1.68× |
| 10 | 5 | 0.239 | 0.425 | 1.78× |
| 10 | 10 | 0.496 | 0.952 | 1.92× |
| 100 | 1 | 0.049 | 0.090 | 1.86× |
| 100 | 5 | 0.251 | 0.440 | 1.75× |
| 100 | 10 | 0.515 | 0.938 | 1.82× |
| 1,000 | 1 | 0.059 | 0.090 | 1.53× |
| 1,000 | 5 | 0.249 | 0.452 | 1.82× |
| 1,000 | 10 | 0.515 | 0.938 | 1.82× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
