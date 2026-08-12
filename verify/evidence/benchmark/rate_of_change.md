# RateOfChange benchmark (`ROC` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 160.47M | 0.005 | 190.49M | 0.032 | 5.14× | 6.10× |
| 10,000 | 0.022 | 459.98M | 0.018 | 543.45M | 0.040 | 1.83× | 2.17× |
| 100,000 | 0.177 | 566.27M | 0.159 | 629.92M | 0.131 | 0.74× | 0.82× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.136 | 0.108 | 0.79× |
| 1 | 5 | 0.414 | 0.494 | 1.19× |
| 1 | 10 | 0.482 | 0.934 | 1.94× |
| 10 | 1 | 0.047 | 0.094 | 2.02× |
| 10 | 5 | 0.217 | 0.433 | 1.99× |
| 10 | 10 | 0.470 | 0.988 | 2.10× |
| 100 | 1 | 0.062 | 0.088 | 1.42× |
| 100 | 5 | 0.229 | 0.426 | 1.86× |
| 100 | 10 | 0.465 | 0.926 | 1.99× |
| 1,000 | 1 | 0.054 | 0.098 | 1.82× |
| 1,000 | 5 | 0.256 | 0.455 | 1.78× |
| 1,000 | 10 | 0.462 | 0.921 | 1.99× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
