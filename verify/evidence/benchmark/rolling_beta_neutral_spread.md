# RollingBetaNeutralSpread benchmark (`BetaNeutralSpread` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.052 | 19.32M | 0.050 | 20.19M | 0.223 | 4.30× | 4.50× |
| 10,000 | 0.476 | 21.00M | 0.466 | 21.48M | 0.961 | 2.02× | 2.06× |
| 100,000 | 4.860 | 20.58M | 4.664 | 21.44M | 8.174 | 1.68× | 1.75× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.168 | 0.256 | 1.53× |
| 1 | 5 | 0.250 | 1.063 | 4.25× |
| 1 | 10 | 0.431 | 2.214 | 5.14× |
| 10 | 1 | 0.045 | 0.204 | 4.56× |
| 10 | 5 | 0.194 | 1.275 | 6.58× |
| 10 | 10 | 0.437 | 2.241 | 5.13× |
| 100 | 1 | 0.052 | 0.219 | 4.25× |
| 100 | 5 | 0.225 | 1.322 | 5.87× |
| 100 | 10 | 0.414 | 2.340 | 5.65× |
| 1,000 | 1 | 0.101 | 0.303 | 3.00× |
| 1,000 | 5 | 0.238 | 1.647 | 6.93× |
| 1,000 | 10 | 0.449 | 3.111 | 6.93× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
