# mesh2sculptr
Convert OBJ files to `Data.csv` that can be imported to [SculptrVR](https://www.sculptrvr.com/) on Desktop.

This was inspired by and partially based on https://github.com/EX0l0N/ply-to-SculptrVR-csv -- which does the same thing for PLY point clouds.

## Binaries
[Windows](https://github.com/lebek/mesh2sculptr/releases/latest/download/mesh2sculptr.exe)

## Usage
```
$ mesh2sculptr.exe -h
mesh2sculptr 0.1.0
OBJ -> SculptrVR Converter

USAGE:
    mesh2sculptr.exe <file> [ARGS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <file>          Sets input OBJ file
    <resolution>    Sets the resolution of the voxel output (i.e. num voxels per axis) [default: 100]
    <fill>          Attempt to fill the inside of the mesh with voxels
    <objviz>        In addition to the CSV, output an OBJ representing the voxelization (useful for debugging)

```

In most cases you just want: 
```
$ mesh2sculptr.exe yourmesh.obj
```

If you want higher resolution:
```
$ mesh2sculptr.exe yourmesh.obj --resolution 200
```

## Importing the `Data.csv` to SculptrVR
From PLY-to-SculptrVR:

> It's a little secret, actually - _and it does only work for the PC version!_
> 
> You have to move the `Data.csv` file into a folder named `CSVs` at the top-level of the SculptrVR installation folder.  
> _Which is **not** your documents folder_.
> 
> Here's a piece of my SteamLibrary to help you figure out where:
> 
> ```
> SteamLibrary
> └── steamapps
>     ├── common
>     │   └── sculptrvr
>     │       ├── Engine
>     │       ├── SculptrVR
>     │       │   ├── Binaries
>     │       │   ├── Content
>     │       │   ├── CSVs
>     │       │   │   └── Data.csv
>     │       │   └── Plugins
>     │       └── SculptrVR.exe
> ```
> 
> If you created that folder and put `Data.csv` there, you may press `ctrl-shift-L` anytime in SculptrVR to load the data.  
> Be sure that **the window has focus** (if you see a steam dialog in front, click into SculptrVRs window).

**After import you have to switch the layer to BLOCK rendering mode to see anything.**
