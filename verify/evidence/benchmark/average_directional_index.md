# AverageDirectionalIndex benchmark (`ADX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 79.78M | 0.012 | 86.93M | 0.039 | 3.08× | 3.36× |
| 10,000 | 0.086 | 115.83M | 0.081 | 122.77M | 0.118 | 1.37× | 1.45× |
| 100,000 | 0.807 | 123.87M | 0.789 | 126.80M | 0.889 | 1.10× | 1.13× |
| 1,000,000 | 9.299 | 107.54M | 8.339 | 119.92M | 9.218 | 0.99× | 1.11× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.078 | 0.111 | 1.43× |
| 1 | 5 | 0.347 | 0.489 | 1.41× |
| 1 | 10 | 0.488 | 0.971 | 1.99× |
| 10 | 1 | 0.049 | 0.098 | 1.99× |
| 10 | 5 | 0.230 | 0.459 | 2.00× |
| 10 | 10 | 0.486 | 0.947 | 1.95× |
| 100 | 1 | 0.050 | 0.091 | 1.80× |
| 100 | 5 | 0.227 | 0.446 | 1.96× |
| 100 | 10 | 0.496 | 0.940 | 1.90× |
| 1,000 | 1 | 0.065 | 0.101 | 1.57× |
| 1,000 | 5 | 0.243 | 0.499 | 2.06× |
| 1,000 | 10 | 0.504 | 1.062 | 2.10× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
