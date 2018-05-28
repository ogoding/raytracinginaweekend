# Rust implementation of [Peter Shirley's ebooks on building a Ray-Tracing Engine in one weekend (and the following weekends)](https://www.amazon.com/s/ref=nb_sb_noss_2?url=search-alias%3Daps&field-keywords=peter+shirley)

## Disclaimer
* The base code and design was created by [Peter Shirley](https://twitter.com/Peter_shirley) [Github page](https://github.com/petershirley).
* My changes and code currently consist of some modifications to better fit Rust idioms and my own preferences and any other changes required as part of writing it in Rust instead of C++

## Description
A Rust implementation of Peter Shirley's excellent [Raytracing book series](https://github.com/petershirley/raytracinginoneweekend) as a side project to better learn Rust and Ray-Tracing Engines.

Current Progress:
![current_progress.ppm](https://github.com/ogoding/raytracinginaweekend/raw/master/images/current_progress.ppm "Current Progress")


## TODO:
* Generate output for other image formats (jpg, png, etc) and fix readme link
* Increase image size
* Add some support for concurrent rays - via Rayon or Faster libraries
* Restructure hit detection and scattering to better support parallel ray tracing
* General refactorings and renaming for clarity
