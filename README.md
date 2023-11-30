# mesh-compress

A method for serializing surface meshes that is more space-efficient.

## File Spec

```
UInt64: Number of Vertices
for each vertex:
    Float32: x, Float32: y, Float32: z
end for
UInt64: Number of Faces
for each face:
    8 bit face header
    if the first 2 bits are 00:
        UInt8: Vertex index of face index 0
    if the first 2 bits are 01:
        UInt16: Vertex index of face index 0
    if the first 2 bits are 10:
        UInt32: Vertex index of face index 0

    if the second 2 bits are 00:
        Int8: Difference between face index 0 and face index 1
    if the second 2 bits are 01:
        Int16: Difference between face index 0 and face index 1
    if the first 2 bits are 10:
        Int32: Difference between face index 0 and face index 1
    if the first 2 bits are 11:
        UInt32: Vertex index of face index 1

    if the third 2 bits are 00:
        Int8: Difference between face index 0 and face index 2
    if the third 2 bits are 01:
        Int16: Difference between face index 0 and face index 2
    if the third 2 bits are 10:
        Int32: Difference between face index 0 and face index 2
    if the third 2 bits are 11:
        UInt32: Vertex index of face index 2
```

## Example:

2529686 faces, 1264847 vertices

As binary dump: 45.5 MB

As new format: 34.1 MB (25% decrease)
