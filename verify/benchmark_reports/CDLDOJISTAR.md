# CandleDojiStar benchmark (`CDLDOJISTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 99.78M | 0.008 | 129.09M | 0.034 | 3.42× | 4.43× |
| 10,000 | 0.104 | 96.32M | 0.102 | 98.33M | 0.143 | 1.38× | 1.41× |
| 100,000 | 1.121 | 89.21M | 1.144 | 87.45M | 1.028 | 0.92× | 0.90× |
| 1,000,000 | 11.250 | 88.89M | 11.091 | 90.16M | 10.617 | 0.94× | 0.96× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.115 | 0.112 | 0.97× |
| 1 | 5 | 0.349 | 0.505 | 1.45× |
| 1 | 10 | 0.496 | 0.923 | 1.86× |
| 10 | 1 | 0.052 | 0.092 | 1.75× |
| 10 | 5 | 0.239 | 0.418 | 1.75× |
| 10 | 10 | 0.500 | 0.940 | 1.88× |
| 100 | 1 | 0.053 | 0.089 | 1.67× |
| 100 | 5 | 0.259 | 0.451 | 1.75× |
| 100 | 10 | 0.527 | 0.921 | 1.75× |
| 1,000 | 1 | 0.065 | 0.103 | 1.58× |
| 1,000 | 5 | 0.275 | 0.506 | 1.84× |
| 1,000 | 10 | 0.544 | 1.040 | 1.91× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
