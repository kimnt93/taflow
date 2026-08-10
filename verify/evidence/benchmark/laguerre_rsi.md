# LaguerreRelativeStrengthIndex benchmark (`LaguerreRSI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 96.43M | 0.010 | 97.07M | 0.188 | 18.11× | 18.23× |
| 10,000 | 0.081 | 123.54M | 0.075 | 133.97M | 0.555 | 6.86× | 7.44× |
| 100,000 | 0.754 | 132.70M | 0.816 | 122.56M | 4.394 | 5.83× | 5.39× |
| 1,000,000 | 7.557 | 132.33M | 7.494 | 133.44M | 41.590 | 5.50× | 5.55× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.084 | 0.258 | 3.09× |
| 1 | 5 | 0.270 | 1.315 | 4.87× |
| 1 | 10 | 0.460 | 2.336 | 5.08× |
| 10 | 1 | 0.051 | 0.208 | 4.04× |
| 10 | 5 | 0.235 | 1.389 | 5.91× |
| 10 | 10 | 0.454 | 2.332 | 5.14× |
| 100 | 1 | 0.048 | 0.212 | 4.43× |
| 100 | 5 | 0.237 | 1.354 | 5.70× |
| 100 | 10 | 0.471 | 2.400 | 5.09× |
| 1,000 | 1 | 0.064 | 0.259 | 4.02× |
| 1,000 | 5 | 0.224 | 1.566 | 6.97× |
| 1,000 | 10 | 0.504 | 2.848 | 5.65× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
