# DecyclerOscillator benchmark (`DecyclerOscillator` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 95.85M | 0.010 | 101.45M | 0.178 | 17.04× | 18.04× |
| 10,000 | 0.078 | 127.72M | 0.080 | 125.13M | 0.568 | 7.25× | 7.10× |
| 100,000 | 0.750 | 133.29M | 0.738 | 135.48M | 3.742 | 4.99× | 5.07× |
| 1,000,000 | 8.480 | 117.92M | 7.492 | 133.47M | 37.436 | 4.41× | 5.00× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.158 | 0.245 | 1.55× |
| 1 | 5 | 0.395 | 1.106 | 2.80× |
| 1 | 10 | 0.502 | 2.272 | 4.53× |
| 10 | 1 | 0.049 | 0.213 | 4.35× |
| 10 | 5 | 0.233 | 1.052 | 4.52× |
| 10 | 10 | 0.484 | 2.317 | 4.78× |
| 100 | 1 | 0.050 | 0.212 | 4.27× |
| 100 | 5 | 0.246 | 1.053 | 4.28× |
| 100 | 10 | 0.503 | 2.328 | 4.62× |
| 1,000 | 1 | 0.063 | 0.250 | 4.00× |
| 1,000 | 5 | 0.244 | 1.237 | 5.07× |
| 1,000 | 10 | 0.518 | 2.730 | 5.27× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
