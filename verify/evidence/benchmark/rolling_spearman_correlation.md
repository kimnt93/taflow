# RollingSpearmanCorrelation benchmark (`SpearmanCorrelation` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.457 | 2.19M | 0.516 | 1.94M | 1.265 | 2.77× | 2.45× |
| 10,000 | 4.788 | 2.09M | 4.319 | 2.32M | 6.781 | 1.42× | 1.57× |
| 100,000 | 42.906 | 2.33M | 40.767 | 2.45M | 65.046 | 1.52× | 1.60× |
| 1,000,000 | 414.167 | 2.41M | 406.151 | 2.46M | 605.640 | 1.46× | 1.49× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.137 | 0.258 | 1.88× |
| 1 | 5 | 0.314 | 1.260 | 4.01× |
| 1 | 10 | 0.478 | 2.349 | 4.92× |
| 10 | 1 | 0.050 | 0.212 | 4.26× |
| 10 | 5 | 0.245 | 1.249 | 5.10× |
| 10 | 10 | 0.485 | 2.379 | 4.91× |
| 100 | 1 | 0.088 | 0.264 | 2.99× |
| 100 | 5 | 0.257 | 1.464 | 5.70× |
| 100 | 10 | 0.540 | 2.840 | 5.26× |
| 1,000 | 1 | 0.451 | 0.864 | 1.92× |
| 1,000 | 5 | 0.612 | 4.471 | 7.31× |
| 1,000 | 10 | 1.218 | 8.778 | 7.21× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
