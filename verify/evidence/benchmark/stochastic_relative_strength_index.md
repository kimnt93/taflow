# StochasticRelativeStrengthIndex benchmark (`STOCHRSI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.177 | 5.63M | 0.202 | 4.96M | 0.056 | 0.31× | 0.28× |
| 10,000 | 1.641 | 6.09M | 1.637 | 6.11M | 0.199 | 0.12× | 0.12× |
| 100,000 | 17.526 | 5.71M | 17.103 | 5.85M | 2.410 | 0.14× | 0.14× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.124 | 0.135 | 1.09× |
| 1 | 5 | 0.379 | 0.535 | 1.41× |
| 1 | 10 | 0.726 | 1.043 | 1.44× |
| 10 | 1 | 0.080 | 0.104 | 1.31× |
| 10 | 5 | 0.354 | 0.506 | 1.43× |
| 10 | 10 | 0.742 | 1.092 | 1.47× |
| 100 | 1 | 0.114 | 0.112 | 0.98× |
| 100 | 5 | 0.351 | 0.527 | 1.50× |
| 100 | 10 | 0.757 | 1.102 | 1.46× |
| 1,000 | 1 | 0.256 | 0.129 | 0.50× |
| 1,000 | 5 | 0.453 | 0.605 | 1.34× |
| 1,000 | 10 | 0.886 | 1.285 | 1.45× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
