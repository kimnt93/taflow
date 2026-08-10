# RateOfChangePercent benchmark (`ROCP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 226.93M | 0.004 | 246.64M | 0.032 | 7.17× | 7.79× |
| 10,000 | 0.022 | 460.29M | 0.022 | 460.06M | 0.039 | 1.80× | 1.79× |
| 100,000 | 0.175 | 570.63M | 0.159 | 629.28M | 0.119 | 0.68× | 0.75× |
| 1,000,000 | 2.069 | 483.26M | 1.808 | 553.13M | 1.045 | 0.50× | 0.58× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.106 | 0.150 | 1.41× |
| 1 | 5 | 0.398 | 0.481 | 1.21× |
| 1 | 10 | 0.466 | 0.905 | 1.94× |
| 10 | 1 | 0.046 | 0.089 | 1.93× |
| 10 | 5 | 0.215 | 0.427 | 1.99× |
| 10 | 10 | 0.444 | 0.953 | 2.14× |
| 100 | 1 | 0.047 | 0.093 | 2.00× |
| 100 | 5 | 0.221 | 0.426 | 1.93× |
| 100 | 10 | 0.464 | 0.955 | 2.06× |
| 1,000 | 1 | 0.070 | 0.100 | 1.42× |
| 1,000 | 5 | 0.236 | 0.471 | 2.00× |
| 1,000 | 10 | 0.484 | 0.952 | 1.97× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
