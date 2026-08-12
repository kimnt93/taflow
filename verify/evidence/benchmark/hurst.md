# Hurst benchmark (`HurstExponent` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.418 | 2.39M | 0.433 | 2.31M | 0.537 | 1.28× | 1.24× |
| 10,000 | 4.087 | 2.45M | 4.083 | 2.45M | 3.354 | 0.82× | 0.82× |
| 100,000 | 39.867 | 2.51M | 42.252 | 2.37M | 33.923 | 0.85× | 0.80× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.088 | 0.302 | 3.42× |
| 1 | 5 | 0.348 | 1.633 | 4.69× |
| 1 | 10 | 0.473 | 3.097 | 6.55× |
| 10 | 1 | 0.054 | 0.252 | 4.68× |
| 10 | 5 | 0.232 | 1.704 | 7.33× |
| 10 | 10 | 0.471 | 2.714 | 5.76× |
| 100 | 1 | 0.094 | 0.353 | 3.74× |
| 100 | 5 | 0.295 | 1.794 | 6.07× |
| 100 | 10 | 0.531 | 3.316 | 6.25× |
| 1,000 | 1 | 0.488 | 0.612 | 1.25× |
| 1,000 | 5 | 0.619 | 3.321 | 5.37× |
| 1,000 | 10 | 0.985 | 6.287 | 6.39× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
