# Decisions

This document is to explain the decisions made for the project. It is also a good place to find the reasons behind the choices made in the project. It is a living document and will be updated as the project evolves.

## Scene coordinate range

The range of coordinates for the canvas is -1.0 to 1.0. This use the most of the precision of the float value by using the sign bit. The aspect ratio of the canvas is not saved in the coordinates, but it is saved externally.

I agree that a canvas with a rectangular aspect ratio will lose some precision for the largest side. But, I believed the it is more important that the range of the coordinates is the constant for every aspect ratio. Calculations can be easly bounded to the range of -1.0 to 1.0 without asking for the aspect ratio. It is easier to work with. The precision lost for the largest side is not a problem because the precision is still large enought for most art piece.

## Coordinate precision

A 32-bit float value has precision of 2^-23 for a value between -1.0 and 1.0. This is enough for the precision of the coordinates. We add a point per pixel to truncate some of the values between some calculation. The biggest art piece in pixels could be defined by this formula :

```
point_per_pixel = 50
largest_size * 2^-23 < (1.0- (-1.0)) / point_per_pixel
largest_size ≈ 560 000 px
```
After with a dpi of 300, we can find the size of canvas in inch/meter.
```
largest_size / 300 ≈ 1864 inchs or 47 meters
```

It should be large enough for any art piece.

The other choice is using a 64-bit float value. This is not necessary and will take more memory and will be slower to compute.
The same formula with a 64-bit float value:

```
largest_size * 2^-52 < (1.0- (-1.0)) / point_per_pixel
largest_size ≈ 300e12 px
largest_size / 300 ≈ 25e9 m or 0.169 au
```

This is mostly a choice base on performance and precision. The precision is enough for most art piece and the performance is better than using a 64-bit float value. But some art piece could be too large for this precision and we will want 64-bit float value. Will need some benchmark to see if the performance is really a problem. TODO