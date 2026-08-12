# CumulativeVolumeIndex benchmark (`CumulativeVolumeIndex` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 136.34M | 0.006 | 178.13M | 4.370 | 595.87× | 778.47× |
| 10,000 | 0.032 | 311.30M | 0.029 | 342.20M | 42.772 | 1331.50× | 1463.66× |
| 100,000 | 0.290 | 345.25M | 0.270 | 370.25M | 424.832 | 1466.74× | 1572.95× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.182 | 0.263 | 1.44× |
| 1 | 5 | 0.314 | 1.266 | 4.03× |
| 1 | 10 | 0.440 | 2.128 | 4.84× |
| 10 | 1 | 0.050 | 0.239 | 4.80× |
| 10 | 5 | 0.244 | 1.161 | 4.76× |
| 10 | 10 | 0.533 | 2.648 | 4.97× |
| 100 | 1 | 0.054 | 0.639 | 11.92× |
| 100 | 5 | 0.242 | 3.229 | 13.32× |
| 100 | 10 | 0.525 | 6.565 | 12.51× |
| 1,000 | 1 | 0.060 | 4.482 | 74.50× |
| 1,000 | 5 | 0.276 | 26.876 | 97.40× |
| 1,000 | 10 | 0.574 | 60.423 | 105.33× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
