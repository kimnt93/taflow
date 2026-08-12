# FibonacciConfluence benchmark (`FibConfluence` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.487 | 2.05M | 0.500 | 2.00M | 1.823 | 3.74× | 3.64× |
| 10,000 | 5.530 | 1.81M | 5.299 | 1.89M | 18.446 | 3.34× | 3.48× |
| 100,000 | 52.122 | 1.92M | 52.324 | 1.91M | 190.710 | 3.66× | 3.64× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.135 | 0.219 | 1.62× |
| 1 | 5 | 0.234 | 0.904 | 3.86× |
| 1 | 10 | 0.482 | 1.924 | 3.99× |
| 10 | 1 | 0.056 | 0.175 | 3.12× |
| 10 | 5 | 0.247 | 0.910 | 3.69× |
| 10 | 10 | 0.504 | 1.924 | 3.82× |
| 100 | 1 | 0.079 | 0.323 | 4.09× |
| 100 | 5 | 0.275 | 1.542 | 5.60× |
| 100 | 10 | 0.525 | 3.446 | 6.56× |
| 1,000 | 1 | 0.587 | 2.308 | 3.93× |
| 1,000 | 5 | 0.892 | 11.377 | 12.75× |
| 1,000 | 10 | 1.398 | 23.100 | 16.52× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
