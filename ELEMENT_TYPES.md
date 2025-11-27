# Gmsh Element Types - All 140 Types

Based on GmshDefines.h and Gmsh documentation.

## Classification
- **Fixed size (128 types)**: Use `[usize; N]` - node count is certain
- **Variable size (12 types)**: Use `Vec<usize>` - node count is variable or uncertain

## All Element Types

| ID  | Name             | Nodes | Type     | Description |
|-----|------------------|-------|----------|-------------|
| 1   | Line2            | 2     | Fixed    | 2-node line |
| 2   | Triangle3        | 3     | Fixed    | 3-node triangle |
| 3   | Quadrangle4      | 4     | Fixed    | 4-node quadrangle |
| 4   | Tetrahedron4     | 4     | Fixed    | 4-node tetrahedron |
| 5   | Hexahedron8      | 8     | Fixed    | 8-node hexahedron |
| 6   | Prism6           | 6     | Fixed    | 6-node prism |
| 7   | Pyramid5         | 5     | Fixed    | 5-node pyramid |
| 8   | Line3            | 3     | Fixed    | 3-node line |
| 9   | Triangle6        | 6     | Fixed    | 6-node triangle |
| 10  | Quadrangle9      | 9     | Fixed    | 9-node quadrangle |
| 11  | Tetrahedron10    | 10    | Fixed    | 10-node tetrahedron |
| 12  | Hexahedron27     | 27    | Fixed    | 27-node hexahedron |
| 13  | Prism18          | 18    | Fixed    | 18-node prism |
| 14  | Pyramid14        | 14    | Fixed    | 14-node pyramid |
| 15  | Point            | 1     | Fixed    | 1-node point |
| 16  | Quadrangle8      | 8     | Fixed    | 8-node quadrangle |
| 17  | Hexahedron20     | 20    | Fixed    | 20-node hexahedron |
| 18  | Prism15          | 15    | Fixed    | 15-node prism |
| 19  | Pyramid13        | 13    | Fixed    | 13-node pyramid |
| 20  | Triangle9        | 9     | Fixed    | 9-node triangle |
| 21  | Triangle10       | 10    | Fixed    | 10-node triangle |
| 22  | Triangle12       | 12    | Fixed    | 12-node triangle |
| 23  | Triangle15       | 15    | Fixed    | 15-node triangle (complete) |
| 24  | Triangle15I      | 15    | Fixed    | 15-node triangle (incomplete) |
| 25  | Triangle21       | 21    | Fixed    | 21-node triangle |
| 26  | Line4            | 4     | Fixed    | 4-node line |
| 27  | Line5            | 5     | Fixed    | 5-node line |
| 28  | Line6            | 6     | Fixed    | 6-node line |
| 29  | Tetrahedron20    | 20    | Fixed    | 20-node tetrahedron |
| 30  | Tetrahedron35    | 35    | Fixed    | 35-node tetrahedron |
| 31  | Tetrahedron56    | 56    | Fixed    | 56-node tetrahedron |
| 32  | Tetrahedron22    | 22    | Fixed    | 22-node tetrahedron |
| 33  | Tetrahedron28    | 28    | Fixed    | 28-node tetrahedron |
| 34  | Polygon          | Var   | Variable | Variable-node polygon |
| 35  | Polyhedron       | Var   | Variable | Variable-node polyhedron |
| 36  | Quadrangle16     | 16    | Fixed    | 16-node quadrangle |
| 37  | Quadrangle25     | 25    | Fixed    | 25-node quadrangle |
| 38  | Quadrangle36     | 36    | Fixed    | 36-node quadrangle |
| 39  | Quadrangle12     | 12    | Fixed    | 12-node quadrangle |
| 40  | Quadrangle16I    | 16    | Fixed    | 16-node quadrangle (incomplete) |
| 41  | Quadrangle20     | 20    | Fixed    | 20-node quadrangle |
| 42  | Triangle28       | 28    | Fixed    | 28-node triangle |
| 43  | Triangle36       | 36    | Fixed    | 36-node triangle |
| 44  | Triangle45       | 45    | Fixed    | 45-node triangle |
| 45  | Triangle55       | 55    | Fixed    | 55-node triangle |
| 46  | Triangle66       | 66    | Fixed    | 66-node triangle |
| 47  | Quadrangle49     | 49    | Fixed    | 49-node quadrangle |
| 48  | Quadrangle64     | 64    | Fixed    | 64-node quadrangle |
| 49  | Quadrangle81     | 81    | Fixed    | 81-node quadrangle |
| 50  | Quadrangle100    | 100   | Fixed    | 100-node quadrangle |
| 51  | Quadrangle121    | 121   | Fixed    | 121-node quadrangle |
| 52  | Triangle18       | 18    | Fixed    | 18-node triangle |
| 53  | Triangle21I      | 21    | Fixed    | 21-node triangle (incomplete) |
| 54  | Triangle24       | 24    | Fixed    | 24-node triangle |
| 55  | Triangle27       | 27    | Fixed    | 27-node triangle |
| 56  | Triangle30       | 30    | Fixed    | 30-node triangle |
| 57  | Quadrangle24     | 24    | Fixed    | 24-node quadrangle |
| 58  | Quadrangle28     | 28    | Fixed    | 28-node quadrangle |
| 59  | Quadrangle32     | 32    | Fixed    | 32-node quadrangle |
| 60  | Quadrangle36I    | 36    | Fixed    | 36-node quadrangle (incomplete) |
| 61  | Quadrangle40     | 40    | Fixed    | 40-node quadrangle |
| 62  | Line7            | 7     | Fixed    | 7-node line |
| 63  | Line8            | 8     | Fixed    | 8-node line |
| 64  | Line9            | 9     | Fixed    | 9-node line |
| 65  | Line10           | 10    | Fixed    | 10-node line |
| 66  | Line11           | 11    | Fixed    | 11-node line |
| 67  | LineB            | Var   | Variable | Line with Bezier/Bubble (uncertain) |
| 68  | TriangleB        | Var   | Variable | Triangle with Bezier/Bubble (uncertain) |
| 69  | PolygonB         | Var   | Variable | Polygon with Bezier/Bubble |
| 70  | LineC            | Var   | Variable | Composite line (uncertain) |
| 71  | Tetrahedron84    | 84    | Fixed    | 84-node tetrahedron (complete) |
| 72  | Tetrahedron120   | 120   | Fixed    | 120-node tetrahedron (complete) |
| 73  | Tetrahedron165   | 165   | Fixed    | 165-node tetrahedron (complete) |
| 74  | Tetrahedron220   | 220   | Fixed    | 220-node tetrahedron (complete) |
| 75  | Tetrahedron286   | 286   | Fixed    | 286-node tetrahedron (complete) |
| 79  | Tetrahedron34    | 34    | Fixed    | 34-node tetrahedron (incomplete) |
| 80  | Tetrahedron40    | 40    | Fixed    | 40-node tetrahedron (incomplete) |
| 81  | Tetrahedron46    | 46    | Fixed    | 46-node tetrahedron (incomplete) |
| 82  | Tetrahedron52    | 52    | Fixed    | 52-node tetrahedron (incomplete) |
| 83  | Tetrahedron58    | 58    | Fixed    | 58-node tetrahedron (incomplete) |
| 84  | Line1            | 1     | Fixed    | 1-node line |
| 85  | Triangle1        | 1     | Fixed    | 1-node triangle |
| 86  | Quadrangle1      | 1     | Fixed    | 1-node quadrangle |
| 87  | Tetrahedron1     | 1     | Fixed    | 1-node tetrahedron |
| 88  | Hexahedron1      | 1     | Fixed    | 1-node hexahedron |
| 89  | Prism1           | 1     | Fixed    | 1-node prism |
| 90  | Prism40          | 40    | Fixed    | 40-node prism (incomplete) |
| 91  | Prism75          | 75    | Fixed    | 75-node prism |
| 92  | Hexahedron64     | 64    | Fixed    | 64-node hexahedron (complete) |
| 93  | Hexahedron125    | 125   | Fixed    | 125-node hexahedron (complete) |
| 94  | Hexahedron216    | 216   | Fixed    | 216-node hexahedron (complete) |
| 95  | Hexahedron343    | 343   | Fixed    | 343-node hexahedron (complete) |
| 96  | Hexahedron512    | 512   | Fixed    | 512-node hexahedron (complete) |
| 97  | Hexahedron729    | 729   | Fixed    | 729-node hexahedron (complete) |
| 98  | Hexahedron1000   | 1000  | Fixed    | 1000-node hexahedron (complete) |
| 99  | Hexahedron32     | 32    | Fixed    | 32-node hexahedron (incomplete) |
| 100 | Hexahedron44     | 44    | Fixed    | 44-node hexahedron (incomplete) |
| 101 | Hexahedron56     | 56    | Fixed    | 56-node hexahedron (incomplete) |
| 102 | Hexahedron68     | 68    | Fixed    | 68-node hexahedron (incomplete) |
| 103 | Hexahedron80     | 80    | Fixed    | 80-node hexahedron (incomplete) |
| 104 | Hexahedron92     | 92    | Fixed    | 92-node hexahedron (incomplete) |
| 105 | Hexahedron104    | 104   | Fixed    | 104-node hexahedron (incomplete) |
| 106 | Prism126         | 126   | Fixed    | 126-node prism (complete) |
| 107 | Prism196         | 196   | Fixed    | 196-node prism (complete) |
| 108 | Prism288         | 288   | Fixed    | 288-node prism (complete) |
| 109 | Prism405         | 405   | Fixed    | 405-node prism (complete) |
| 110 | Prism550         | 550   | Fixed    | 550-node prism (complete) |
| 111 | Prism24          | 24    | Fixed    | 24-node prism (incomplete) |
| 112 | Prism33          | 33    | Fixed    | 33-node prism (incomplete) |
| 113 | Prism42          | 42    | Fixed    | 42-node prism (incomplete) |
| 114 | Prism51          | 51    | Fixed    | 51-node prism (incomplete) |
| 115 | Prism60          | 60    | Fixed    | 60-node prism (incomplete) |
| 116 | Prism69          | 69    | Fixed    | 69-node prism (incomplete) |
| 117 | Prism78          | 78    | Fixed    | 78-node prism (incomplete) |
| 118 | Pyramid30        | 30    | Fixed    | 30-node pyramid (complete) |
| 119 | Pyramid55        | 55    | Fixed    | 55-node pyramid (complete) |
| 120 | Pyramid91        | 91    | Fixed    | 91-node pyramid (complete) |
| 121 | Pyramid140       | 140   | Fixed    | 140-node pyramid (complete) |
| 122 | Pyramid204       | 204   | Fixed    | 204-node pyramid (complete) |
| 123 | Pyramid285       | 285   | Fixed    | 285-node pyramid (complete) |
| 124 | Pyramid385       | 385   | Fixed    | 385-node pyramid (complete) |
| 125 | Pyramid21        | 21    | Fixed    | 21-node pyramid (incomplete) |
| 126 | Pyramid29        | 29    | Fixed    | 29-node pyramid (incomplete) |
| 127 | Pyramid37        | 37    | Fixed    | 37-node pyramid (incomplete) |
| 128 | Pyramid45        | 45    | Fixed    | 45-node pyramid (incomplete) |
| 129 | Pyramid53        | 53    | Fixed    | 53-node pyramid (incomplete) |
| 130 | Pyramid61        | 61    | Fixed    | 61-node pyramid (incomplete) |
| 131 | Pyramid69        | 69    | Fixed    | 69-node pyramid (incomplete) |
| 132 | Pyramid1         | 1     | Fixed    | 1-node pyramid |
| 133 | PointSub         | Var   | Variable | Point sub-element (uncertain) |
| 134 | LineSub          | Var   | Variable | Line sub-element (uncertain) |
| 135 | TriangleSub      | Var   | Variable | Triangle sub-element (uncertain) |
| 136 | TetrahedronSub   | Var   | Variable | Tetrahedron sub-element (uncertain) |
| 137 | Tetrahedron16    | 16    | Fixed    | 16-node tetrahedron |
| 138 | TriangleMini     | Var   | Variable | Triangle mini element (uncertain) |
| 139 | TetrahedronMini  | Var   | Variable | Tetrahedron mini element (uncertain) |
| 140 | TriHedron4       | 4     | Fixed    | 4-node tri-hedron (one quad + two tri faces) |

## Summary

- **Fixed size**: 128 types (IDs 1-33, 36-66, 71-75, 79-132, 137, 140)
- **Variable size**: 12 types (IDs 34-35, 67-70, 133-136, 138-139)
- **Missing IDs**: 76-78 (not defined in GmshDefines.h)

## Notes

Variable-size elements require runtime checking of node count. These include:
- Polygons and polyhedra (truly variable)
- Bezier/Bubble elements (node count uncertain)
- Composite elements (node count uncertain)
- Sub-elements (node count uncertain)
- Mini elements (node count uncertain - typically 3+1 or 4+1 but not confirmed)
