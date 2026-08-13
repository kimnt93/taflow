# SharkPattern benchmark (`Shark` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.059 | 17.02M | 0.051 | 19.71M | 0.220 | 3.74× | 4.33× |
| 10,000 | 0.417 | 23.96M | 0.409 | 24.42M | 1.397 | 3.35× | 3.41× |
| 100,000 | 3.932 | 25.43M | 3.897 | 25.66M | 12.557 | 3.19× | 3.22× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.106 | 0.190 | 1.79× |
| 1 | 5 | 0.393 | 0.831 | 2.12× |
| 1 | 10 | 0.664 | 1.773 | 2.67× |
| 10 | 1 | 0.085 | 0.160 | 1.88× |
| 10 | 5 | 0.330 | 1.071 | 3.25× |
| 10 | 10 | 0.659 | 1.701 | 2.58× |
| 100 | 1 | 0.085 | 0.176 | 2.06× |
| 100 | 5 | 0.314 | 1.149 | 3.66× |
| 100 | 10 | 0.684 | 1.803 | 2.63× |
| 1,000 | 1 | 0.112 | 0.293 | 2.62× |
| 1,000 | 5 | 0.326 | 1.772 | 5.44× |
| 1,000 | 10 | 0.685 | 2.976 | 4.34× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
