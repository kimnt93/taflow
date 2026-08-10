# PositiveVolumeIndex benchmark (`PVI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 102.79M | 0.008 | 118.86M | 0.183 | 18.79× | 21.73× |
| 10,000 | 0.065 | 154.19M | 0.062 | 160.44M | 0.790 | 12.19× | 12.68× |
| 100,000 | 0.584 | 171.28M | 0.641 | 156.12M | 9.456 | 16.20× | 14.76× |
| 1,000,000 | 6.138 | 162.92M | 5.895 | 169.63M | 65.818 | 10.72× | 11.16× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.103 | 0.258 | 2.49× |
| 1 | 5 | 0.357 | 1.278 | 3.58× |
| 1 | 10 | 0.507 | 2.152 | 4.25× |
| 10 | 1 | 0.049 | 0.198 | 4.06× |
| 10 | 5 | 0.225 | 1.253 | 5.57× |
| 10 | 10 | 0.476 | 2.159 | 4.54× |
| 100 | 1 | 0.050 | 0.205 | 4.07× |
| 100 | 5 | 0.242 | 1.292 | 5.34× |
| 100 | 10 | 0.485 | 2.235 | 4.61× |
| 1,000 | 1 | 0.058 | 0.254 | 4.40× |
| 1,000 | 5 | 0.256 | 1.604 | 6.26× |
| 1,000 | 10 | 0.511 | 2.837 | 5.55× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
