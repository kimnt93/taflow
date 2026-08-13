# RollingRecoveryFactor benchmark (`rolling recovery factor on equity` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.314 | 3.18M | 0.332 | 3.01M | 0.200 | 0.64× | 0.60× |
| 10,000 | 2.984 | 3.35M | 2.987 | 3.35M | 1.256 | 0.42× | 0.42× |
| 100,000 | 32.106 | 3.11M | 30.405 | 3.29M | 16.217 | 0.51× | 0.53× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.155 | 0.110 | 0.71× |
| 1 | 5 | 0.370 | 0.419 | 1.13× |
| 1 | 10 | 0.581 | 1.005 | 1.73× |
| 10 | 1 | 0.067 | 0.082 | 1.21× |
| 10 | 5 | 0.295 | 0.434 | 1.47× |
| 10 | 10 | 0.611 | 0.852 | 1.40× |
| 100 | 1 | 0.103 | 0.191 | 1.86× |
| 100 | 5 | 0.307 | 0.928 | 3.02× |
| 100 | 10 | 0.644 | 1.917 | 2.98× |
| 1,000 | 1 | 0.389 | 0.295 | 0.76× |
| 1,000 | 5 | 0.561 | 1.036 | 1.85× |
| 1,000 | 10 | 1.004 | 2.205 | 2.20× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
