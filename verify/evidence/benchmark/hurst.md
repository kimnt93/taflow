# Hurst benchmark (`HurstExponent` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.349 | 2.87M | 0.338 | 2.96M | 0.478 | 1.37× | 1.41× |
| 10,000 | 3.577 | 2.80M | 3.432 | 2.91M | 3.302 | 0.92× | 0.96× |
| 100,000 | 34.928 | 2.86M | 34.631 | 2.89M | 33.029 | 0.95× | 0.95× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.108 | 0.282 | 2.60× |
| 1 | 5 | 0.315 | 1.553 | 4.94× |
| 1 | 10 | 0.397 | 2.930 | 7.38× |
| 10 | 1 | 0.045 | 0.251 | 5.56× |
| 10 | 5 | 0.192 | 1.483 | 7.72× |
| 10 | 10 | 0.400 | 2.568 | 6.42× |
| 100 | 1 | 0.079 | 0.288 | 3.67× |
| 100 | 5 | 0.206 | 1.649 | 8.01× |
| 100 | 10 | 0.457 | 3.189 | 6.98× |
| 1,000 | 1 | 0.417 | 0.589 | 1.41× |
| 1,000 | 5 | 0.544 | 3.200 | 5.88× |
| 1,000 | 10 | 0.829 | 6.038 | 7.28× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
