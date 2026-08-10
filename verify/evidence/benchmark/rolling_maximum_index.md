# RollingMaximumIndex benchmark (`MAXINDEX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 111.17M | 0.008 | 129.41M | 0.040 | 4.45× | 5.18× |
| 10,000 | 0.071 | 140.96M | 0.060 | 166.03M | 0.114 | 1.61× | 1.89× |
| 100,000 | 0.593 | 168.50M | 0.639 | 156.50M | 0.839 | 1.41× | 1.31× |
| 1,000,000 | 7.134 | 140.18M | 5.797 | 172.50M | 7.393 | 1.04× | 1.28× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.101 | 0.118 | 1.17× |
| 1 | 5 | 0.316 | 0.639 | 2.02× |
| 1 | 10 | 0.553 | 1.105 | 2.00× |
| 10 | 1 | 0.053 | 0.089 | 1.66× |
| 10 | 5 | 0.257 | 0.604 | 2.35× |
| 10 | 10 | 0.543 | 1.021 | 1.88× |
| 100 | 1 | 0.051 | 0.097 | 1.91× |
| 100 | 5 | 0.237 | 0.523 | 2.21× |
| 100 | 10 | 0.628 | 1.091 | 1.74× |
| 1,000 | 1 | 0.055 | 0.101 | 1.85× |
| 1,000 | 5 | 0.310 | 0.530 | 1.71× |
| 1,000 | 10 | 0.569 | 1.144 | 2.01× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
