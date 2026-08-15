# StochasticOscillator benchmark (`STOCH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 71.50M | 0.012 | 83.82M | 0.054 | 3.86× | 4.53× |
| 10,000 | 0.108 | 92.40M | 0.103 | 97.28M | 0.163 | 1.51× | 1.59× |
| 100,000 | 1.680 | 59.51M | 1.643 | 60.88M | 1.198 | 0.71× | 0.73× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.073 | 0.196 | 2.67× |
| 1 | 5 | 0.232 | 0.540 | 2.33× |
| 1 | 10 | 0.427 | 1.095 | 2.57× |
| 10 | 1 | 0.050 | 0.102 | 2.06× |
| 10 | 5 | 0.215 | 0.563 | 2.62× |
| 10 | 10 | 0.488 | 1.159 | 2.38× |
| 100 | 1 | 0.054 | 0.103 | 1.93× |
| 100 | 5 | 0.214 | 0.575 | 2.69× |
| 100 | 10 | 0.543 | 1.175 | 2.16× |
| 1,000 | 1 | 0.063 | 0.124 | 1.95× |
| 1,000 | 5 | 0.218 | 0.618 | 2.84× |
| 1,000 | 10 | 0.503 | 1.300 | 2.58× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
