# IntradayVolatilityProfile benchmark (`IntradayVolatilityProfile` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.065 | 15.34M | 0.055 | 18.11M | 1.700 | 26.08× | 30.78× |
| 10,000 | 0.577 | 17.34M | 0.524 | 19.10M | 15.093 | 26.17× | 28.83× |
| 100,000 | 6.489 | 15.41M | 5.039 | 19.84M | 185.505 | 28.59× | 36.81× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.072 | 0.325 | 4.49× |
| 1 | 5 | 0.260 | 1.442 | 5.55× |
| 1 | 10 | 0.444 | 2.706 | 6.10× |
| 10 | 1 | 0.049 | 0.273 | 5.59× |
| 10 | 5 | 0.195 | 1.540 | 7.89× |
| 10 | 10 | 0.488 | 3.060 | 6.28× |
| 100 | 1 | 0.050 | 0.425 | 8.51× |
| 100 | 5 | 0.221 | 2.195 | 9.93× |
| 100 | 10 | 0.450 | 4.498 | 9.99× |
| 1,000 | 1 | 0.100 | 2.018 | 20.08× |
| 1,000 | 5 | 0.237 | 10.032 | 42.28× |
| 1,000 | 10 | 0.528 | 19.967 | 37.81× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
