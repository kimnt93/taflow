# ExponentiallyWeightedStandardDeviation benchmark (`ewm standard deviation` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 119.30M | 0.007 | 140.69M | 1.365 | 162.80× | 192.00× |
| 10,000 | 0.069 | 144.86M | 0.063 | 159.50M | 18.297 | 265.06× | 291.84× |
| 100,000 | 0.469 | 213.15M | 0.451 | 221.76M | 131.643 | 280.59× | 291.93× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.083 | 0.118 | 1.43× |
| 1 | 5 | 0.240 | 0.452 | 1.88× |
| 1 | 10 | 0.459 | 0.952 | 2.08× |
| 10 | 1 | 0.060 | 0.104 | 1.73× |
| 10 | 5 | 0.237 | 0.558 | 2.35× |
| 10 | 10 | 0.530 | 1.012 | 1.91× |
| 100 | 1 | 0.063 | 0.223 | 3.57× |
| 100 | 5 | 0.268 | 1.098 | 4.10× |
| 100 | 10 | 0.513 | 2.271 | 4.43× |
| 1,000 | 1 | 0.056 | 1.340 | 24.00× |
| 1,000 | 5 | 0.279 | 6.934 | 24.90× |
| 1,000 | 10 | 0.539 | 13.676 | 25.39× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
