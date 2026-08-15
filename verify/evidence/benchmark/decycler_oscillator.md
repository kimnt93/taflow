# DecyclerOscillator benchmark (`DecyclerOscillator` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 120.69M | 0.007 | 139.59M | 0.170 | 20.55× | 23.76× |
| 10,000 | 0.065 | 154.35M | 0.063 | 158.51M | 0.507 | 7.82× | 8.03× |
| 100,000 | 0.619 | 161.51M | 0.578 | 172.93M | 3.761 | 6.07× | 6.50× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.068 | 0.278 | 4.07× |
| 1 | 5 | 0.239 | 1.041 | 4.36× |
| 1 | 10 | 0.415 | 2.352 | 5.67× |
| 10 | 1 | 0.051 | 0.207 | 4.02× |
| 10 | 5 | 0.187 | 1.038 | 5.55× |
| 10 | 10 | 0.424 | 2.352 | 5.55× |
| 100 | 1 | 0.048 | 0.212 | 4.46× |
| 100 | 5 | 0.200 | 1.039 | 5.20× |
| 100 | 10 | 0.447 | 2.379 | 5.33× |
| 1,000 | 1 | 0.056 | 0.249 | 4.48× |
| 1,000 | 5 | 0.224 | 1.328 | 5.92× |
| 1,000 | 10 | 0.456 | 2.756 | 6.05× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
