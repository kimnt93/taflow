# Rising benchmark (`period-over-period rising` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 130.12M | 0.007 | 141.40M | 0.030 | 3.97× | 4.31× |
| 10,000 | 0.051 | 195.36M | 0.049 | 206.07M | 0.040 | 0.77× | 0.82× |
| 100,000 | 0.462 | 216.58M | 0.432 | 231.69M | 0.122 | 0.26× | 0.28× |
| 1,000,000 | 4.797 | 208.44M | 4.311 | 231.99M | 1.625 | 0.34× | 0.38× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.081 | 0.120 | 1.49× |
| 1 | 5 | 0.256 | 0.465 | 1.82× |
| 1 | 10 | 0.455 | 0.912 | 2.00× |
| 10 | 1 | 0.046 | 0.090 | 1.93× |
| 10 | 5 | 0.213 | 0.431 | 2.02× |
| 10 | 10 | 0.464 | 0.899 | 1.94× |
| 100 | 1 | 0.046 | 0.095 | 2.07× |
| 100 | 5 | 0.212 | 0.446 | 2.10× |
| 100 | 10 | 0.473 | 0.945 | 2.00× |
| 1,000 | 1 | 0.056 | 0.093 | 1.66× |
| 1,000 | 5 | 0.217 | 0.477 | 2.20× |
| 1,000 | 10 | 0.471 | 1.174 | 2.49× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
