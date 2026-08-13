# RollingConditionalValueAtRisk benchmark (`ConditionalValueAtRisk` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 1.582 | 632.13K | 1.562 | 640.27K | 0.325 | 0.21× | 0.21× |
| 10,000 | 15.367 | 650.74K | 16.649 | 600.62K | 1.767 | 0.11× | 0.11× |
| 100,000 | 153.433 | 651.75K | 152.665 | 655.03K | 16.163 | 0.11× | 0.11× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.106 | 0.269 | 2.54× |
| 1 | 5 | 0.381 | 1.185 | 3.11× |
| 1 | 10 | 0.622 | 2.593 | 4.17× |
| 10 | 1 | 0.073 | 0.232 | 3.16× |
| 10 | 5 | 0.304 | 1.136 | 3.74× |
| 10 | 10 | 0.629 | 2.380 | 3.78× |
| 100 | 1 | 0.208 | 0.241 | 1.16× |
| 100 | 5 | 0.451 | 1.421 | 3.15× |
| 100 | 10 | 0.710 | 2.641 | 3.72× |
| 1,000 | 1 | 1.739 | 0.416 | 0.24× |
| 1,000 | 5 | 2.373 | 2.253 | 0.95× |
| 1,000 | 10 | 3.460 | 4.235 | 1.22× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
