# CloseToCloseSigma benchmark (`annualized close-to-close volatility` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.022 | 45.08M | 0.021 | 47.25M | 0.140 | 6.31× | 6.61× |
| 10,000 | 0.203 | 49.38M | 0.193 | 51.83M | 0.769 | 3.80× | 3.99× |
| 100,000 | 1.938 | 51.60M | 1.853 | 53.97M | 6.734 | 3.47× | 3.63× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.100 | 0.148 | 1.49× |
| 1 | 5 | 0.343 | 0.790 | 2.30× |
| 1 | 10 | 0.476 | 1.154 | 2.43× |
| 10 | 1 | 0.050 | 0.118 | 2.33× |
| 10 | 5 | 0.252 | 0.584 | 2.32× |
| 10 | 10 | 0.504 | 1.194 | 2.37× |
| 100 | 1 | 0.057 | 0.165 | 2.90× |
| 100 | 5 | 0.252 | 0.843 | 3.35× |
| 100 | 10 | 0.539 | 1.719 | 3.19× |
| 1,000 | 1 | 0.070 | 0.228 | 3.27× |
| 1,000 | 5 | 0.252 | 1.050 | 4.16× |
| 1,000 | 10 | 0.495 | 2.227 | 4.50× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
