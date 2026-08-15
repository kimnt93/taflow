# UlcerIndex benchmark (`UlcerIndex` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 56.20M | 0.017 | 58.62M | 0.193 | 10.85× | 11.32× |
| 10,000 | 0.207 | 48.36M | 0.194 | 51.68M | 0.608 | 2.94× | 3.14× |
| 100,000 | 1.968 | 50.82M | 1.939 | 51.58M | 4.845 | 2.46× | 2.50× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.067 | 0.280 | 4.20× |
| 1 | 5 | 0.247 | 1.100 | 4.45× |
| 1 | 10 | 0.406 | 2.491 | 6.14× |
| 10 | 1 | 0.047 | 0.235 | 4.97× |
| 10 | 5 | 0.212 | 1.380 | 6.51× |
| 10 | 10 | 0.436 | 2.502 | 5.74× |
| 100 | 1 | 0.047 | 0.218 | 4.62× |
| 100 | 5 | 0.197 | 1.447 | 7.35× |
| 100 | 10 | 0.423 | 2.440 | 5.77× |
| 1,000 | 1 | 0.068 | 0.271 | 4.00× |
| 1,000 | 5 | 0.227 | 1.754 | 7.74× |
| 1,000 | 10 | 0.433 | 2.949 | 6.81× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
