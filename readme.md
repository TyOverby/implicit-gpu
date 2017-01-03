# Installation Instructions

## OS-X

1. `cargo build`

## Ubuntu
1. `sudo apt-get install ocl-icd-opencl-dev`
2. `cargo build`

## Windows

1. Install Visual Studio
2. Install the OpenCL driver for your graphics card
    * [Nvidia](https://developer.nvidia.com/cuda-downloads)
    * [AMD](http://developer.amd.com/tools-and-sdks/opencl-zone/amd-accelerated-parallel-processing-app-sdk/)
    * [Intel](https://software.intel.com/en-us/articles/opencl-drivers)
3. Find the `OpenCL.lib` file that should have been installed by the driver.
4. Set the `LIB` environment variable to the directory containing the `OpenCL.lib` file found in the previous step.
5. Restart your computer at least once (to appease the gods).
    * (It is currently unknown if praying helps.)
7. `cargo build`
