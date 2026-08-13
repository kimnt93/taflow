# WeightedClose benchmark (`WCLPRICE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.031 | 32.75M | 0.024 | 42.35M | 0.028 | 0.90× | 1.17× |
| 10,000 | 0.176 | 56.69M | 0.152 | 65.63M | 0.033 | 0.19× | 0.22× |
| 100,000 | 1.470 | 68.02M | 1.420 | 70.44M | 0.087 | 0.06× | 0.06× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.104 | 0.154 | 1.48× |
| 1 | 5 | 0.384 | 0.440 | 1.15× |
| 1 | 10 | 0.640 | 0.901 | 1.41× |
| 10 | 1 | 0.066 | 0.086 | 1.30× |
| 10 | 5 | 0.291 | 0.426 | 1.46× |
| 10 | 10 | 0.654 | 0.964 | 1.47× |
| 100 | 1 | 0.069 | 0.091 | 1.33× |
| 100 | 5 | 0.306 | 0.424 | 1.39× |
| 100 | 10 | 0.604 | 0.881 | 1.46× |
| 1,000 | 1 | 0.082 | 0.086 | 1.04× |
| 1,000 | 5 | 0.298 | 0.413 | 1.39× |
| 1,000 | 10 | 0.634 | 0.904 | 1.43× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
