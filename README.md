# JUNGLE

<img src="https://github.com/antonKalinin/jungle/blob/master/assets/progress.gif?raw=true" width="764" />

Just wanted to have a collection of steps to create a platformer game using [rustlang](https://www.rust-lang.org/).
This game made with [bevy](https://github.com/bevyengine/bevy).

1. [Window size & scale options](https://github.com/antonKalinin/jungle/tree/1-window-options)
2. [Parallax background](https://github.com/antonKalinin/jungle/tree/2-parallax-background)
3. Asset animations
4. Loading map & objects
5. Collision detection
6. Extra interaction with objects & surfaces

## Some useful gamedev hints:

### Split animated GIF to sequence of PNGs:

```
convert run.gif run_%01d.png
```

### To join 8 images into 1 sprite via ImageMagic:

```
montage run_[0-7].png -geometry '1x1+0+0<' -tile 8x1 -background none run.png
```

More about command parameters here: http://www.imagemagick.org/Usage/montage/
