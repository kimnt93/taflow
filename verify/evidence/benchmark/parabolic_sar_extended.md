# ParabolicSarExtended benchmark (`SAREXT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 66.12M | 0.014 | 70.61M | 0.056 | 3.70× | 3.95× |
| 10,000 | 0.122 | 81.73M | 0.126 | 79.39M | 0.108 | 0.88× | 0.85× |
| 100,000 | 1.342 | 74.54M | 1.189 | 84.09M | 0.728 | 0.54× | 0.61× |
| 1,000,000 | 12.444 | 80.36M | 11.518 | 86.82M | 6.083 | 0.49× | 0.53× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.125 | 0.156 | 1.25× |
| 1 | 5 | 0.328 | 0.609 | 1.86× |
| 1 | 10 | 0.484 | 1.104 | 2.28× |
| 10 | 1 | 0.050 | 0.109 | 2.20× |
| 10 | 5 | 0.242 | 0.546 | 2.26× |
| 10 | 10 | 0.515 | 1.158 | 2.25× |
| 100 | 1 | 0.052 | 0.116 | 2.23× |
| 100 | 5 | 0.220 | 0.552 | 2.51× |
| 100 | 10 | 0.493 | 1.294 | 2.62× |
| 1,000 | 1 | 0.065 | 0.120 | 1.87× |
| 1,000 | 5 | 0.259 | 0.582 | 2.25× |
| 1,000 | 10 | 0.532 | 1.249 | 2.35× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
