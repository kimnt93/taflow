# Squeeze benchmark (`squeeze` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.054 | 18.46M | 0.047 | 21.19M | 4.520 | 83.45× | 95.81× |
| 10,000 | 0.396 | 25.23M | 0.381 | 26.28M | 8.396 | 21.19× | 22.06× |
| 100,000 | 3.908 | 25.59M | 3.762 | 26.58M | 27.843 | 7.12× | 7.40× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.108 | 0.415 | 3.84× |
| 1 | 5 | 0.491 | 1.693 | 3.45× |
| 1 | 10 | 0.542 | 3.261 | 6.02× |
| 10 | 1 | 0.057 | 0.347 | 6.13× |
| 10 | 5 | 0.271 | 1.641 | 6.05× |
| 10 | 10 | 0.554 | 3.277 | 5.92× |
| 100 | 1 | 0.061 | 4.808 | 78.42× |
| 100 | 5 | 0.289 | 25.169 | 87.03× |
| 100 | 10 | 0.578 | 50.982 | 88.15× |
| 1,000 | 1 | 0.106 | 5.002 | 47.34× |
| 1,000 | 5 | 0.386 | 27.198 | 70.47× |
| 1,000 | 10 | 0.585 | 53.850 | 91.99× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
