# CloseToCloseSigma benchmark (`annualized close-to-close volatility` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 52.06M | 0.019 | 53.84M | 0.144 | 7.48× | 7.74× |
| 10,000 | 0.193 | 51.71M | 0.198 | 50.61M | 0.744 | 3.85× | 3.77× |
| 100,000 | 1.805 | 55.41M | 1.728 | 57.86M | 6.644 | 3.68× | 3.84× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.113 | 0.149 | 1.31× |
| 1 | 5 | 0.303 | 0.641 | 2.12× |
| 1 | 10 | 0.394 | 1.180 | 2.99× |
| 10 | 1 | 0.047 | 0.115 | 2.43× |
| 10 | 5 | 0.183 | 0.577 | 3.15× |
| 10 | 10 | 0.389 | 1.238 | 3.18× |
| 100 | 1 | 0.044 | 0.178 | 4.07× |
| 100 | 5 | 0.199 | 1.110 | 5.59× |
| 100 | 10 | 0.534 | 2.024 | 3.79× |
| 1,000 | 1 | 0.069 | 0.240 | 3.49× |
| 1,000 | 5 | 0.197 | 1.194 | 6.07× |
| 1,000 | 10 | 0.413 | 2.203 | 5.34× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
