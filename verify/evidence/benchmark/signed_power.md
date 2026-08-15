# SignedPower benchmark (`numpy.sign/abs/power` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 267.69M | 0.003 | 339.71M | 0.026 | 6.86× | 8.71× |
| 10,000 | 0.025 | 407.59M | 0.019 | 522.91M | 0.047 | 1.90× | 2.44× |
| 100,000 | 0.210 | 477.03M | 0.166 | 603.41M | 0.199 | 0.95× | 1.20× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.081 | 0.138 | 1.70× |
| 1 | 5 | 0.265 | 0.444 | 1.68× |
| 1 | 10 | 0.413 | 0.858 | 2.08× |
| 10 | 1 | 0.044 | 0.094 | 2.14× |
| 10 | 5 | 0.189 | 0.414 | 2.19× |
| 10 | 10 | 0.402 | 0.857 | 2.13× |
| 100 | 1 | 0.044 | 0.091 | 2.08× |
| 100 | 5 | 0.184 | 0.424 | 2.30× |
| 100 | 10 | 0.378 | 0.876 | 2.32× |
| 1,000 | 1 | 0.044 | 0.091 | 2.08× |
| 1,000 | 5 | 0.195 | 0.457 | 2.35× |
| 1,000 | 10 | 0.418 | 0.994 | 2.38× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
