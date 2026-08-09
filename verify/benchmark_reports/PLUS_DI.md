# PlusDirectionalIndicator benchmark (`PLUS_DI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 88.91M | 0.010 | 104.04M | 0.039 | 3.46× | 4.05× |
| 10,000 | 0.067 | 148.23M | 0.064 | 156.53M | 0.101 | 1.50× | 1.59× |
| 100,000 | 0.607 | 164.88M | 0.577 | 173.16M | 0.704 | 1.16× | 1.22× |
| 1,000,000 | 6.653 | 150.32M | 6.238 | 160.30M | 7.166 | 1.08× | 1.15× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.103 | 0.149 | 1.45× |
| 1 | 5 | 0.304 | 0.533 | 1.75× |
| 1 | 10 | 0.480 | 0.946 | 1.97× |
| 10 | 1 | 0.057 | 0.097 | 1.71× |
| 10 | 5 | 0.268 | 0.491 | 1.83× |
| 10 | 10 | 0.497 | 0.985 | 1.98× |
| 100 | 1 | 0.052 | 0.095 | 1.81× |
| 100 | 5 | 0.246 | 0.495 | 2.01× |
| 100 | 10 | 0.545 | 1.000 | 1.83× |
| 1,000 | 1 | 0.058 | 0.106 | 1.82× |
| 1,000 | 5 | 0.243 | 0.490 | 2.02× |
| 1,000 | 10 | 0.565 | 1.125 | 1.99× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
