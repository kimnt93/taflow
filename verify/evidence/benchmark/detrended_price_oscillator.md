# DetrendedPriceOscillator benchmark (`dpo` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 124.19M | 0.007 | 136.38M | 0.351 | 43.57× | 47.84× |
| 10,000 | 0.051 | 195.02M | 0.048 | 208.21M | 0.418 | 8.15× | 8.71× |
| 100,000 | 0.468 | 213.72M | 0.440 | 227.07M | 1.263 | 2.70× | 2.87× |
| 1,000,000 | 4.700 | 212.76M | 4.216 | 237.22M | 13.294 | 2.83× | 3.15× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.079 | 0.169 | 2.15× |
| 1 | 5 | 0.342 | 0.814 | 2.38× |
| 1 | 10 | 0.489 | 1.519 | 3.11× |
| 10 | 1 | 0.052 | 0.153 | 2.96× |
| 10 | 5 | 0.216 | 0.759 | 3.52× |
| 10 | 10 | 0.476 | 1.540 | 3.23× |
| 100 | 1 | 0.049 | 0.423 | 8.67× |
| 100 | 5 | 0.229 | 1.920 | 8.37× |
| 100 | 10 | 0.484 | 3.830 | 7.91× |
| 1,000 | 1 | 0.060 | 0.392 | 6.57× |
| 1,000 | 5 | 0.235 | 1.990 | 8.48× |
| 1,000 | 10 | 0.490 | 4.271 | 8.71× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
