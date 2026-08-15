# WedgePattern benchmark (`Wedge` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 97.33M | 0.007 | 142.38M | 0.231 | 22.47× | 32.86× |
| 10,000 | 0.089 | 112.54M | 0.077 | 129.81M | 1.419 | 15.97× | 18.42× |
| 100,000 | 0.816 | 122.49M | 0.808 | 123.77M | 12.670 | 15.52× | 15.68× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.063 | 0.189 | 2.99× |
| 1 | 5 | 0.263 | 0.789 | 3.00× |
| 1 | 10 | 0.448 | 1.667 | 3.72× |
| 10 | 1 | 0.047 | 0.167 | 3.53× |
| 10 | 5 | 0.194 | 1.100 | 5.66× |
| 10 | 10 | 0.457 | 1.726 | 3.77× |
| 100 | 1 | 0.046 | 0.182 | 3.99× |
| 100 | 5 | 0.205 | 1.143 | 5.58× |
| 100 | 10 | 0.439 | 1.819 | 4.14× |
| 1,000 | 1 | 0.054 | 0.291 | 5.40× |
| 1,000 | 5 | 0.197 | 1.804 | 9.13× |
| 1,000 | 10 | 0.414 | 2.952 | 7.13× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
