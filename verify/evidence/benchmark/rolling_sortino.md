# RollingSortino benchmark (`SortinoRatio` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.416 | 2.40M | 0.402 | 2.49M | 0.184 | 0.44× | 0.46× |
| 10,000 | 4.050 | 2.47M | 3.863 | 2.59M | 0.708 | 0.17× | 0.18× |
| 100,000 | 42.298 | 2.36M | 38.818 | 2.58M | 5.189 | 0.12× | 0.13× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.099 | 0.296 | 2.98× |
| 1 | 5 | 0.465 | 1.118 | 2.40× |
| 1 | 10 | 0.645 | 2.350 | 3.65× |
| 10 | 1 | 0.070 | 0.212 | 3.01× |
| 10 | 5 | 0.299 | 1.236 | 4.13× |
| 10 | 10 | 0.594 | 2.254 | 3.79× |
| 100 | 1 | 0.106 | 0.220 | 2.07× |
| 100 | 5 | 0.293 | 1.281 | 4.37× |
| 100 | 10 | 0.648 | 2.316 | 3.58× |
| 1,000 | 1 | 0.473 | 0.272 | 0.57× |
| 1,000 | 5 | 0.742 | 1.559 | 2.10× |
| 1,000 | 10 | 1.173 | 2.865 | 2.44× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
