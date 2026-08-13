# AutomaticFibonacci benchmark (`AutoFib` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.204 | 4.90M | 0.184 | 5.45M | 0.658 | 3.22× | 3.58× |
| 10,000 | 1.828 | 5.47M | 1.777 | 5.63M | 5.405 | 2.96× | 3.04× |
| 100,000 | 19.096 | 5.24M | 17.961 | 5.57M | 62.191 | 3.26× | 3.46× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.091 | 0.215 | 2.37× |
| 1 | 5 | 0.364 | 0.846 | 2.33× |
| 1 | 10 | 0.649 | 1.855 | 2.86× |
| 10 | 1 | 0.073 | 0.172 | 2.35× |
| 10 | 5 | 0.320 | 0.843 | 2.63× |
| 10 | 10 | 0.690 | 1.941 | 2.81× |
| 100 | 1 | 0.091 | 0.231 | 2.54× |
| 100 | 5 | 0.327 | 1.136 | 3.48× |
| 100 | 10 | 0.646 | 2.454 | 3.80× |
| 1,000 | 1 | 0.272 | 0.972 | 3.57× |
| 1,000 | 5 | 0.491 | 4.150 | 8.45× |
| 1,000 | 10 | 0.823 | 8.331 | 10.12× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
