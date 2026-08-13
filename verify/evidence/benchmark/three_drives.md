# ThreeDrives benchmark (`ThreeDrives` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.060 | 16.72M | 0.050 | 20.02M | 0.221 | 3.70× | 4.43× |
| 10,000 | 0.399 | 25.08M | 0.395 | 25.30M | 1.368 | 3.43× | 3.46× |
| 100,000 | 3.822 | 26.16M | 3.888 | 25.72M | 12.826 | 3.36× | 3.30× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.118 | 0.213 | 1.80× |
| 1 | 5 | 0.339 | 0.810 | 2.39× |
| 1 | 10 | 0.643 | 1.653 | 2.57× |
| 10 | 1 | 0.075 | 0.163 | 2.16× |
| 10 | 5 | 0.318 | 1.122 | 3.52× |
| 10 | 10 | 0.652 | 1.663 | 2.55× |
| 100 | 1 | 0.080 | 0.181 | 2.27× |
| 100 | 5 | 0.313 | 1.195 | 3.82× |
| 100 | 10 | 0.681 | 1.840 | 2.70× |
| 1,000 | 1 | 0.126 | 0.302 | 2.39× |
| 1,000 | 5 | 0.308 | 1.801 | 5.85× |
| 1,000 | 10 | 0.662 | 3.007 | 4.54× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
