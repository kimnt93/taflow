# Hurst benchmark (`HurstExponent` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.337 | 2.97M | 0.351 | 2.85M | 0.478 | 1.42× | 1.36× |
| 10,000 | 3.366 | 2.97M | 3.342 | 2.99M | 3.244 | 0.96× | 0.97× |
| 100,000 | 34.391 | 2.91M | 34.098 | 2.93M | 32.063 | 0.93× | 0.94× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.054 | 0.264 | 4.88× |
| 1 | 5 | 0.317 | 1.259 | 3.97× |
| 1 | 10 | 0.393 | 2.404 | 6.12× |
| 10 | 1 | 0.041 | 0.246 | 5.94× |
| 10 | 5 | 0.182 | 1.302 | 7.14× |
| 10 | 10 | 0.383 | 2.596 | 6.78× |
| 100 | 1 | 0.074 | 0.259 | 3.52× |
| 100 | 5 | 0.201 | 7.487 | 37.24× |
| 100 | 10 | 0.462 | 2.756 | 5.97× |
| 1,000 | 1 | 0.403 | 0.577 | 1.43× |
| 1,000 | 5 | 0.561 | 2.988 | 5.33× |
| 1,000 | 10 | 0.826 | 5.890 | 7.13× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
