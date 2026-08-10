# WeightedClose benchmark (`WCLPRICE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 150.13M | 0.005 | 195.75M | 0.027 | 4.11× | 5.36× |
| 10,000 | 0.020 | 496.80M | 0.018 | 569.39M | 0.033 | 1.62× | 1.86× |
| 100,000 | 0.174 | 573.59M | 0.136 | 736.38M | 0.076 | 0.43× | 0.56× |
| 1,000,000 | 2.230 | 448.43M | 1.801 | 555.10M | 1.513 | 0.68× | 0.84× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.130 | 0.099 | 0.76× |
| 1 | 5 | 0.381 | 0.475 | 1.25× |
| 1 | 10 | 0.491 | 0.878 | 1.79× |
| 10 | 1 | 0.047 | 0.094 | 1.99× |
| 10 | 5 | 0.229 | 0.419 | 1.83× |
| 10 | 10 | 0.466 | 0.898 | 1.93× |
| 100 | 1 | 0.053 | 0.086 | 1.60× |
| 100 | 5 | 0.242 | 0.416 | 1.72× |
| 100 | 10 | 0.484 | 0.899 | 1.86× |
| 1,000 | 1 | 0.051 | 0.091 | 1.78× |
| 1,000 | 5 | 0.223 | 0.424 | 1.90× |
| 1,000 | 10 | 0.481 | 0.892 | 1.85× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
