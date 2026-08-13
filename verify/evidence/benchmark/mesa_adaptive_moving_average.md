# MesaAdaptiveMovingAverage benchmark (`MAMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.146 | 6.83M | 0.132 | 7.59M | 0.083 | 0.57× | 0.63× |
| 10,000 | 1.262 | 7.93M | 1.271 | 7.87M | 0.528 | 0.42× | 0.42× |
| 100,000 | 13.612 | 7.35M | 12.620 | 7.92M | 4.886 | 0.36× | 0.39× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.083 | 0.140 | 1.68× |
| 1 | 5 | 0.457 | 0.494 | 1.08× |
| 1 | 10 | 0.624 | 3.400 | 5.45× |
| 10 | 1 | 0.107 | 0.148 | 1.38× |
| 10 | 5 | 0.403 | 0.728 | 1.81× |
| 10 | 10 | 0.795 | 1.463 | 1.84× |
| 100 | 1 | 0.144 | 0.158 | 1.10× |
| 100 | 5 | 0.406 | 0.576 | 1.42× |
| 100 | 10 | 0.733 | 1.242 | 1.69× |
| 1,000 | 1 | 0.203 | 0.156 | 0.77× |
| 1,000 | 5 | 0.433 | 0.795 | 1.84× |
| 1,000 | 10 | 0.726 | 1.560 | 2.15× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
