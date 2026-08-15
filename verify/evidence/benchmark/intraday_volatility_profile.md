# IntradayVolatilityProfile benchmark (`IntradayVolatilityProfile` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.068 | 14.77M | 0.056 | 17.87M | 1.835 | 27.10× | 32.78× |
| 10,000 | 0.657 | 15.22M | 0.562 | 17.80M | 14.821 | 22.55× | 26.38× |
| 100,000 | 6.467 | 15.46M | 5.309 | 18.84M | 195.533 | 30.23× | 36.83× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.079 | 0.302 | 3.85× |
| 1 | 5 | 0.333 | 1.446 | 4.34× |
| 1 | 10 | 0.422 | 2.646 | 6.26× |
| 10 | 1 | 0.046 | 0.268 | 5.80× |
| 10 | 5 | 0.245 | 1.574 | 6.42× |
| 10 | 10 | 0.425 | 2.962 | 6.98× |
| 100 | 1 | 0.065 | 0.408 | 6.27× |
| 100 | 5 | 0.209 | 2.131 | 10.21× |
| 100 | 10 | 0.478 | 4.082 | 8.54× |
| 1,000 | 1 | 0.100 | 2.072 | 20.70× |
| 1,000 | 5 | 0.227 | 9.654 | 42.56× |
| 1,000 | 10 | 0.478 | 26.094 | 54.60× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
