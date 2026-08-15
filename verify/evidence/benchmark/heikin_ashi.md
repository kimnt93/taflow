# HeikinAshi benchmark (`HeikinAshi` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 89.61M | 0.008 | 129.36M | 0.576 | 51.63× | 74.52× |
| 10,000 | 0.078 | 128.28M | 0.071 | 141.82M | 4.561 | 58.51× | 64.69× |
| 100,000 | 0.805 | 124.20M | 0.684 | 146.23M | 52.851 | 65.64× | 77.29× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.060 | 0.245 | 4.07× |
| 1 | 5 | 0.261 | 0.909 | 3.49× |
| 1 | 10 | 0.394 | 2.069 | 5.26× |
| 10 | 1 | 0.043 | 0.183 | 4.23× |
| 10 | 5 | 0.195 | 0.882 | 4.53× |
| 10 | 10 | 0.398 | 2.106 | 5.29× |
| 100 | 1 | 0.046 | 0.229 | 4.93× |
| 100 | 5 | 0.200 | 1.126 | 5.63× |
| 100 | 10 | 0.379 | 2.524 | 6.66× |
| 1,000 | 1 | 0.053 | 0.951 | 17.89× |
| 1,000 | 5 | 0.210 | 3.691 | 17.57× |
| 1,000 | 10 | 0.478 | 7.416 | 15.53× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
