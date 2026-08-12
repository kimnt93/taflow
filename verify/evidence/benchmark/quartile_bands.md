# QuartileBands benchmark (`QuartileBands` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.075 | 13.33M | 0.074 | 13.56M | 0.710 | 9.47× | 9.63× |
| 10,000 | 0.739 | 13.53M | 0.734 | 13.62M | 5.577 | 7.55× | 7.59× |
| 100,000 | 7.633 | 13.10M | 7.451 | 13.42M | 65.275 | 8.55× | 8.76× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.085 | 0.268 | 3.16× |
| 1 | 5 | 0.337 | 1.325 | 3.93× |
| 1 | 10 | 0.571 | 2.479 | 4.34× |
| 10 | 1 | 0.053 | 0.217 | 4.13× |
| 10 | 5 | 0.271 | 1.293 | 4.76× |
| 10 | 10 | 0.498 | 2.374 | 4.77× |
| 100 | 1 | 0.060 | 0.274 | 4.57× |
| 100 | 5 | 0.236 | 1.557 | 6.60× |
| 100 | 10 | 0.512 | 2.897 | 5.66× |
| 1,000 | 1 | 0.131 | 0.945 | 7.21× |
| 1,000 | 5 | 0.278 | 4.306 | 15.49× |
| 1,000 | 10 | 0.541 | 15.057 | 27.84× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
