# StochasticOscillator benchmark (`STOCH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 51.67M | 0.019 | 53.86M | 0.066 | 3.42× | 3.56× |
| 10,000 | 0.172 | 58.06M | 0.161 | 62.01M | 0.186 | 1.08× | 1.15× |
| 100,000 | 1.597 | 62.62M | 1.628 | 61.43M | 1.437 | 0.90× | 0.88× |
| 1,000,000 | 18.184 | 54.99M | 16.940 | 59.03M | 13.917 | 0.77× | 0.82× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.103 | 0.157 | 1.52× |
| 1 | 5 | 0.278 | 0.546 | 1.97× |
| 1 | 10 | 0.537 | 1.212 | 2.26× |
| 10 | 1 | 0.063 | 0.115 | 1.82× |
| 10 | 5 | 0.309 | 0.618 | 2.00× |
| 10 | 10 | 0.626 | 1.183 | 1.89× |
| 100 | 1 | 0.064 | 0.117 | 1.83× |
| 100 | 5 | 0.352 | 0.630 | 1.79× |
| 100 | 10 | 0.590 | 1.101 | 1.87× |
| 1,000 | 1 | 0.085 | 0.153 | 1.80× |
| 1,000 | 5 | 0.306 | 0.682 | 2.23× |
| 1,000 | 10 | 0.645 | 1.260 | 1.95× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
