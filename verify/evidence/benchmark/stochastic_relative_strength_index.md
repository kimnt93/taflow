# StochasticRelativeStrengthIndex benchmark (`STOCHRSI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 49.23M | 0.019 | 53.38M | 0.051 | 2.49× | 2.70× |
| 10,000 | 0.183 | 54.61M | 0.177 | 56.38M | 0.187 | 1.02× | 1.05× |
| 100,000 | 2.753 | 36.33M | 2.336 | 42.81M | 1.793 | 0.65× | 0.77× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.069 | 0.119 | 1.72× |
| 1 | 5 | 0.224 | 0.530 | 2.37× |
| 1 | 10 | 0.389 | 1.056 | 2.71× |
| 10 | 1 | 0.045 | 0.107 | 2.41× |
| 10 | 5 | 0.186 | 0.501 | 2.69× |
| 10 | 10 | 0.398 | 1.049 | 2.64× |
| 100 | 1 | 0.045 | 0.103 | 2.30× |
| 100 | 5 | 0.204 | 0.498 | 2.44× |
| 100 | 10 | 0.428 | 1.053 | 2.46× |
| 1,000 | 1 | 0.069 | 0.123 | 1.79× |
| 1,000 | 5 | 0.205 | 0.592 | 2.89× |
| 1,000 | 10 | 0.423 | 1.207 | 2.86× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
