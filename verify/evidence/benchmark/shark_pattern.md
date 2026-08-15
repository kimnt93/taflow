# SharkPattern benchmark (`Shark` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 93.74M | 0.008 | 131.03M | 0.234 | 21.97× | 30.71× |
| 10,000 | 0.097 | 102.81M | 0.091 | 109.33M | 1.387 | 14.26× | 15.17× |
| 100,000 | 0.953 | 104.92M | 0.928 | 107.81M | 13.222 | 13.87× | 14.25× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.073 | 0.213 | 2.93× |
| 1 | 5 | 0.314 | 0.833 | 2.65× |
| 1 | 10 | 0.406 | 1.637 | 4.03× |
| 10 | 1 | 0.052 | 0.161 | 3.10× |
| 10 | 5 | 0.223 | 1.096 | 4.92× |
| 10 | 10 | 0.385 | 1.618 | 4.20× |
| 100 | 1 | 0.045 | 0.179 | 3.98× |
| 100 | 5 | 0.232 | 1.186 | 5.11× |
| 100 | 10 | 0.396 | 1.779 | 4.49× |
| 1,000 | 1 | 0.059 | 0.312 | 5.28× |
| 1,000 | 5 | 0.211 | 1.756 | 8.34× |
| 1,000 | 10 | 0.410 | 3.017 | 7.37× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
