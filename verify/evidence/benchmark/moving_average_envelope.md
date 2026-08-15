# MovingAverageEnvelope benchmark (`MaEnvelope` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.022 | 45.17M | 0.021 | 46.76M | 0.506 | 22.87× | 23.68× |
| 10,000 | 0.202 | 49.50M | 0.197 | 50.86M | 3.552 | 17.58× | 18.07× |
| 100,000 | 1.986 | 50.36M | 1.995 | 50.12M | 40.362 | 20.33× | 20.23× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.075 | 0.300 | 4.01× |
| 1 | 5 | 0.241 | 1.410 | 5.84× |
| 1 | 10 | 0.395 | 2.625 | 6.64× |
| 10 | 1 | 0.049 | 0.254 | 5.17× |
| 10 | 5 | 0.199 | 1.508 | 7.57× |
| 10 | 10 | 0.422 | 2.815 | 6.67× |
| 100 | 1 | 0.049 | 0.284 | 5.78× |
| 100 | 5 | 0.212 | 1.587 | 7.49× |
| 100 | 10 | 0.486 | 3.010 | 6.20× |
| 1,000 | 1 | 0.064 | 0.851 | 13.21× |
| 1,000 | 5 | 0.225 | 3.377 | 14.99× |
| 1,000 | 10 | 0.431 | 6.793 | 15.77× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
