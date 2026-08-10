# InstantaneousTrendline benchmark (`InstantaneousTrendline` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 108.29M | 0.008 | 127.57M | 0.146 | 15.84× | 18.66× |
| 10,000 | 0.058 | 172.29M | 0.055 | 180.82M | 0.471 | 8.11× | 8.51× |
| 100,000 | 0.542 | 184.47M | 0.547 | 182.67M | 3.672 | 6.77× | 6.71× |
| 1,000,000 | 5.489 | 182.19M | 8.011 | 124.82M | 38.269 | 6.97× | 4.78× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.081 | 0.260 | 3.20× |
| 1 | 5 | 0.331 | 0.947 | 2.87× |
| 1 | 10 | 0.447 | 2.007 | 4.49× |
| 10 | 1 | 0.047 | 0.197 | 4.16× |
| 10 | 5 | 0.228 | 0.939 | 4.12× |
| 10 | 10 | 0.464 | 2.047 | 4.41× |
| 100 | 1 | 0.054 | 0.183 | 3.36× |
| 100 | 5 | 0.218 | 0.945 | 4.33× |
| 100 | 10 | 0.465 | 2.166 | 4.66× |
| 1,000 | 1 | 0.060 | 0.239 | 3.99× |
| 1,000 | 5 | 0.241 | 1.122 | 4.65× |
| 1,000 | 10 | 0.523 | 2.557 | 4.89× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
