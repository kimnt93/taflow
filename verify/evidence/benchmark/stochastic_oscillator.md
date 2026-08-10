# StochasticOscillator benchmark (`STOCH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.016 | 62.86M | 0.014 | 69.99M | 0.051 | 3.21× | 3.57× |
| 10,000 | 0.156 | 64.04M | 0.130 | 77.16M | 0.157 | 1.01× | 1.22× |
| 100,000 | 1.318 | 75.89M | 1.267 | 78.93M | 1.162 | 0.88× | 0.92× |
| 1,000,000 | 15.658 | 63.86M | 14.946 | 66.91M | 12.182 | 0.78× | 0.82× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.075 | 0.145 | 1.92× |
| 1 | 5 | 0.362 | 0.564 | 1.56× |
| 1 | 10 | 0.526 | 1.073 | 2.04× |
| 10 | 1 | 0.050 | 0.100 | 1.98× |
| 10 | 5 | 0.250 | 0.500 | 2.00× |
| 10 | 10 | 0.502 | 1.056 | 2.10× |
| 100 | 1 | 0.064 | 0.106 | 1.66× |
| 100 | 5 | 0.251 | 0.517 | 2.06× |
| 100 | 10 | 0.559 | 1.065 | 1.90× |
| 1,000 | 1 | 0.066 | 0.121 | 1.84× |
| 1,000 | 5 | 0.258 | 0.600 | 2.32× |
| 1,000 | 10 | 0.545 | 1.173 | 2.15× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
