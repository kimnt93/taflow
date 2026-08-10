# KeltnerChannels benchmark (`Keltner` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.021 | 47.74M | 0.017 | 58.89M | 0.579 | 27.65× | 34.11× |
| 10,000 | 0.130 | 76.66M | 0.125 | 79.68M | 4.182 | 32.06× | 33.33× |
| 100,000 | 1.380 | 72.49M | 1.202 | 83.21M | 46.213 | 33.50× | 38.45× |
| 1,000,000 | 22.081 | 45.29M | 12.884 | 77.62M | 472.509 | 21.40× | 36.67× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.075 | 0.390 | 5.19× |
| 1 | 5 | 0.438 | 1.601 | 3.66× |
| 1 | 10 | 0.553 | 3.379 | 6.11× |
| 10 | 1 | 0.056 | 0.308 | 5.49× |
| 10 | 5 | 0.258 | 1.635 | 6.34× |
| 10 | 10 | 0.525 | 3.333 | 6.35× |
| 100 | 1 | 0.060 | 0.347 | 5.78× |
| 100 | 5 | 0.250 | 1.800 | 7.20× |
| 100 | 10 | 0.616 | 3.868 | 6.27× |
| 1,000 | 1 | 0.074 | 0.852 | 11.49× |
| 1,000 | 5 | 0.271 | 3.970 | 14.67× |
| 1,000 | 10 | 0.556 | 7.813 | 14.06× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
