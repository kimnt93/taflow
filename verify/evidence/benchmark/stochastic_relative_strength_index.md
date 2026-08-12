# StochasticRelativeStrengthIndex benchmark (`STOCHRSI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.024 | 42.29M | 0.022 | 45.03M | 0.055 | 2.31× | 2.46× |
| 10,000 | 0.229 | 43.58M | 0.231 | 43.38M | 0.203 | 0.89× | 0.88× |
| 100,000 | 2.978 | 33.58M | 2.955 | 33.84M | 1.694 | 0.57× | 0.57× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.083 | 0.134 | 1.61× |
| 1 | 5 | 0.322 | 0.608 | 1.89× |
| 1 | 10 | 0.564 | 1.100 | 1.95× |
| 10 | 1 | 0.051 | 0.100 | 1.98× |
| 10 | 5 | 0.239 | 0.542 | 2.27× |
| 10 | 10 | 0.506 | 1.132 | 2.24× |
| 100 | 1 | 0.061 | 0.111 | 1.80× |
| 100 | 5 | 0.259 | 0.558 | 2.15× |
| 100 | 10 | 0.562 | 1.183 | 2.10× |
| 1,000 | 1 | 0.082 | 0.121 | 1.47× |
| 1,000 | 5 | 0.251 | 0.627 | 2.49× |
| 1,000 | 10 | 0.506 | 1.292 | 2.56× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
