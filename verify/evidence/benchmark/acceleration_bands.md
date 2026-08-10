# AccelerationBands benchmark (`ACCBANDS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 55.29M | 0.015 | 68.91M | 0.050 | 2.77× | 3.46× |
| 10,000 | 0.102 | 97.95M | 0.093 | 107.91M | 0.120 | 1.17× | 1.29× |
| 100,000 | 0.986 | 101.46M | 0.896 | 111.58M | 0.848 | 0.86× | 0.95× |
| 1,000,000 | 20.867 | 47.92M | 16.110 | 62.07M | 14.064 | 0.67× | 0.87× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.095 | 0.147 | 1.55× |
| 1 | 5 | 0.301 | 0.566 | 1.88× |
| 1 | 10 | 0.499 | 1.170 | 2.35× |
| 10 | 1 | 0.057 | 0.104 | 1.82× |
| 10 | 5 | 0.272 | 0.560 | 2.06× |
| 10 | 10 | 0.582 | 1.121 | 1.93× |
| 100 | 1 | 0.065 | 0.109 | 1.69× |
| 100 | 5 | 0.276 | 0.538 | 1.95× |
| 100 | 10 | 0.519 | 1.033 | 1.99× |
| 1,000 | 1 | 0.064 | 0.120 | 1.87× |
| 1,000 | 5 | 0.294 | 0.585 | 1.99× |
| 1,000 | 10 | 0.592 | 1.180 | 1.99× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
