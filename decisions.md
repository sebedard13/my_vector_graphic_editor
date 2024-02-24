# Decisions

This document is to explain the decisions made for the project. It is also a good place to find the reasons behind the choices made in the project. It is a living document and will be updated as the project evolves.

## Scene coordinate range

The range of coordinates for the canvas is -1.0 to 1.0. This use the most of the precision of the float value by using the sign bit. The aspect ratio of the canvas is not saved in the coordinates, but it is saved externally.

I agree that a canvas with a rectangular aspect ratio will lose some precision for the largest side. But, I believed the it is more important that the range of the coordinates is the constant for every aspect ratio. Calculations can be easly bounded to the range of -1.0 to 1.0 without asking for the aspect ratio. It is easier to work with. The precision lost for the largest side is not a problem because the precision is still large enought for most art piece.

## Coordinate precision

A 32-bit float value has precision of 2^-23 for a value between -1.0 and 1.0. This is enough for the precision of the coordinates. The biggest art piece in pixels could be defined by this formula if we want a point to still be in 1/3 pixel, :

```
point_per_pixel = 3
largest_size * 2^23 < (1.0- (-1.0)) * point_per_pixel
largest_size ≈ 52 000 000
```

After with a dpi of 300, we can find the size of canvas in inch/meter.

```
largest_size / 300 ≈ 173333.3 inch or 4402.6 meter
```

It should be large enough for any art piece.

The other choice is using a 64-bit float value. This is not necessary and will take more memory and will be slower to compute.
The same formula with a 64-bit float value:

```
largest_size * 2^-52 < (1.0- (-1.0)) * point_per_pixel
largest_size ≈ 2.70e16
largest_size / 300 ≈ 2.28e12 m or 15.29 au
```
