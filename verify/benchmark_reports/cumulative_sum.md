# CumulativeSum benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 178.94M | 0.004 | 227.26M | 0.050 | 8.86× | 11.25× |
| 10,000 | 0.028 | 359.28M | 0.025 | 406.94M | 0.087 | 3.13× | 3.54× |
| 100,000 | 0.288 | 347.16M | 0.222 | 451.40M | 0.451 | 1.56× | 2.03× |
| 1,000,000 | 2.791 | 358.27M | 2.363 | 423.19M | 4.047 | 1.45× | 1.71× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.121 | 0.270 | 2.23× |
| 1 | 5 | 0.314 | 0.719 | 2.29× |
| 1 | 10 | 0.493 | 1.613 | 3.28× |
| 10 | 1 | 0.053 | 0.150 | 2.86× |
| 10 | 5 | 0.231 | 0.601 | 2.61× |
| 10 | 10 | 0.471 | 1.193 | 2.53× |
| 100 | 1 | 0.052 | 0.154 | 2.98× |
| 100 | 5 | 0.227 | 0.574 | 2.53× |
| 100 | 10 | 0.479 | 1.202 | 2.51× |
| 1,000 | 1 | 0.053 | 0.171 | 3.23× |
| 1,000 | 5 | 0.237 | 0.603 | 2.54× |
| 1,000 | 10 | 0.574 | 1.435 | 2.50× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
