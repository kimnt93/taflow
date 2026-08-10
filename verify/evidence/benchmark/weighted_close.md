# WeightedClose benchmark (`WCLPRICE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 141.43M | 0.005 | 183.86M | 0.030 | 4.26× | 5.53× |
| 10,000 | 0.023 | 425.76M | 0.020 | 508.56M | 0.037 | 1.58× | 1.89× |
| 100,000 | 0.191 | 524.72M | 0.173 | 578.86M | 0.095 | 0.50× | 0.55× |
| 1,000,000 | 3.429 | 291.60M | 2.133 | 468.92M | 1.451 | 0.42× | 0.68× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.078 | 0.143 | 1.83× |
| 1 | 5 | 0.324 | 0.497 | 1.54× |
| 1 | 10 | 0.520 | 0.896 | 1.72× |
| 10 | 1 | 0.052 | 0.088 | 1.70× |
| 10 | 5 | 0.220 | 0.436 | 1.98× |
| 10 | 10 | 0.532 | 0.927 | 1.74× |
| 100 | 1 | 0.051 | 0.094 | 1.86× |
| 100 | 5 | 0.235 | 0.416 | 1.77× |
| 100 | 10 | 0.493 | 0.936 | 1.90× |
| 1,000 | 1 | 0.059 | 0.093 | 1.56× |
| 1,000 | 5 | 0.230 | 0.425 | 1.85× |
| 1,000 | 10 | 0.485 | 0.904 | 1.86× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
