# Hurst benchmark (`HurstExponent` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 1.662 | 601.55K | 1.637 | 610.92K | 0.472 | 0.28× | 0.29× |
| 10,000 | 16.893 | 591.95K | 16.555 | 604.06K | 3.276 | 0.19× | 0.20× |
| 100,000 | 174.904 | 571.74K | 166.416 | 600.90K | 31.112 | 0.18× | 0.19× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.109 | 0.327 | 3.00× |
| 1 | 5 | 0.405 | 1.252 | 3.09× |
| 1 | 10 | 0.565 | 2.509 | 4.44× |
| 10 | 1 | 0.073 | 0.232 | 3.19× |
| 10 | 5 | 0.287 | 1.456 | 5.07× |
| 10 | 10 | 0.603 | 2.709 | 4.49× |
| 100 | 1 | 0.219 | 0.272 | 1.24× |
| 100 | 5 | 0.408 | 1.606 | 3.93× |
| 100 | 10 | 0.698 | 2.834 | 4.06× |
| 1,000 | 1 | 1.823 | 0.598 | 0.33× |
| 1,000 | 5 | 2.018 | 3.249 | 1.61× |
| 1,000 | 10 | 3.633 | 6.399 | 1.76× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
