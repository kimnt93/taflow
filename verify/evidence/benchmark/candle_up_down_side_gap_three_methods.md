# CandleUpDownSideGapThreeMethods benchmark (`CDLXSIDEGAP3METHODS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 217.34M | 0.003 | 349.26M | 0.034 | 7.43× | 11.95× |
| 10,000 | 0.021 | 470.76M | 0.019 | 517.33M | 0.088 | 4.13× | 4.54× |
| 100,000 | 0.186 | 536.26M | 0.199 | 501.52M | 0.645 | 3.46× | 3.23× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.055 | 0.138 | 2.52× |
| 1 | 5 | 0.234 | 0.446 | 1.91× |
| 1 | 10 | 0.392 | 0.895 | 2.28× |
| 10 | 1 | 0.038 | 0.085 | 2.21× |
| 10 | 5 | 0.178 | 0.476 | 2.67× |
| 10 | 10 | 0.378 | 0.898 | 2.38× |
| 100 | 1 | 0.042 | 0.091 | 2.20× |
| 100 | 5 | 0.171 | 0.438 | 2.56× |
| 100 | 10 | 0.412 | 0.971 | 2.36× |
| 1,000 | 1 | 0.044 | 0.097 | 2.18× |
| 1,000 | 5 | 0.194 | 0.483 | 2.48× |
| 1,000 | 10 | 0.444 | 1.036 | 2.33× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
